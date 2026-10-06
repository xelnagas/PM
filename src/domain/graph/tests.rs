use super::*;
use chrono::NaiveDate;
use pretty_assertions::assert_eq;
use proptest::prelude::*;
use crate::domain::errors::GraphError;
use crate::domain::models::{DependencyType, Task, TaskPriority, TaskStatus, TaskDependency};

fn create_mock_task(id: &str, duration_hours: i32, start_date: NaiveDate) -> Task {
    Task {
        id: id.to_string(),
        project_id: "prj-1".to_string(),
        assignee_id: None,
        title: format!("Task {}", id),
        description: "".to_string(),
        status: TaskStatus::Todo,
        priority: TaskPriority::Normal,
        duration_hours,
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
    }
}

fn create_mock_dependency(pred: &str, succ: &str) -> TaskDependency {
    TaskDependency {
        id: format!("{}_{}", pred, succ),
        project_id: "prj-1".to_string(),
        predecessor_task_id: pred.to_string(),
        successor_task_id: succ.to_string(),
        dependency_type: DependencyType::FinishToStart,
        lag_days: 0,
        created_at: "2026-10-06T00:00:00Z".to_string(),
    }
}

#[test]
fn test_cycle_detector_direct_self_loop() {
    let nodes = vec!["A".to_string(), "B".to_string()];
    let edges = vec![];
    let result = CycleDetector::would_create_cycle(&nodes, &edges, "A", "A");
    assert_eq!(result, Err(GraphError::SelfDependency("A".to_string())));
}

#[test]
fn test_cycle_detector_direct_two_node_cycle() {
    let nodes = vec!["A".to_string(), "B".to_string()];
    let edges = vec![("A".to_string(), "B".to_string())];
    
    // Tenter d'ajouter B -> A doit être rejeté
    let result = CycleDetector::would_create_cycle(&nodes, &edges, "B", "A");
    assert_eq!(
        result,
        Err(GraphError::CycleDetected {
            predecessor_id: "B".to_string(),
            successor_id: "A".to_string(),
        })
    );
}

#[test]
fn test_cycle_detector_multi_node_transitive_cycle() {
    // A -> B -> C -> D
    let nodes = vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()];
    let edges = vec![
        ("A".to_string(), "B".to_string()),
        ("B".to_string(), "C".to_string()),
        ("C".to_string(), "D".to_string()),
    ];

    // Tenter d'ajouter D -> A doit être rejeté
    let result = CycleDetector::would_create_cycle(&nodes, &edges, "D", "A");
    assert_eq!(
        result,
        Err(GraphError::CycleDetected {
            predecessor_id: "D".to_string(),
            successor_id: "A".to_string(),
        })
    );
}

#[test]
fn test_cycle_detector_valid_diamond_dag() {
    // A -> B, A -> C, B -> D, C -> D (Diamant valide)
    let nodes = vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()];
    let edges = vec![
        ("A".to_string(), "B".to_string()),
        ("A".to_string(), "C".to_string()),
        ("B".to_string(), "D".to_string()),
    ];

    let result = CycleDetector::would_create_cycle(&nodes, &edges, "C", "D");
    assert_eq!(result, Ok(()));
}

#[test]
fn test_cpm_sequential_tasks_schedule() {
    let start_date = NaiveDate::from_ymd_opt(2026, 10, 1).unwrap();
    // T1 (8h = 1j) -> T2 (16h = 2j) -> T3 (24h = 3j)
    let tasks = vec![
        create_mock_task("T1", 8, start_date),
        create_mock_task("T2", 16, start_date),
        create_mock_task("T3", 24, start_date),
    ];
    let dependencies = vec![
        create_mock_dependency("T1", "T2"),
        create_mock_dependency("T2", "T3"),
    ];

    let schedule = CpmEngine::compute_schedule("prj-1", start_date, &tasks, &dependencies)
        .expect("Schedule computation should succeed");

    assert_eq!(schedule.total_duration_days, 6); // 1 + 2 + 3 = 6 jours
    assert_eq!(schedule.estimated_end_date, NaiveDate::from_ymd_opt(2026, 10, 7).unwrap());
    assert_eq!(schedule.critical_path_task_ids, vec!["T1", "T2", "T3"]);
}

#[test]
fn test_cpm_parallel_branches_and_critical_path() {
    let start_date = NaiveDate::from_ymd_opt(2026, 10, 1).unwrap();
    // T1 (8h = 1j)
    // Branche A : T2 (8h = 1j)
    // Branche B : T3 (24h = 3j) [CHEMIN CRITIQUE]
    // T4 (8h = 1j) dépend de T2 et T3
    let tasks = vec![
        create_mock_task("T1", 8, start_date),
        create_mock_task("T2", 8, start_date),
        create_mock_task("T3", 24, start_date),
        create_mock_task("T4", 8, start_date),
    ];
    let dependencies = vec![
        create_mock_dependency("T1", "T2"),
        create_mock_dependency("T1", "T3"),
        create_mock_dependency("T2", "T4"),
        create_mock_dependency("T3", "T4"),
    ];

    let schedule = CpmEngine::compute_schedule("prj-1", start_date, &tasks, &dependencies)
        .expect("Schedule computation should succeed");

    // Total = T1 (1) + T3 (3) + T4 (1) = 5 jours
    assert_eq!(schedule.total_duration_days, 5);
    assert_eq!(schedule.estimated_end_date, NaiveDate::from_ymd_opt(2026, 10, 6).unwrap());
    
    // Le chemin critique doit être T1 -> T3 -> T4
    assert!(schedule.critical_path_task_ids.contains(&"T1".to_string()));
    assert!(schedule.critical_path_task_ids.contains(&"T3".to_string()));
    assert!(schedule.critical_path_task_ids.contains(&"T4".to_string()));
    assert!(!schedule.critical_path_task_ids.contains(&"T2".to_string())); // T2 a du slack
}

proptest! {
    #[test]
    fn prop_dag_without_backward_edges_never_fails_cycle_check(
        num_nodes in 3usize..30,
        edge_pairs in proptest::collection::vec((0usize..30, 0usize..30), 1..50)
    ) {
        let node_ids: Vec<String> = (0..num_nodes).map(|i| format!("node_{}", i)).collect();
        let mut edges = Vec::new();

        for (u, v) in edge_pairs {
            if u < v && u < num_nodes && v < num_nodes {
                let from = format!("node_{}", u);
                let to = format!("node_{}", v);
                if !edges.contains(&(from.clone(), to.clone())) {
                    edges.push((from, to));
                }
            }
        }

        let topo = CycleDetector::topological_sort(&node_ids, &edges);
        prop_assert!(topo.is_ok(), "Tout DAG où les indices sont croissants doit être valide");
    }
}
