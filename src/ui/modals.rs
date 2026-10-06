use chrono::NaiveDate;
use egui::{Color32, Context, Rounding, Stroke, Window};
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

        let title = if state.is_editing { "✏️ Modifier le Projet" } else { "📁 Nouveau Projet" };
        Window::new(title)
            .collapsible(false)
            .resizable(false)
            .fixed_size([440.0, 310.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Clé unique :").color(Theme::TEXT_MUTED).size(12.0));
                    ui.add(egui::TextEdit::singleline(&mut state.key_prefix).hint_text("ex: ALPHA"));
                });
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Nom du projet :").color(Theme::TEXT_MUTED).size(12.0));
                    ui.add(egui::TextEdit::singleline(&mut state.name).hint_text("ex: Refonte Système"));
                });
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Date de début :").color(Theme::TEXT_MUTED).size(12.0));
                    ui.add(egui::TextEdit::singleline(&mut state.start_date_str).hint_text("YYYY-MM-DD"));
                });
                ui.add_space(4.0);
                ui.label(egui::RichText::new("Description & Objectifs :").color(Theme::TEXT_MUTED).size(12.0));
                ui.add(egui::TextEdit::multiline(&mut state.description).desired_rows(3).desired_width(f32::INFINITY));

                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    if ui.button(egui::RichText::new("Annuler").color(Theme::TEXT_MUTED)).clicked() {
                        state.is_open = false;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let save_btn = egui::Button::new(egui::RichText::new("💾 Enregistrer").color(Color32::WHITE).strong())
                            .fill(Theme::ACCENT_PRIMARY);

                        if ui.add(save_btn).clicked() {
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

        let modal_title = if state.is_editing { "✏️ Édition de la Tâche" } else { "➕ Nouvelle Tâche" };
        Window::new(modal_title)
            .collapsible(false)
            .resizable(false)
            .fixed_size([480.0, 400.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Titre de la tâche :").color(Theme::TEXT_MUTED).size(12.0));
                    ui.add(egui::TextEdit::singleline(&mut state.title).desired_width(f32::INFINITY));
                });

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Durée estimée :").color(Theme::TEXT_MUTED).size(12.0));
                    ui.add(egui::DragValue::new(&mut state.duration_hours).range(1..=500).suffix(" heures"));
                    ui.label(egui::RichText::new(format!("(~{} jour(s) ouvrés)", (state.duration_hours + 7) / 8)).color(Theme::TEXT_SUBTLE).size(11.0));
                });

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Priorité :").color(Theme::TEXT_MUTED).size(12.0));
                    egui::ComboBox::from_id_salt("task_modal_prio")
                        .selected_text(state.priority.label_fr())
                        .show_ui(ui, |ui| {
                            for p in &[TaskPriority::Low, TaskPriority::Normal, TaskPriority::High, TaskPriority::Urgent] {
                                ui.selectable_value(&mut state.priority, *p, p.label_fr());
                            }
                        });

                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Statut :").color(Theme::TEXT_MUTED).size(12.0));
                    egui::ComboBox::from_id_salt("task_modal_status")
                        .selected_text(state.status.label_fr())
                        .show_ui(ui, |ui| {
                            for s in &[TaskStatus::Todo, TaskStatus::InProgress, TaskStatus::Review, TaskStatus::Done] {
                                ui.selectable_value(&mut state.status, *s, s.label_fr());
                            }
                        });
                });

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Affecté à :").color(Theme::TEXT_MUTED).size(12.0));
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
                    ui.add_space(4.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("🔗 Dépendance amont :").color(Theme::ACCENT_CYAN).size(12.0));
                        let current_pred_title = other_tasks
                            .iter()
                            .find(|t| t.id == state.selected_dependency_pred_id)
                            .map(|t| t.title.as_str())
                            .unwrap_or("Aucune (Démarrage initial)");
                        
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
                ui.label(egui::RichText::new("Description détaillée :").color(Theme::TEXT_MUTED).size(12.0));
                ui.add(egui::TextEdit::multiline(&mut state.description).desired_rows(3).desired_width(f32::INFINITY));

                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    if ui.button(egui::RichText::new("Annuler").color(Theme::TEXT_MUTED)).clicked() {
                        state.is_open = false;
                    }
                    if state.is_editing {
                        let delete_btn = egui::Button::new(egui::RichText::new("🗑 Supprimer").color(Theme::CRITICAL_PATH))
                            .stroke(Stroke::new(1.0, Theme::CRITICAL_PATH.gamma_multiply(0.4)));
                        if ui.add(delete_btn).clicked() {
                            *on_delete_task = Some(state.task_id.clone());
                            state.is_open = false;
                        }
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let save_btn = egui::Button::new(egui::RichText::new("💾 Enregistrer").color(Color32::WHITE).strong())
                            .fill(Theme::ACCENT_PRIMARY);

                        if ui.add(save_btn).clicked() {
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
            .fixed_size([400.0, 190.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(8.0);
                egui::Frame::none()
                    .fill(Theme::CRITICAL_PATH.gamma_multiply(0.12))
                    .stroke(Stroke::new(1.0, Theme::CRITICAL_PATH.gamma_multiply(0.5)))
                    .rounding(Rounding::same(6.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.colored_label(Theme::CRITICAL_PATH, "Blocage du moteur de graphe :");
                        ui.add_space(4.0);
                        ui.label(egui::RichText::new(&state.message).color(Theme::TEXT_TITLE).size(12.0));
                    });
                
                ui.add_space(14.0);
                ui.vertical_centered(|ui| {
                    let ok_btn = egui::Button::new(egui::RichText::new("Compris").color(Color32::WHITE).strong())
                        .fill(Theme::CARD_BG)
                        .stroke(Stroke::new(1.0, Theme::BORDER));
                    if ui.add(ok_btn).clicked() {
                        state.is_open = false;
                    }
                });
            });
    }
}
