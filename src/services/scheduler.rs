use std::sync::Arc;
use parking_lot::Mutex;
use crate::domain::errors::AppError;
use crate::domain::graph::{CpmEngine, CycleDetector};
use crate::domain::models::{ScheduleResult, TaskDependency};
use crate::storage::Database;

pub struct SchedulerService {
    db: Arc<Mutex<Database>>,
}

#[allow(dead_code)]
impl SchedulerService {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db }
    }

    pub fn recalculate_project_schedule(&self, project_id: &str) -> Result<ScheduleResult, AppError> {
        let db = self.db.lock();
        
        let projects = db.get_projects(true)?;
        let project = projects
            .into_iter()
            .find(|p| p.id == project_id)
            .ok_or_else(|| AppError::NotFound(format!("Projet {}", project_id)))?;

        let tasks = db.get_tasks_by_project(project_id)?;
        let dependencies = db.get_dependencies_by_project(project_id)?;

        let schedule_result = CpmEngine::compute_schedule(
            project_id,
            project.start_date,
            &tasks,
            &dependencies,
        )?;

        db.update_project_estimated_end_date(project_id, schedule_result.estimated_end_date)?;

        for ts in &schedule_result.task_schedules {
            db.update_task_cpm_schedule(
                &ts.task_id,
                ts.early_start,
                ts.early_finish,
                ts.late_start,
                ts.late_finish,
                ts.is_critical,
            )?;
        }

        Ok(schedule_result)
    }

    pub fn add_dependency(
        &self,
        project_id: &str,
        predecessor_id: &str,
        successor_id: &str,
    ) -> Result<ScheduleResult, AppError> {
        let db = self.db.lock();
        let tasks = db.get_tasks_by_project(project_id)?;
        let dependencies = db.get_dependencies_by_project(project_id)?;

        let node_ids: Vec<String> = tasks.iter().map(|t| t.id.clone()).collect();
        let edges: Vec<(String, String)> = dependencies
            .iter()
            .map(|d| (d.predecessor_task_id.clone(), d.successor_task_id.clone()))
            .collect();

        CycleDetector::would_create_cycle(&node_ids, &edges, predecessor_id, successor_id)?;

        let new_dep = TaskDependency {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: project_id.to_string(),
            predecessor_task_id: predecessor_task_id_clean(predecessor_id),
            successor_task_id: predecessor_task_id_clean(successor_id),
            dependency_type: crate::domain::models::DependencyType::FinishToStart,
            lag_days: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        db.insert_dependency(&new_dep)?;
        drop(db);

        self.recalculate_project_schedule(project_id)
    }

    pub fn remove_dependency(
        &self,
        project_id: &str,
        predecessor_id: &str,
        successor_id: &str,
    ) -> Result<ScheduleResult, AppError> {
        let db = self.db.lock();
        db.delete_dependency_between(predecessor_id, successor_id)?;
        drop(db);
        self.recalculate_project_schedule(project_id)
    }
}

fn predecessor_task_id_clean(s: &str) -> String {
    s.trim().to_string()
}
