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
            ui.heading("📅 Planning Chronologique des Activités");
            ui.label("Visualisation des dates au plus tôt calculées par le moteur de chemin critique (CPM).");
            ui.add_space(10.0);

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

                        let response = egui::Frame::none()
                            .fill(card_bg)
                            .stroke(Stroke::new(
                                if is_selected { 2.0 } else { 1.0 },
                                if is_selected { Theme::ACCENT_PRIMARY } else { Theme::BORDER },
                            ))
                            .rounding(Rounding::same(6.0))
                            .inner_margin(egui::Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let tag_color = if task.is_critical {
                                        Theme::CRITICAL_PATH
                                    } else {
                                        Theme::ACCENT_PRIMARY
                                    };
                                    ui.colored_label(tag_color, format!("{} à {}", es.format("%d/%m/%Y"), ef.format("%d/%m/%Y")));
                                    ui.separator();
                                    ui.strong(&task.title);
                                    if task.is_critical {
                                        ui.colored_label(Theme::CRITICAL_PATH, "● CHEMIN CRITIQUE");
                                    }
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(format!("Durée : {}h | Statut : {}", task.duration_hours, task.status.label_fr()));
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
                        ui.add_space(6.0);
                    }
                });
        });
    }
}
