use egui::{Rounding, Stroke, Vec2};
use crate::domain::models::{Task, TaskPriority, TaskStatus};
use crate::ui::theme::Theme;

pub struct KanbanView;

impl KanbanView {
    pub fn render(
        ui: &mut egui::Ui,
        tasks: &[Task],
        selected_task_id: &mut Option<String>,
        on_status_change: &mut Option<(String, TaskStatus)>,
        on_open_task_modal: &mut bool,
    ) {
        let columns = [
            (TaskStatus::Todo, "À FAIRE", Theme::TEXT_MUTED),
            (TaskStatus::InProgress, "EN COURS", Theme::ACCENT_PRIMARY),
            (TaskStatus::Review, "EN REVUE / QA", Theme::WARNING),
            (TaskStatus::Done, "TERMINÉ", Theme::SUCCESS),
        ];

        ui.columns(columns.len(), |cols| {
            for (i, (status, title, accent_color)) in columns.iter().enumerate() {
                let col_ui = &mut cols[i];
                let col_tasks: Vec<&Task> = tasks.iter().filter(|t| t.status == *status).collect();

                egui::Frame::none()
                    .fill(Theme::PANEL_BG)
                    .rounding(Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(10.0))
                    .show(col_ui, |ui| {
                        ui.horizontal(|ui| {
                            let (rect, _) = ui.allocate_exact_size(Vec2::new(8.0, 16.0), egui::Sense::hover());
                            ui.painter().rect_filled(rect, Rounding::same(2.0), *accent_color);
                            ui.strong(*title);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(format!("({})", col_tasks.len()));
                            });
                        });
                        ui.add_space(8.0);

                        egui::ScrollArea::vertical()
                            .id_salt(format!("kanban_col_{}", status.as_str()))
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                for task in col_tasks {
                                    let is_selected = selected_task_id.as_deref() == Some(&task.id);
                                    let card_bg = if is_selected {
                                        Theme::CARD_HOVER
                                    } else {
                                        Theme::CARD_BG
                                    };

                                    let card_response = egui::Frame::none()
                                        .fill(card_bg)
                                        .stroke(Stroke::new(
                                            if is_selected { 2.0 } else { 1.0 },
                                            if is_selected { Theme::ACCENT_PRIMARY } else { Theme::BORDER },
                                        ))
                                        .rounding(Rounding::same(6.0))
                                        .inner_margin(egui::Margin::same(10.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                if task.is_critical {
                                                    ui.colored_label(Theme::CRITICAL_PATH, "● CRITIQUE");
                                                }
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    let prio_color = match task.priority {
                                                        TaskPriority::Urgent => Theme::CRITICAL_PATH,
                                                        TaskPriority::High => Theme::WARNING,
                                                        _ => Theme::TEXT_MUTED,
                                                    };
                                                    ui.colored_label(prio_color, task.priority.label_fr());
                                                });
                                            });

                                            ui.add_space(4.0);
                                            ui.strong(&task.title);

                                            if !task.description.is_empty() {
                                                ui.add_space(2.0);
                                                ui.label(egui::RichText::new(&task.description).color(Theme::TEXT_MUTED).size(11.0));
                                            }

                                            ui.add_space(6.0);
                                            ui.horizontal(|ui| {
                                                ui.label(format!("⏱ {}h", task.duration_hours));
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    egui::ComboBox::from_id_salt(format!("status_combo_{}", task.id))
                                                        .selected_text(task.status.label_fr())
                                                        .width(90.0)
                                                        .show_ui(ui, |ui| {
                                                            for s in &[TaskStatus::Todo, TaskStatus::InProgress, TaskStatus::Review, TaskStatus::Done] {
                                                                if ui.selectable_label(task.status == *s, s.label_fr()).clicked() {
                                                                    *on_status_change = Some((task.id.clone(), *s));
                                                                }
                                                            }
                                                        });
                                                });
                                            });
                                        });

                                    if card_response.response.clicked() {
                                        *selected_task_id = Some(task.id.clone());
                                    }
                                    if card_response.response.double_clicked() {
                                        *selected_task_id = Some(task.id.clone());
                                        *on_open_task_modal = true;
                                    }
                                    ui.add_space(6.0);
                                }
                            });
                    });
            }
        });
    }
}
