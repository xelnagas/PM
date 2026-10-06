use chrono::{Duration, NaiveDate};
use egui::{epaint::CubicBezierShape, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use std::collections::HashMap;
use crate::domain::models::{ScheduleResult, Task, TaskDependency};
use crate::ui::theme::Theme;

pub struct GanttView;

impl GanttView {
    pub fn render(
        ui: &mut egui::Ui,
        project_start_date: NaiveDate,
        tasks: &[Task],
        dependencies: &[TaskDependency],
        schedule: Option<&ScheduleResult>,
        selected_task_id: &mut Option<String>,
        on_open_task_modal: &mut bool,
    ) {
        if tasks.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                ui.heading("Aucune tâche dans ce projet");
                ui.label("Cliquez sur '+ Nouvelle Tâche' ci-dessus pour planifier vos premières activités.");
            });
            return;
        }

        let day_width = 36.0;
        let row_height = 36.0;
        let sidebar_width = 240.0;

        // Calcul de la durée totale couverte par le Gantt
        let max_days = schedule
            .map(|s| s.total_duration_days.max(14) + 5)
            .unwrap_or(20) as usize;

        let total_canvas_width = sidebar_width + (max_days as f32 * day_width);
        let total_canvas_height = (tasks.len() as f32 * row_height) + 50.0;

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let (response, painter) = ui.allocate_painter(
                    Vec2::new(total_canvas_width, total_canvas_height),
                    egui::Sense::click(),
                );
                let origin = response.rect.min;

                // 1. En-tête temporel (Jours & Dates)
                let header_rect = Rect::from_min_size(origin, Vec2::new(total_canvas_width, 40.0));
                painter.rect_filled(header_rect, Rounding::ZERO, Theme::PANEL_BG);

                painter.text(
                    Pos2::new(origin.x + 12.0, origin.y + 14.0),
                    egui::Align2::LEFT_CENTER,
                    "TÂCHE / ACTIVITÉ",
                    egui::FontId::proportional(12.0),
                    Theme::TEXT_MUTED,
                );

                for day_idx in 0..max_days {
                    let current_date = project_start_date + Duration::days(day_idx as i64);
                    let x = origin.x + sidebar_width + (day_idx as f32 * day_width);
                    
                    // Ligne verticale de séparation de jour
                    painter.line_segment(
                        [Pos2::new(x, origin.y), Pos2::new(x, origin.y + total_canvas_height)],
                        Stroke::new(1.0, Theme::BORDER.gamma_multiply(0.4)),
                    );

                    // Libellé de date
                    painter.text(
                        Pos2::new(x + day_width * 0.5, origin.y + 12.0),
                        egui::Align2::CENTER_CENTER,
                        format!("J{}", day_idx + 1),
                        egui::FontId::proportional(11.0),
                        Theme::TEXT_PRIMARY,
                    );
                    painter.text(
                        Pos2::new(x + day_width * 0.5, origin.y + 26.0),
                        egui::Align2::CENTER_CENTER,
                        current_date.format("%d/%m").to_string(),
                        egui::FontId::proportional(9.0),
                        Theme::TEXT_MUTED,
                    );
                }

                // Map pour stocker les coordonnées exactes d'ancrage des barres pour le tracé des flèches
                let mut bar_endpoints: HashMap<String, (Pos2, Pos2)> = HashMap::new();

                // 2. Tracé des Lignes de Tâches
                for (row_idx, task) in tasks.iter().enumerate() {
                    let y = origin.y + 45.0 + (row_idx as f32 * row_height);
                    let is_selected = selected_task_id.as_deref() == Some(&task.id);

                    // Fond de la ligne du panneau latéral
                    let row_rect = Rect::from_min_size(
                        Pos2::new(origin.x, y),
                        Vec2::new(sidebar_width, row_height),
                    );
                    let bg_color = if is_selected {
                        Theme::CARD_HOVER
                    } else if row_idx % 2 == 0 {
                        Theme::CARD_BG.gamma_multiply(0.5)
                    } else {
                        Theme::PANEL_BG
                    };
                    painter.rect_filled(row_rect, Rounding::ZERO, bg_color);

                    // Indicateur visuel critique ou normal
                    let indicator_color = if task.is_critical {
                        Theme::CRITICAL_PATH
                    } else {
                        Theme::ACCENT_PRIMARY
                    };
                    painter.circle_filled(Pos2::new(origin.x + 12.0, y + row_height * 0.5), 4.0, indicator_color);

                    // Titre de la tâche dans la sidebar
                    painter.text(
                        Pos2::new(origin.x + 24.0, y + row_height * 0.5),
                        egui::Align2::LEFT_CENTER,
                        &task.title,
                        egui::FontId::proportional(13.0),
                        if is_selected { Theme::ACCENT_PRIMARY } else { Theme::TEXT_PRIMARY },
                    );

                    // Calcul de la position de la barre de tâche sur la timeline
                    let start_offset_days = (task.early_start.unwrap_or(task.start_date) - project_start_date)
                        .num_days()
                        .max(0) as f32;
                    let duration_days = ((task.duration_hours + 7) / 8).max(1) as f32;

                    let bar_x_start = origin.x + sidebar_width + (start_offset_days * day_width) + 2.0;
                    let bar_width = (duration_days * day_width) - 4.0;
                    let bar_rect = Rect::from_min_size(
                        Pos2::new(bar_x_start, y + 6.0),
                        Vec2::new(bar_width.max(16.0), row_height - 12.0),
                    );

                    let bar_color = if task.is_critical {
                        Theme::CRITICAL_PATH
                    } else if task.status == crate::domain::models::TaskStatus::Done {
                        Theme::SUCCESS
                    } else {
                        Theme::ACCENT_PRIMARY
                    };

                    // Rendu de la barre
                    painter.rect_filled(bar_rect, Rounding::same(5.0), bar_color);
                    if is_selected {
                        painter.rect_stroke(bar_rect, Rounding::same(5.0), Stroke::new(2.0, Theme::TEXT_PRIMARY));
                    }

                    // Libellé de durée sur la barre
                    painter.text(
                        Pos2::new(bar_rect.center().x, bar_rect.center().y),
                        egui::Align2::CENTER_CENTER,
                        format!("{}h", task.duration_hours),
                        egui::FontId::proportional(11.0),
                        Color32::WHITE,
                    );

                    // Enregistrement des points d'attache
                    let start_point = Pos2::new(bar_rect.min.x, bar_rect.center().y);
                    let end_point = Pos2::new(bar_rect.max.x, bar_rect.center().y);
                    bar_endpoints.insert(task.id.clone(), (start_point, end_point));

                    // Interaction clic sur la ligne
                    let full_row_rect = Rect::from_min_size(
                        Pos2::new(origin.x, y),
                        Vec2::new(total_canvas_width, row_height),
                    );
                    if response.clicked() {
                        if let Some(mouse_pos) = ui.input(|i| i.pointer.interact_pos()) {
                            if full_row_rect.contains(mouse_pos) {
                                *selected_task_id = Some(task.id.clone());
                                if response.double_clicked() {
                                    *on_open_task_modal = true;
                                }
                            }
                        }
                    }
                }

                // 3. Tracé des Flèches de Dépendance en Courbes de Bézier Cubiques
                for dep in dependencies {
                    if let (Some((_, pred_end)), Some((succ_start, _))) = (
                        bar_endpoints.get(&dep.predecessor_task_id),
                        bar_endpoints.get(&dep.successor_task_id),
                    ) {
                        let is_dep_critical = schedule.map(|s| {
                            s.critical_path_task_ids.contains(&dep.predecessor_task_id)
                                && s.critical_path_task_ids.contains(&dep.successor_task_id)
                        }).unwrap_or(false);

                        let stroke_color = if is_dep_critical {
                            Theme::DEP_ARROW_CRITICAL
                        } else {
                            Theme::DEP_ARROW_NORMAL
                        };
                        let stroke_width = if is_dep_critical { 2.5 } else { 1.5 };

                        let dx = (succ_start.x - pred_end.x).max(20.0) * 0.5;
                        let cp1 = Pos2::new(pred_end.x + dx, pred_end.y);
                        let cp2 = Pos2::new(succ_start.x - dx, succ_start.y);

                        let bezier = CubicBezierShape::from_points_stroke(
                            [*pred_end, cp1, cp2, *succ_start],
                            false,
                            Color32::TRANSPARENT,
                            Stroke::new(stroke_width, stroke_color),
                        );
                        painter.add(bezier);

                        // Pointe de flèche triangulaire
                        let arrow_size = 5.0;
                        let tip = *succ_start;
                        let p1 = Pos2::new(tip.x - arrow_size * 1.5, tip.y - arrow_size);
                        let p2 = Pos2::new(tip.x - arrow_size * 1.5, tip.y + arrow_size);
                        painter.add(egui::Shape::convex_polygon(
                            vec![tip, p1, p2],
                            stroke_color,
                            Stroke::NONE,
                        ));
                    }
                }
            });
    }
}
