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
            (TaskStatus::InProgress, "EN COURS", Theme::ACCENT_CYAN),
            (TaskStatus::Review, "EN REVUE / QA", Theme::WARNING),
            (TaskStatus::Done, "TERMINÉ", Theme::SUCCESS),
        ];

        ui.columns(columns.len(), |cols| {
            for (i, (status, title, accent_color)) in columns.iter().enumerate() {
                let col_ui = &mut cols[i];
                let col_tasks: Vec<&Task> = tasks.iter().filter(|t| t.status == *status).collect();

                egui::Frame::none()
                    .fill(Theme::PANEL_BG)
                    .stroke(Stroke::new(1.0, Theme::BORDER))
                    .rounding(Rounding::same(10.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(col_ui, |ui| {
                        ui.horizontal(|ui| {
                            let (rect, _) = ui.allocate_exact_size(Vec2::new(4.0, 16.0), egui::Sense::hover());
                            ui.painter().rect_filled(rect, Rounding::same(2.0), *accent_color);
                            ui.strong(egui::RichText::new(*title).color(Theme::TEXT_TITLE).size(13.0));
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                egui::Frame::none()
                                    .fill(Theme::CARD_BG)
                                    .rounding(Rounding::same(12.0))
                                    .inner_margin(egui::Margin::symmetric(8.0, 2.0))
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new(col_tasks.len().to_string()).color(Theme::TEXT_MUTED).size(11.0));
                                    });
                            });
                        });
                        ui.add_space(10.0);

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

                                    let card_border = if is_selected {
                                        Stroke::new(1.5, Theme::ACCENT_PRIMARY)
                                    } else if task.is_critical {
                                        Stroke::new(1.0, Theme::CRITICAL_PATH.gamma_multiply(0.6))
                                    } else {
                                        Stroke::new(1.0, Theme::BORDER)
                                    };

                                    let card_response = egui::Frame::none()
                                        .fill(card_bg)
                                        .stroke(card_border)
                                        .rounding(Rounding::same(8.0))
                                        .inner_margin(egui::Margin::same(12.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                if task.is_critical {
                                                    egui::Frame::none()
                                                        .fill(Theme::CRITICAL_PATH.gamma_multiply(0.2))
                                                        .rounding(Rounding::same(4.0))
                                                        .inner_margin(egui::Margin::symmetric(6.0, 2.0))
                                                        .show(ui, |ui| {
                                                            ui.label(egui::RichText::new("● CRITIQUE").color(Theme::CRITICAL_PATH).size(10.0));
                                                        });
                                                }
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    let (prio_color, prio_text) = match task.priority {
                                                        TaskPriority::Urgent => (Theme::CRITICAL_PATH, "🔥 Urgent"),
                                                        TaskPriority::High => (Theme::WARNING, "⚡ Haute"),
                                                        TaskPriority::Normal => (Theme::TEXT_MUTED, "Normale"),
                                                        TaskPriority::Low => (Theme::TEXT_SUBTLE, "Basse"),
                                                    };
                                                    ui.label(egui::RichText::new(prio_text).color(prio_color).size(10.5));
                                                });
                                            });

                                            ui.add_space(6.0);
                                            ui.label(egui::RichText::new(&task.title).color(Theme::TEXT_TITLE).strong().size(13.0));

                                            if !task.description.is_empty() {
                                                ui.add_space(4.0);
                                                ui.label(egui::RichText::new(&task.description).color(Theme::TEXT_MUTED).size(11.0));
                                            }

                                            ui.add_space(8.0);
                                            ui.horizontal(|ui| {
                                                egui::Frame::none()
                                                    .fill(Theme::PANEL_BG)
                                                    .rounding(Rounding::same(4.0))
                                                    .inner_margin(egui::Margin::symmetric(6.0, 3.0))
                                                    .show(ui, |ui| {
                                                        ui.label(egui::RichText::new(format!("⏱ {}h", task.duration_hours)).color(Theme::TEXT_PRIMARY).size(11.0));
                                                    });

                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    egui::ComboBox::from_id_salt(format!("status_combo_{}", task.id))
                                                        .selected_text(egui::RichText::new(task.status.label_fr()).size(11.0))
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
                                    ui.add_space(8.0);
                                }
                            });
                    });
            }
        });
    }
}
