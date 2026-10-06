use std::collections::{HashMap, VecDeque};
use crate::domain::errors::GraphError;

pub struct CycleDetector;

impl CycleDetector {
    /// Vérifie si l'ajout d'une arête (predecessor -> successor) créerait un cycle dans le graphe
    /// Utilise l'algorithme de Kahn / Tri Topologique.
    pub fn would_create_cycle(
        all_node_ids: &[String],
        existing_edges: &[(String, String)],
        new_predecessor: &str,
        new_successor: &str,
    ) -> Result<(), GraphError> {
        if new_predecessor == new_successor {
            return Err(GraphError::SelfDependency(new_predecessor.to_string()));
        }

        let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut in_degree: HashMap<&str, usize> = HashMap::new();

        for id in all_node_ids {
            adj.entry(id.as_str()).or_default();
            in_degree.insert(id.as_str(), 0);
        }

        if !in_degree.contains_key(new_predecessor) {
            return Err(GraphError::TaskNotFound(new_predecessor.to_string()));
        }
        if !in_degree.contains_key(new_successor) {
            return Err(GraphError::TaskNotFound(new_successor.to_string()));
        }

        for (pred, succ) in existing_edges {
            if pred == new_predecessor && succ == new_successor {
                return Err(GraphError::DuplicateDependency);
            }
            if let Some(neighbors) = adj.get_mut(pred.as_str()) {
                neighbors.push(succ.as_str());
            }
            if let Some(deg) = in_degree.get_mut(succ.as_str()) {
                *deg += 1;
            }
        }

        if let Some(neighbors) = adj.get_mut(new_predecessor) {
            neighbors.push(new_successor);
        }
        if let Some(deg) = in_degree.get_mut(new_successor) {
            *deg += 1;
        }

        let mut queue: VecDeque<&str> = VecDeque::new();
        for (&id, &deg) in &in_degree {
            if deg == 0 {
                queue.push_back(id);
            }
        }

        let mut visited_count = 0;
        while let Some(u) = queue.pop_front() {
            visited_count += 1;
            if let Some(neighbors) = adj.get(u) {
                for &v in neighbors {
                    if let Some(deg) = in_degree.get_mut(v) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(v);
                        }
                    }
                }
            }
        }

        if visited_count == all_node_ids.len() {
            Ok(())
        } else {
            Err(GraphError::CycleDetected {
                predecessor_id: new_predecessor.to_string(),
                successor_id: new_successor.to_string(),
            })
        }
    }

    pub fn topological_sort(
        all_node_ids: &[String],
        edges: &[(String, String)],
    ) -> Result<Vec<String>, GraphError> {
        let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut in_degree: HashMap<&str, usize> = HashMap::new();

        for id in all_node_ids {
            adj.entry(id.as_str()).or_default();
            in_degree.insert(id.as_str(), 0);
        }

        for (pred, succ) in edges {
            if let Some(neighbors) = adj.get_mut(pred.as_str()) {
                neighbors.push(succ.as_str());
            }
            if let Some(deg) = in_degree.get_mut(succ.as_str()) {
                *deg += 1;
            }
        }

        let mut queue: VecDeque<&str> = VecDeque::new();
        for (&id, &deg) in &in_degree {
            if deg == 0 {
                queue.push_back(id);
            }
        }

        let mut order = Vec::with_capacity(all_node_ids.len());
        while let Some(u) = queue.pop_front() {
            order.push(u.to_string());
            if let Some(neighbors) = adj.get(u) {
                for &v in neighbors {
                    if let Some(deg) = in_degree.get_mut(v) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(v);
                        }
                    }
                }
            }
        }

        if order.len() == all_node_ids.len() {
            Ok(order)
        } else {
            Err(GraphError::CycleDetected {
                predecessor_id: "unknown".to_string(),
                successor_id: "unknown".to_string(),
            })
        }
    }
}
