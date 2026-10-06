use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TaskStatus {
    #[default]
    Todo,
    InProgress,
    Review,
    Done,
    Blocked,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "TODO",
            TaskStatus::InProgress => "IN_PROGRESS",
            TaskStatus::Review => "REVIEW",
            TaskStatus::Done => "DONE",
            TaskStatus::Blocked => "BLOCKED",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "IN_PROGRESS" | "INPROGRESS" => TaskStatus::InProgress,
            "REVIEW" => TaskStatus::Review,
            "DONE" => TaskStatus::Done,
            "BLOCKED" => TaskStatus::Blocked,
            _ => TaskStatus::Todo,
        }
    }

    pub fn label_fr(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "À faire",
            TaskStatus::InProgress => "En cours",
            TaskStatus::Review => "En revue / QA",
            TaskStatus::Done => "Terminé",
            TaskStatus::Blocked => "Bloqué",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TaskPriority {
    Low,
    #[default]
    Normal,
    High,
    Urgent,
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskPriority::Low => "LOW",
            TaskPriority::Normal => "NORMAL",
            TaskPriority::High => "HIGH",
            TaskPriority::Urgent => "URGENT",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "LOW" => TaskPriority::Low,
            "HIGH" => TaskPriority::High,
            "URGENT" => TaskPriority::Urgent,
            _ => TaskPriority::Normal,
        }
    }

    pub fn label_fr(&self) -> &'static str {
        match self {
            TaskPriority::Low => "Basse",
            TaskPriority::Normal => "Normale",
            TaskPriority::High => "Haute",
            TaskPriority::Urgent => "Urgente",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DependencyType {
    #[default]
    FinishToStart, // FS (Prédécesseur doit finir avant que Successeur commence)
    StartToStart,  // SS
}

impl DependencyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DependencyType::FinishToStart => "FS",
            DependencyType::StartToStart => "SS",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "SS" => DependencyType::StartToStart,
            _ => DependencyType::FinishToStart,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMember {
    pub id: String,
    pub workspace_id: String,
    pub full_name: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub workspace_id: String,
    pub key_prefix: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub start_date: NaiveDate,
    pub estimated_end_date: Option<NaiveDate>,
    pub is_archived: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub assignee_id: Option<String>,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub duration_hours: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub early_start: Option<NaiveDate>,
    pub early_finish: Option<NaiveDate>,
    pub late_start: Option<NaiveDate>,
    pub late_finish: Option<NaiveDate>,
    pub is_critical: bool,
    pub position_order: i32,
    pub is_deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependency {
    pub id: String,
    pub project_id: String,
    pub predecessor_task_id: String,
    pub successor_task_id: String,
    pub dependency_type: DependencyType,
    pub lag_days: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleResult {
    pub project_id: String,
    pub project_start_date: NaiveDate,
    pub estimated_end_date: NaiveDate,
    pub total_duration_days: i64,
    pub critical_path_task_ids: Vec<String>,
    pub task_schedules: Vec<TaskScheduleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskScheduleInfo {
    pub task_id: String,
    pub early_start: NaiveDate,
    pub early_finish: NaiveDate,
    pub late_start: NaiveDate,
    pub late_finish: NaiveDate,
    pub total_slack_days: i64,
    pub is_critical: bool,
}
