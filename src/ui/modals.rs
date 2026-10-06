use chrono::NaiveDate;
use egui::{Context, Window};
use crate::domain::models::{Project, Task, TaskPriority, TaskStatus, UserMember};
use crate::ui::theme::Theme;

#[derive(Default)]
pub struct ProjectModalState {
    pub is_open: bool,
    pub is_editing: bool,
    pub project_id: String,
    pub key_prefix: String,
    pub name: String,
    pub description: String,
    pub start_date_str: String,
}

#[derive(Default)]
pub struct TaskModalState {
    pub is_open: bool,
    pub is_editing: bool,
    pub task_id: String,
    pub title: String,
    pub description: String,
    pub duration_hours: i32,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub assignee_id: Option<String>,
    pub selected_dependency_pred_id: String,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct DependencyModalState {
    pub is_open: bool,
    pub predecessor_id: String,
    pub successor_id: String,
}

#[derive(Default)]
pub struct ErrorAlertState {
    pub is_open: bool,
    pub title: String,
    pub message: String,
}

pub struct Modals;

impl Modals {
    pub fn render_project_modal(
        ctx: &Context,
        state: &mut ProjectModalState,
        on_save: &mut Option<Project>,
    ) {
        if !state.is_open {
            return;
        }

        let title = if state.is_editing { "Modifier le Projet" } else { "Nouveau Projet" };
        Window::new(title)
            .collapsible(false)
            .resizable(false)
            .fixed_size([420.0, 300.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label("Clé unique (2-6 lettres) :");
                    ui.text_edit_singleline(&mut state.key_prefix);
                });
                ui.horizontal(|ui| {
                    ui.label("Nom du projet :");
                    ui.text_edit_singleline(&mut state.name);
                });
                ui.horizontal(|ui| {
                    ui.label("Date de début (YYYY-MM-DD) :");
                    ui.text_edit_singleline(&mut state.start_date_str);
                });
                ui.add_space(4.0);
                ui.label("Description :");
                ui.text_edit_multiline(&mut state.description);

                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.button("Annuler").clicked() {
                        state.is_open = false;
                    }
                    if ui.colored_label(Theme::ACCENT_PRIMARY, "Sauvegarder").clicked() || ui.button("💾 Enregistrer").clicked() {
                        if !state.name.trim().is_empty() && !state.key_prefix.trim().is_empty() {
                            let start_date = NaiveDate::parse_from_str(&state.start_date_str, "%Y-%m-%d")
                                .unwrap_or_else(|_| chrono::Local::now().date_naive());
                            
                            let proj = Project {
                                id: if state.is_editing { state.project_id.clone() } else { uuid::Uuid::new_v4().to_string() },
                                workspace_id: "ws-default".to_string(),
                                key_prefix: state.key_prefix.to_uppercase(),
                                name: state.name.clone(),
                                description: state.description.clone(),
                                status: "ACTIVE".to_string(),
                                start_date,
                                estimated_end_date: None,
                                is_archived: false,
                                created_at: chrono::Utc::now().to_rfc3339(),
                                updated_at: chrono::Utc::now().to_rfc3339(),
                            };
                            *on_save = Some(proj);
                            state.is_open = false;
                        }
                    }
                });
            });
    }

    pub fn render_task_modal(
        ctx: &Context,
        state: &mut TaskModalState,
        project_id: &str,
        project_start_date: NaiveDate,
        members: &[UserMember],
        other_tasks: &[Task],
        on_save: &mut Option<Task>,
        on_add_dependency: &mut Option<(String, String)>,
        on_delete_task: &mut Option<String>,
    ) {
        if !state.is_open {
            return;
        }

        let modal_title = if state.is_editing { "Édition de la Tâche" } else { "Nouvelle Tâche" };
        Window::new(modal_title)
            .collapsible(false)
            .resizable(false)
            .fixed_size([460.0, 380.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label("Titre :");
                    ui.text_edit_singleline(&mut state.title);
                });

                ui.horizontal(|ui| {
                    ui.label("Durée estimée (heures) :");
                    ui.add(egui::DragValue::new(&mut state.duration_hours).range(1..=500));
                    ui.label(format!("(~{} jour(s))", (state.duration_hours + 7) / 8));
                });

                ui.horizontal(|ui| {
                    ui.label("Priorité :");
                    egui::ComboBox::from_id_salt("task_modal_prio")
                        .selected_text(state.priority.label_fr())
                        .show_ui(ui, |ui| {
                            for p in &[TaskPriority::Low, TaskPriority::Normal, TaskPriority::High, TaskPriority::Urgent] {
                                ui.selectable_value(&mut state.priority, *p, p.label_fr());
                            }
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Statut :");
                    egui::ComboBox::from_id_salt("task_modal_status")
                        .selected_text(state.status.label_fr())
                        .show_ui(ui, |ui| {
                            for s in &[TaskStatus::Todo, TaskStatus::InProgress, TaskStatus::Review, TaskStatus::Done] {
                                ui.selectable_value(&mut state.status, *s, s.label_fr());
                            }
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Intervenant assigné :");
                    let current_name = members
                        .iter()
                        .find(|m| Some(&m.id) == state.assignee_id.as_ref())
                        .map(|m| m.full_name.as_str())
                        .unwrap_or("Non assigné");
                    
                    egui::ComboBox::from_id_salt("task_modal_assignee")
                        .selected_text(current_name)
                        .show_ui(ui, |ui| {
                            if ui.selectable_label(state.assignee_id.is_none(), "Non assigné").clicked() {
                                state.assignee_id = None;
                            }
                            for m in members {
                                if ui.selectable_label(state.assignee_id.as_deref() == Some(&m.id), &m.full_name).clicked() {
                                    state.assignee_id = Some(m.id.clone());
                                }
                            }
                        });
                });

                if !other_tasks.is_empty() {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Dépendance (Doit suivre) :");
                        let current_pred_title = other_tasks
                            .iter()
                            .find(|t| t.id == state.selected_dependency_pred_id)
                            .map(|t| t.title.as_str())
                            .unwrap_or("Aucune dépendance");
                        
                        egui::ComboBox::from_id_salt("task_modal_dep_pred")
                            .selected_text(current_pred_title)
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(state.selected_dependency_pred_id.is_empty(), "Aucune").clicked() {
                                    state.selected_dependency_pred_id.clear();
                                }
                                for ot in other_tasks {
                                    if ot.id != state.task_id {
                                        if ui.selectable_label(state.selected_dependency_pred_id == ot.id, &ot.title).clicked() {
                                            state.selected_dependency_pred_id = ot.id.clone();
                                        }
                                    }
                                }
                            });
                    });
                }

                ui.add_space(4.0);
                ui.label("Description :");
                ui.text_edit_multiline(&mut state.description);

                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    if ui.button("Annuler").clicked() {
                        state.is_open = false;
                    }
                    if state.is_editing {
                        if ui.colored_label(Theme::CRITICAL_PATH, "🗑 Supprimer").clicked() || ui.button("🗑 Supprimer").clicked() {
                            *on_delete_task = Some(state.task_id.clone());
                            state.is_open = false;
                        }
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("💾 Enregistrer").clicked() {
                            if !state.title.trim().is_empty() {
                                let new_task_id = if state.is_editing { state.task_id.clone() } else { uuid::Uuid::new_v4().to_string() };
                                let task = Task {
                                    id: new_task_id.clone(),
                                    project_id: project_id.to_string(),
                                    assignee_id: state.assignee_id.clone(),
                                    title: state.title.clone(),
                                    description: state.description.clone(),
                                    status: state.status,
                                    priority: state.priority,
                                    duration_hours: state.duration_hours,
                                    start_date: project_start_date,
                                    end_date: project_start_date,
                                    early_start: None,
                                    early_finish: None,
                                    late_start: None,
                                    late_finish: None,
                                    is_critical: false,
                                    position_order: 0,
                                    is_deleted: false,
                                    created_at: chrono::Utc::now().to_rfc3339(),
                                    updated_at: chrono::Utc::now().to_rfc3339(),
                                };
                                *on_save = Some(task);
                                if !state.selected_dependency_pred_id.is_empty() {
                                    *on_add_dependency = Some((state.selected_dependency_pred_id.clone(), new_task_id));
                                }
                                state.is_open = false;
                            }
                        }
                    });
                });
            });
    }

    pub fn render_error_modal(ctx: &Context, state: &mut ErrorAlertState) {
        if !state.is_open {
            return;
        }

        Window::new(format!("⚠️ {}", state.title))
            .collapsible(false)
            .resizable(false)
            .fixed_size([380.0, 180.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.colored_label(Theme::CRITICAL_PATH, "Opération non autorisée par le moteur de graphe :");
                ui.add_space(4.0);
                ui.label(&state.message);
                ui.add_space(16.0);
                ui.vertical_centered(|ui| {
                    if ui.button("Compris").clicked() {
                        state.is_open = false;
                    }
                });
            });
    }
}
