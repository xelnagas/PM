use std::collections::HashMap;
use chrono::{Duration, NaiveDate};
use crate::domain::errors::GraphError;
use crate::domain::graph::cycle_detector::CycleDetector;
use crate::domain::models::{ScheduleResult, Task, TaskDependency, TaskScheduleInfo};

pub struct CpmEngine;

impl CpmEngine {
    /// Calcule l'ordonnancement complet CPM du projet
    pub fn compute_schedule(
        project_id: &str,
        project_start_date: NaiveDate,
        tasks: &[Task],
        dependencies: &[TaskDependency],
    ) -> Result<ScheduleResult, GraphError> {
        if tasks.is_empty() {
            return Ok(ScheduleResult {
                project_id: project_id.to_string(),
                project_start_date,
                estimated_end_date: project_start_date,
                total_duration_days: 0,
                critical_path_task_ids: Vec::new(),
                task_schedules: Vec::new(),
            });
        }

        let node_ids: Vec<String> = tasks.iter().map(|t| t.id.clone()).collect();
        let edges: Vec<(String, String)> = dependencies
            .iter()
            .map(|d| (d.predecessor_task_id.clone(), d.successor_task_id.clone()))
            .collect();

        // 1. Tri topologique pour valider l'absence de cycle
        let topo_order = CycleDetector::topological_sort(&node_ids, &edges)?;

        // Map de durées en jours (minimum 1 jour)
        let mut durations_days: HashMap<&str, i64> = HashMap::new();
        let mut task_map: HashMap<&str, &Task> = HashMap::new();
        for t in tasks {
            let days = ((t.duration_hours + 7) / 8).max(1) as i64;
            durations_days.insert(t.id.as_str(), days);
            task_map.insert(t.id.as_str(), t);
        }

        // Listes d'adjacence directe et inverse
        let mut predecessors: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut successors: HashMap<&str, Vec<&str>> = HashMap::new();
        for id in &node_ids {
            predecessors.entry(id.as_str()).or_default();
            successors.entry(id.as_str()).or_default();
        }
        for (pred, succ) in &edges {
            predecessors.entry(succ.as_str()).or_default().push(pred.as_str());
            successors.entry(pred.as_str()).or_default().push(succ.as_str());
        }

        // 2. Forward Pass (Calcul des dates au plus tôt ES & EF)
        // Offset en jours depuis project_start_date
        let mut early_start_offsets: HashMap<&str, i64> = HashMap::new();
        let mut early_finish_offsets: HashMap<&str, i64> = HashMap::new();

        for task_id in &topo_order {
            let id = task_id.as_str();
            let duration = durations_days[id];

            let preds = &predecessors[id];
            let es_offset = if preds.is_empty() {
                // Si la tâche a une date de début fixée plus tard que le projet, on la respecte
                let custom_offset = (task_map[id].start_date - project_start_date).num_days().max(0);
                custom_offset
            } else {
                let max_pred_ef = preds
                    .iter()
                    .map(|&p| early_finish_offsets[p])
                    .max()
                    .unwrap_or(0);
                max_pred_ef
            };

            let ef_offset = es_offset + duration;
            early_start_offsets.insert(id, es_offset);
            early_finish_offsets.insert(id, ef_offset);
        }

        // Date de fin estimée du projet = max(EF de toutes les tâches)
        let max_project_duration_days = early_finish_offsets
            .values()
            .copied()
            .max()
            .unwrap_or(0);
        let project_estimated_end_date = project_start_date + Duration::days(max_project_duration_days);

        // 3. Backward Pass (Calcul des dates au plus tard LS & LF)
        let mut late_start_offsets: HashMap<&str, i64> = HashMap::new();
        let mut late_finish_offsets: HashMap<&str, i64> = HashMap::new();

        for task_id in topo_order.iter().rev() {
            let id = task_id.as_str();
            let duration = durations_days[id];

            let succs = &successors[id];
            let lf_offset = if succs.is_empty() {
                max_project_duration_days
            } else {
                let min_succ_ls = succs
                    .iter()
                    .map(|&s| late_start_offsets[s])
                    .min()
                    .unwrap_or(max_project_duration_days);
                min_succ_ls
            };

            let ls_offset = lf_offset - duration;
            late_start_offsets.insert(id, ls_offset);
            late_finish_offsets.insert(id, lf_offset);
        }

        // 4. Calcul des Marges (Slack) & Identification du Chemin Critique
        let mut task_schedules = Vec::new();
        let mut critical_path_task_ids = Vec::new();

        for t in tasks {
            let id = t.id.as_str();
            let es = project_start_date + Duration::days(early_start_offsets[id]);
            let ef = project_start_date + Duration::days(early_finish_offsets[id]);
            let ls = project_start_date + Duration::days(late_start_offsets[id]);
            let lf = project_start_date + Duration::days(late_finish_offsets[id]);

            let slack_days = (ls - es).num_days();
            let is_critical = slack_days == 0;

            if is_critical {
                critical_path_task_ids.push(t.id.clone());
            }

            task_schedules.push(TaskScheduleInfo {
                task_id: t.id.clone(),
                early_start: es,
                early_finish: ef,
                late_start: ls,
                late_finish: lf,
                total_slack_days: slack_days,
                is_critical,
            });
        }

        Ok(ScheduleResult {
            project_id: project_id.to_string(),
            project_start_date,
            estimated_end_date: project_estimated_end_date,
            total_duration_days: max_project_duration_days,
            critical_path_task_ids,
            task_schedules,
        })
    }
}
