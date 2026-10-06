use chrono::NaiveDate;
use rusqlite::{params, Connection, Result};
use std::path::Path;
use crate::domain::models::*;

pub struct Database {
    conn: Connection,
}

#[allow(dead_code)]
impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;

            CREATE TABLE IF NOT EXISTS workspaces (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS user_members (
                id TEXT PRIMARY KEY NOT NULL,
                workspace_id TEXT NOT NULL,
                full_name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                role TEXT NOT NULL DEFAULT 'MEMBER',
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY NOT NULL,
                workspace_id TEXT NOT NULL,
                key_prefix TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                status TEXT NOT NULL DEFAULT 'ACTIVE',
                start_date TEXT NOT NULL,
                estimated_end_date TEXT,
                is_archived INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY NOT NULL,
                project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                assignee_id TEXT REFERENCES user_members(id) ON DELETE SET NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                status TEXT NOT NULL DEFAULT 'TODO',
                priority TEXT NOT NULL DEFAULT 'NORMAL',
                duration_hours INTEGER NOT NULL DEFAULT 8,
                start_date TEXT NOT NULL,
                end_date TEXT NOT NULL,
                early_start TEXT,
                early_finish TEXT,
                late_start TEXT,
                late_finish TEXT,
                is_critical INTEGER NOT NULL DEFAULT 0,
                position_order INTEGER NOT NULL DEFAULT 0,
                is_deleted INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS task_dependencies (
                id TEXT PRIMARY KEY NOT NULL,
                project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                predecessor_task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
                successor_task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
                dependency_type TEXT NOT NULL DEFAULT 'FS',
                lag_days INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                CONSTRAINT uq_dep UNIQUE (predecessor_task_id, successor_task_id)
            );

            CREATE TABLE IF NOT EXISTS project_tab_state (
                id TEXT PRIMARY KEY NOT NULL,
                project_id TEXT NOT NULL UNIQUE REFERENCES projects(id) ON DELETE CASCADE,
                tab_order INTEGER NOT NULL DEFAULT 0,
                is_active INTEGER NOT NULL DEFAULT 0
            );

            INSERT OR IGNORE INTO workspaces (id, name) VALUES ('ws-default', 'Mon Organisation');
            INSERT OR IGNORE INTO user_members (id, workspace_id, full_name, email, role) 
                VALUES ('user-1', 'ws-default', 'Julien Simand', 'julien@pm.local', 'ADMIN');
            INSERT OR IGNORE INTO user_members (id, workspace_id, full_name, email, role) 
                VALUES ('user-2', 'ws-default', 'Alice Développeuse', 'alice@pm.local', 'MEMBER');
            INSERT OR IGNORE INTO user_members (id, workspace_id, full_name, email, role) 
                VALUES ('user-3', 'ws-default', 'Bob Testeur', 'bob@pm.local', 'MEMBER');
            "
        )?;
        Ok(())
    }

    pub fn get_projects(&self, include_archived: bool) -> Result<Vec<Project>> {
        let sql = if include_archived {
            "SELECT id, workspace_id, key_prefix, name, description, status, start_date, estimated_end_date, is_archived, created_at, updated_at FROM projects ORDER BY created_at DESC"
        } else {
            "SELECT id, workspace_id, key_prefix, name, description, status, start_date, estimated_end_date, is_archived, created_at, updated_at FROM projects WHERE is_archived = 0 ORDER BY created_at DESC"
        };
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let start_date_str: String = row.get(6)?;
            let est_date_str: Option<String> = row.get(7)?;
            Ok(Project {
                id: row.get(0)?,
                workspace_id: row.get(1)?,
                key_prefix: row.get(2)?,
                name: row.get(3)?,
                description: row.get(4)?,
                status: row.get(5)?,
                start_date: NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d").unwrap_or_else(|_| chrono::Local::now().date_naive()),
                estimated_end_date: est_date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                is_archived: row.get::<_, i32>(8)? != 0,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?;
        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn insert_project(&self, p: &Project) -> Result<()> {
        self.conn.execute(
            "INSERT INTO projects (id, workspace_id, key_prefix, name, description, status, start_date, estimated_end_date, is_archived, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                p.id,
                p.workspace_id,
                p.key_prefix,
                p.name,
                p.description,
                p.status,
                p.start_date.format("%Y-%m-%d").to_string(),
                p.estimated_end_date.map(|d| d.format("%Y-%m-%d").to_string()),
                if p.is_archived { 1 } else { 0 },
                p.created_at,
                p.updated_at,
            ],
        )?;
        Ok(())
    }

    pub fn update_project(&self, p: &Project) -> Result<()> {
        self.conn.execute(
            "UPDATE projects SET name = ?1, description = ?2, status = ?3, start_date = ?4, estimated_end_date = ?5, is_archived = ?6, updated_at = datetime('now')
             WHERE id = ?7",
            params![
                p.name,
                p.description,
                p.status,
                p.start_date.format("%Y-%m-%d").to_string(),
                p.estimated_end_date.map(|d| d.format("%Y-%m-%d").to_string()),
                if p.is_archived { 1 } else { 0 },
                p.id,
            ],
        )?;
        Ok(())
    }

    pub fn update_project_estimated_end_date(&self, project_id: &str, end_date: NaiveDate) -> Result<()> {
        self.conn.execute(
            "UPDATE projects SET estimated_end_date = ?1, updated_at = datetime('now') WHERE id = ?2",
            params![end_date.format("%Y-%m-%d").to_string(), project_id],
        )?;
        Ok(())
    }

    pub fn delete_project(&self, project_id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM projects WHERE id = ?1", params![project_id])?;
        Ok(())
    }

    pub fn get_tasks_by_project(&self, project_id: &str) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, assignee_id, title, description, status, priority, duration_hours, start_date, end_date, early_start, early_finish, late_start, late_finish, is_critical, position_order, is_deleted, created_at, updated_at
             FROM tasks WHERE project_id = ?1 AND is_deleted = 0 ORDER BY position_order ASC, created_at ASC",
        )?;
        let rows = stmt.query_map(params![project_id], |row| {
            let start_str: String = row.get(8)?;
            let end_str: String = row.get(9)?;
            let es_str: Option<String> = row.get(10)?;
            let ef_str: Option<String> = row.get(11)?;
            let ls_str: Option<String> = row.get(12)?;
            let lf_str: Option<String> = row.get(13)?;
            let status_str: String = row.get(5)?;
            let prio_str: String = row.get(6)?;

            Ok(Task {
                id: row.get(0)?,
                project_id: row.get(1)?,
                assignee_id: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                status: TaskStatus::from_str(&status_str),
                priority: TaskPriority::from_str(&prio_str),
                duration_hours: row.get(7)?,
                start_date: NaiveDate::parse_from_str(&start_str, "%Y-%m-%d").unwrap_or_else(|_| chrono::Local::now().date_naive()),
                end_date: NaiveDate::parse_from_str(&end_str, "%Y-%m-%d").unwrap_or_else(|_| chrono::Local::now().date_naive()),
                early_start: es_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                early_finish: ef_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                late_start: ls_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                late_finish: lf_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                is_critical: row.get::<_, i32>(14)? != 0,
                position_order: row.get(15)?,
                is_deleted: row.get::<_, i32>(16)? != 0,
                created_at: row.get(17)?,
                updated_at: row.get(18)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn insert_task(&self, t: &Task) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tasks (id, project_id, assignee_id, title, description, status, priority, duration_hours, start_date, end_date, position_order, is_deleted, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                t.id,
                t.project_id,
                t.assignee_id,
                t.title,
                t.description,
                t.status.as_str(),
                t.priority.as_str(),
                t.duration_hours,
                t.start_date.format("%Y-%m-%d").to_string(),
                t.end_date.format("%Y-%m-%d").to_string(),
                t.position_order,
                if t.is_deleted { 1 } else { 0 },
                t.created_at,
                t.updated_at,
            ],
        )?;
        Ok(())
    }

    pub fn update_task(&self, t: &Task) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET assignee_id = ?1, title = ?2, description = ?3, status = ?4, priority = ?5, duration_hours = ?6, start_date = ?7, end_date = ?8, position_order = ?9, updated_at = datetime('now')
             WHERE id = ?10",
            params![
                t.assignee_id,
                t.title,
                t.description,
                t.status.as_str(),
                t.priority.as_str(),
                t.duration_hours,
                t.start_date.format("%Y-%m-%d").to_string(),
                t.end_date.format("%Y-%m-%d").to_string(),
                t.position_order,
                t.id,
            ],
        )?;
        Ok(())
    }

    pub fn update_task_cpm_schedule(&self, t_id: &str, es: NaiveDate, ef: NaiveDate, ls: NaiveDate, lf: NaiveDate, is_critical: bool) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET early_start = ?1, early_finish = ?2, late_start = ?3, late_finish = ?4, is_critical = ?5, updated_at = datetime('now')
             WHERE id = ?6",
            params![
                es.format("%Y-%m-%d").to_string(),
                ef.format("%Y-%m-%d").to_string(),
                ls.format("%Y-%m-%d").to_string(),
                lf.format("%Y-%m-%d").to_string(),
                if is_critical { 1 } else { 0 },
                t_id,
            ],
        )?;
        Ok(())
    }

    pub fn soft_delete_task(&self, task_id: &str) -> Result<()> {
        self.conn.execute("UPDATE tasks SET is_deleted = 1, updated_at = datetime('now') WHERE id = ?1", params![task_id])?;
        Ok(())
    }

    pub fn get_dependencies_by_project(&self, project_id: &str) -> Result<Vec<TaskDependency>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, predecessor_task_id, successor_task_id, dependency_type, lag_days, created_at
             FROM task_dependencies WHERE project_id = ?1",
        )?;
        let rows = stmt.query_map(params![project_id], |row| {
            let dep_str: String = row.get(4)?;
            Ok(TaskDependency {
                id: row.get(0)?,
                project_id: row.get(1)?,
                predecessor_task_id: row.get(2)?,
                successor_task_id: row.get(3)?,
                dependency_type: DependencyType::from_str(&dep_str),
                lag_days: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn insert_dependency(&self, dep: &TaskDependency) -> Result<()> {
        self.conn.execute(
            "INSERT INTO task_dependencies (id, project_id, predecessor_task_id, successor_task_id, dependency_type, lag_days, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                dep.id,
                dep.project_id,
                dep.predecessor_task_id,
                dep.successor_task_id,
                dep.dependency_type.as_str(),
                dep.lag_days,
                dep.created_at,
            ],
        )?;
        Ok(())
    }

    pub fn delete_dependency(&self, dep_id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM task_dependencies WHERE id = ?1", params![dep_id])?;
        Ok(())
    }

    pub fn delete_dependency_between(&self, pred_id: &str, succ_id: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM task_dependencies WHERE predecessor_task_id = ?1 AND successor_task_id = ?2",
            params![pred_id, succ_id],
        )?;
        Ok(())
    }

    pub fn get_members(&self) -> Result<Vec<UserMember>> {
        let mut stmt = self.conn.prepare("SELECT id, workspace_id, full_name, email, role FROM user_members")?;
        let rows = stmt.query_map([], |row| {
            Ok(UserMember {
                id: row.get(0)?,
                workspace_id: row.get(1)?,
                full_name: row.get(2)?,
                email: row.get(3)?,
                role: row.get(4)?,
            })
        })?;
        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn insert_member(&self, m: &UserMember) -> Result<()> {
        self.conn.execute(
            "INSERT INTO user_members (id, workspace_id, full_name, email, role) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![m.id, m.workspace_id, m.full_name, m.email, m.role],
        )?;
        Ok(())
    }

    pub fn delete_member(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM user_members WHERE id = ?1", params![id])?;
        Ok(())
    }
}
