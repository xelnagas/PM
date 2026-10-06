use chrono::{Datelike, Duration, NaiveDate, Weekday};
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
                ui.add_space(80.0);
                ui.label(egui::RichText::new("📊").size(48.0));
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("Aucune tâche dans ce projet").color(Theme::TEXT_TITLE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("Cliquez sur '+ Nouvelle Tâche' ci-dessus pour planifier vos premières activités.").color(Theme::TEXT_MUTED));
            });
            return;
        }

        let day_width = 40.0;
        let row_height = 42.0;
        let sidebar_width = 280.0;

        let max_days = schedule
            .map(|s| s.total_duration_days.max(14) + 6)
            .unwrap_or(20) as usize;

        let total_canvas_width = sidebar_width + (max_days as f32 * day_width);
        let total_canvas_height = (tasks.len() as f32 * row_height) + 60.0;

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let (response, painter) = ui.allocate_painter(
                    Vec2::new(total_canvas_width, total_canvas_height),
                    egui::Sense::click(),
                );
                let origin = response.rect.min;

                // 1. En-tête Temporel (Axe horizontal des Jours & Dates)
                let header_rect = Rect::from_min_size(origin, Vec2::new(total_canvas_width, 48.0));
                painter.rect_filled(header_rect, Rounding::ZERO, Theme::HEADER_BG);
                painter.line_segment(
                    [Pos2::new(origin.x, origin.y + 48.0), Pos2::new(origin.x + total_canvas_width, origin.y + 48.0)],
                    Stroke::new(1.0, Theme::BORDER),
                );

                // Libellé colonne latérale
                painter.text(
                    Pos2::new(origin.x + 16.0, origin.y + 24.0),
                    egui::Align2::LEFT_CENTER,
                    "ACTIVITÉS & JALONS",
                    egui::FontId::proportional(11.0),
                    Theme::TEXT_MUTED,
                );

                // Tracé des colonnes journalières
                for day_idx in 0..max_days {
                    let current_date = project_start_date + Duration::days(day_idx as i64);
                    let x = origin.x + sidebar_width + (day_idx as f32 * day_width);
                    let is_weekend = matches!(current_date.weekday(), Weekday::Sat | Weekday::Sun);

                    if is_weekend {
                        let weekend_rect = Rect::from_min_size(
                            Pos2::new(x, origin.y + 48.0),
                            Vec2::new(day_width, total_canvas_height - 48.0),
                        );
                        painter.rect_filled(weekend_rect, Rounding::ZERO, Color32::from_black_alpha(40));
                    }

                    painter.line_segment(
                        [Pos2::new(x, origin.y), Pos2::new(x, origin.y + total_canvas_height)],
                        Stroke::new(1.0, Theme::BORDER_SUBTLE),
                    );

                    let day_label = match current_date.weekday() {
                        Weekday::Mon => "Lun",
                        Weekday::Tue => "Mar",
                        Weekday::Wed => "Mer",
                        Weekday::Thu => "Jeu",
                        Weekday::Fri => "Ven",
                        Weekday::Sat => "Sam",
                        Weekday::Sun => "Dim",
                    };

                    painter.text(
                        Pos2::new(x + day_width * 0.5, origin.y + 16.0),
                        egui::Align2::CENTER_CENTER,
                        day_label,
                        egui::FontId::proportional(10.0),
                        if is_weekend { Theme::TEXT_SUBTLE } else { Theme::TEXT_MUTED },
                    );
                    painter.text(
                        Pos2::new(x + day_width * 0.5, origin.y + 32.0),
                        egui::Align2::CENTER_CENTER,
                        current_date.format("%d/%m").to_string(),
                        egui::FontId::proportional(11.0),
                        if is_weekend { Theme::TEXT_MUTED } else { Theme::TEXT_TITLE },
                    );
                }

                let mut bar_endpoints: HashMap<String, (Pos2, Pos2)> = HashMap::new();

                // 2. Tracé des Lignes de Tâches
                for (row_idx, task) in tasks.iter().enumerate() {
                    let y = origin.y + 54.0 + (row_idx as f32 * row_height);
                    let is_selected = selected_task_id.as_deref() == Some(&task.id);

                    let row_sidebar_rect = Rect::from_min_size(
                        Pos2::new(origin.x, y),
                        Vec2::new(sidebar_width, row_height),
                    );
                    let row_full_rect = Rect::from_min_size(
                        Pos2::new(origin.x, y),
                        Vec2::new(total_canvas_width, row_height),
                    );

                    let is_hovered = ui.input(|i| i.pointer.hover_pos()).map_or(false, |pos| row_full_rect.contains(pos));

                    let bg_color = if is_selected {
                        Theme::ACCENT_PRIMARY.gamma_multiply(0.2)
                    } else if is_hovered {
                        Theme::CARD_HOVER
                    } else if row_idx % 2 == 0 {
                        Theme::CARD_BG.gamma_multiply(0.4)
                    } else {
                        Theme::PANEL_BG
                    };

                    painter.rect_filled(row_full_rect, Rounding::ZERO, bg_color);
                    painter.line_segment(
                        [Pos2::new(origin.x, y + row_height), Pos2::new(origin.x + total_canvas_width, y + row_height)],
                        Stroke::new(1.0, Theme::BORDER_SUBTLE),
                    );

                    painter.line_segment(
                        [Pos2::new(origin.x + sidebar_width, origin.y), Pos2::new(origin.x + sidebar_width, origin.y + total_canvas_height)],
                        Stroke::new(1.5, Theme::BORDER),
                    );

                    let indicator_color = if task.is_critical {
                        Theme::CRITICAL_PATH
                    } else if task.status == crate::domain::models::TaskStatus::Done {
                        Theme::SUCCESS
                    } else {
                        Theme::ACCENT_PRIMARY
                    };
                    painter.circle_filled(Pos2::new(origin.x + 16.0, y + row_height * 0.5), 4.5, indicator_color);

                    let title_color = if is_selected {
                        Theme::TEXT_TITLE
                    } else if task.is_critical {
                        Theme::TEXT_TITLE
                    } else {
                        Theme::TEXT_PRIMARY
                    };

                    painter.text(
                        Pos2::new(origin.x + 30.0, y + row_height * 0.5),
                        egui::Align2::LEFT_CENTER,
                        &task.title,
                        egui::FontId::proportional(12.5),
                        title_color,
                    );

                    let start_offset_days = (task.early_start.unwrap_or(task.start_date) - project_start_date)
                        .num_days()
                        .max(0) as f32;
                    let duration_days = ((task.duration_hours + 7) / 8).max(1) as f32;

                    let bar_x_start = origin.x + sidebar_width + (start_offset_days * day_width) + 3.0;
                    let bar_width = (duration_days * day_width) - 6.0;
                    let bar_rect = Rect::from_min_size(
                        Pos2::new(bar_x_start, y + 8.0),
                        Vec2::new(bar_width.max(22.0), row_height - 16.0),
                    );

                    let (bar_bg, bar_border) = if task.is_critical {
                        (Theme::CRITICAL_PATH, Stroke::new(1.5, Theme::CRITICAL_GLOW))
                    } else if task.status == crate::domain::models::TaskStatus::Done {
                        (Theme::SUCCESS, Stroke::new(1.0, Color32::from_rgb(52, 211, 153)))
                    } else {
                        (Theme::ACCENT_PRIMARY, Stroke::new(1.0, Theme::ACCENT_HOVER))
                    };

                    painter.rect_filled(bar_rect, Rounding::same(6.0), bar_bg);
                    painter.rect_stroke(bar_rect, Rounding::same(6.0), bar_border);

                    if is_selected {
                        painter.rect_stroke(
                            bar_rect.expand(2.0),
                            Rounding::same(8.0),
                            Stroke::new(2.0, Color32::WHITE),
                        );
                    }

                    let duration_text = format!("{}h", task.duration_hours);
                    painter.text(
                        Pos2::new(bar_rect.center().x, bar_rect.center().y),
                        egui::Align2::CENTER_CENTER,
                        duration_text,
                        egui::FontId::proportional(11.0),
                        Color32::WHITE,
                    );

                    let start_point = Pos2::new(bar_rect.min.x, bar_rect.center().y);
                    let end_point = Pos2::new(bar_rect.max.x, bar_rect.center().y);
                    bar_endpoints.insert(task.id.clone(), (start_point, end_point));

                    if response.clicked() {
                        if let Some(mouse_pos) = ui.input(|i| i.pointer.interact_pos()) {
                            if row_sidebar_rect.contains(mouse_pos) || bar_rect.contains(mouse_pos) {
                                *selected_task_id = Some(task.id.clone());
                                if response.double_clicked() {
                                    *on_open_task_modal = true;
                                }
                            }
                        }
                    }
                }

                // 3. Tracé Vectoriel des Dépendances en Courbes de Bézier Cubiques
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

                        let dx = (succ_start.x - pred_end.x).max(24.0) * 0.5;
                        let cp1 = Pos2::new(pred_end.x + dx, pred_end.y);
                        let cp2 = Pos2::new(succ_start.x - dx, succ_start.y);

                        let bezier = CubicBezierShape::from_points_stroke(
                            [*pred_end, cp1, cp2, *succ_start],
                            false,
                            Color32::TRANSPARENT,
                            Stroke::new(stroke_width, stroke_color),
                        );
                        painter.add(bezier);

                        let arrow_size = 5.5;
                        let tip = *succ_start;
                        let p1 = Pos2::new(tip.x - arrow_size * 1.6, tip.y - arrow_size);
                        let p2 = Pos2::new(tip.x - arrow_size * 1.6, tip.y + arrow_size);
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
