use super::*;
use chrono::NaiveDate;
use pretty_assertions::assert_eq;
use crate::domain::models::*;

#[test]
fn test_database_crud_flow_for_projects_and_tasks() {
    let db = Database::open_in_memory().expect("In-memory DB creation failed");

    // 1. Création d'un projet
    let start_date = NaiveDate::from_ymd_opt(2026, 10, 1).unwrap();
    let project = Project {
        id: "p-1".to_string(),
        workspace_id: "ws-default".to_string(),
        key_prefix: "ALPHA".to_string(),
        name: "Projet Alpha".to_string(),
        description: "Description test".to_string(),
        status: "ACTIVE".to_string(),
        start_date,
        estimated_end_date: None,
        is_archived: false,
        created_at: "2026-10-06T00:00:00Z".to_string(),
        updated_at: "2026-10-06T00:00:00Z".to_string(),
    };
    db.insert_project(&project).expect("Insert project failed");

    let projects = db.get_projects(false).expect("Get projects failed");
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "Projet Alpha");

    // 2. Création de deux tâches
    let t1 = Task {
        id: "t-1".to_string(),
        project_id: "p-1".to_string(),
        assignee_id: None,
        title: "Tâche 1".to_string(),
        description: "".to_string(),
        status: TaskStatus::Todo,
        priority: TaskPriority::High,
        duration_hours: 16,
        start_date,
        end_date: start_date,
        early_start: None,
        early_finish: None,
        late_start: None,
        late_finish: None,
        is_critical: false,
        position_order: 0,
        is_deleted: false,
        created_at: "2026-10-06T00:00:00Z".to_string(),
        updated_at: "2026-10-06T00:00:00Z".to_string(),
    };
    let t2 = Task {
        id: "t-2".to_string(),
        project_id: "p-1".to_string(),
        assignee_id: None,
        title: "Tâche 2".to_string(),
        description: "".to_string(),
        status: TaskStatus::Todo,
        priority: TaskPriority::Normal,
        duration_hours: 8,
        start_date,
        end_date: start_date,
        early_start: None,
        early_finish: None,
        late_start: None,
        late_finish: None,
        is_critical: false,
        position_order: 1,
        is_deleted: false,
        created_at: "2026-10-06T00:00:00Z".to_string(),
        updated_at: "2026-10-06T00:00:00Z".to_string(),
    };
    db.insert_task(&t1).expect("Insert task 1 failed");
    db.insert_task(&t2).expect("Insert task 2 failed");

    let tasks = db.get_tasks_by_project("p-1").expect("Get tasks failed");
    assert_eq!(tasks.len(), 2);

    // 3. Liaison de dépendance t-1 -> t-2
    let dep = TaskDependency {
        id: "dep-1".to_string(),
        project_id: "p-1".to_string(),
        predecessor_task_id: "t-1".to_string(),
        successor_task_id: "t-2".to_string(),
        dependency_type: DependencyType::FinishToStart,
        lag_days: 0,
        created_at: "2026-10-06T00:00:00Z".to_string(),
    };
    db.insert_dependency(&dep).expect("Insert dep failed");

    let deps = db.get_dependencies_by_project("p-1").expect("Get deps failed");
    assert_eq!(deps.len(), 1);
    assert_eq!(deps[0].predecessor_task_id, "t-1");
    assert_eq!(deps[0].successor_task_id, "t-2");

    // 4. Suppression en cascade du projet
    db.delete_project("p-1").expect("Delete project failed");
    assert_eq!(db.get_projects(false).unwrap().len(), 0);
    assert_eq!(db.get_tasks_by_project("p-1").unwrap().len(), 0);
    assert_eq!(db.get_dependencies_by_project("p-1").unwrap().len(), 0);
}
