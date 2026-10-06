use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    #[error("Cycle de dépendance détecté entre la tâche '{predecessor_id}' et la tâche '{successor_id}'")]
    CycleDetected {
        predecessor_id: String,
        successor_id: String,
    },

    #[error("La tâche référencée '{0}' n'existe pas dans le graphe")]
    TaskNotFound(String),

    #[error("Une tâche ne peut pas dépendre d'elle-même : '{0}'")]
    SelfDependency(String),

    #[error("Plage de dates invalide : date de début ({start}) postérieure à la date de fin ({end})")]
    InvalidDateRange {
        start: String,
        end: String,
    },

    #[error("Cette dépendance existe déjà dans le projet")]
    DuplicateDependency,
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Erreur de graphe : {0}")]
    Graph(#[from] GraphError),

    #[error("Erreur de base de données : {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Erreur de validation : {0}")]
    Validation(String),

    #[error("Ressource non trouvée : {0}")]
    NotFound(String),
}
