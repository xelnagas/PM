use chrono::NaiveDate;
use egui::{Rounding, Stroke};
use crate::domain::models::Task;
use crate::ui::theme::Theme;

pub struct CalendarView;

impl CalendarView {
    pub fn render(
        ui: &mut egui::Ui,
        _project_start_date: NaiveDate,
        tasks: &[Task],
        selected_task_id: &mut Option<String>,
        on_open_task_modal: &mut bool,
    ) {
        ui.vertical(|ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("📅 Planning Chronologique des Activités").color(Theme::TEXT_TITLE));
            });
            ui.label(egui::RichText::new("Visualisation ordonnée des dates d'exécution calculées par le moteur de chemin critique (CPM).").color(Theme::TEXT_MUTED));
            ui.add_space(12.0);

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let mut sorted_tasks = tasks.to_vec();
                    sorted_tasks.sort_by_key(|t| t.early_start.unwrap_or(t.start_date));

                    for task in sorted_tasks {
                        let is_selected = selected_task_id.as_deref() == Some(&task.id);
                        let es = task.early_start.unwrap_or(task.start_date);
                        let ef = task.early_finish.unwrap_or(task.end_date);

                        let card_bg = if is_selected {
                            Theme::CARD_HOVER
                        } else {
                            Theme::CARD_BG
                        };

                        let border_stroke = if is_selected {
                            Stroke::new(1.5, Theme::ACCENT_PRIMARY)
                        } else if task.is_critical {
                            Stroke::new(1.0, Theme::CRITICAL_PATH.gamma_multiply(0.6))
                        } else {
                            Stroke::new(1.0, Theme::BORDER)
                        };

                        let response = egui::Frame::none()
                            .fill(card_bg)
                            .stroke(border_stroke)
                            .rounding(Rounding::same(8.0))
                            .inner_margin(egui::Margin::symmetric(16.0, 12.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let tag_color = if task.is_critical {
                                        Theme::CRITICAL_PATH
                                    } else {
                                        Theme::ACCENT_PRIMARY
                                    };

                                    egui::Frame::none()
                                        .fill(tag_color.gamma_multiply(0.15))
                                        .rounding(Rounding::same(4.0))
                                        .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                        .show(ui, |ui| {
                                            ui.label(egui::RichText::new(format!("{} ➔ {}", es.format("%d/%m/%Y"), ef.format("%d/%m/%Y"))).color(tag_color).strong().size(12.0));
                                        });

                                    ui.add_space(8.0);
                                    ui.label(egui::RichText::new(&task.title).color(Theme::TEXT_TITLE).strong().size(13.0));

                                    if task.is_critical {
                                        ui.colored_label(Theme::CRITICAL_PATH, "● CHEMIN CRITIQUE");
                                    }

                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(egui::RichText::new(format!("⏱ {}h | Statut : {}", task.duration_hours, task.status.label_fr())).color(Theme::TEXT_MUTED).size(11.5));
                                    });
                                });
                            });

                        if response.response.clicked() {
                            *selected_task_id = Some(task.id.clone());
                        }
                        if response.response.double_clicked() {
                            *selected_task_id = Some(task.id.clone());
                            *on_open_task_modal = true;
                        }
                        ui.add_space(8.0);
                    }
                });
        });
    }
}
