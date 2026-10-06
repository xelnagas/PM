use egui::{Color32, Frame, Margin, Pos2, Rounding, Stroke, Vec2};
use crate::domain::models::{Project, Task, TaskPriority};
use crate::ui::theme::Theme;

pub struct DashboardView;

impl DashboardView {
    pub fn render(
        ui: &mut egui::Ui,
        projects: &[Project],
        tasks: &[Task],
        active_project_id: &mut Option<String>,
        on_select_project: &mut Option<String>,
        on_open_task_modal: &mut bool,
        selected_task_id: &mut Option<String>,
    ) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(6.0);

                // 1. Barre de Titre du Dashboard & Filtres
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🖥️").size(20.0));
                    ui.heading(egui::RichText::new("Tableau de bord").color(Theme::TEXT_TITLE).size(18.0));
                    
                    egui::Frame::none()
                        .fill(Theme::PANEL_BG)
                        .stroke(Stroke::new(1.0, Theme::BORDER))
                        .rounding(Rounding::same(4.0))
                        .inner_margin(Margin::symmetric(8.0, 3.0))
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("par défaut ▼").color(Theme::TEXT_MUTED).size(11.0));
                        });

                    ui.add_space(12.0);

                    ui.label(egui::RichText::new("Filtrer par :").color(Theme::TEXT_SUBTLE).size(11.0));

                    render_filter_pill(ui, "Catégorie personnelle ▼", Theme::PILL_BLUE);
                    render_filter_pill(ui, "Catégorie d'entreprise ▼", Theme::PILL_GREEN);
                    render_filter_pill(ui, "Étiquettes d'activités ▼", Theme::PILL_BURGUNDY);
                });

                ui.add_space(14.0);

                // 2. Grille Principale (2 Colonnes)
                ui.columns(2, |cols| {
                    // --- COLONNE GAUCHE (65% Largeur) : PROJETS ACTIFS & TÂCHES ---
                    let left_ui = &mut cols[0];
                    left_ui.set_width(left_ui.available_width() * 1.15);

                    // Cartouche "Projets actifs"
                    Frame::none()
                        .fill(Theme::PANEL_BG)
                        .stroke(Stroke::new(1.0, Theme::BORDER))
                        .rounding(Rounding::same(6.0))
                        .inner_margin(Margin::same(0.0))
                        .show(left_ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(10.0);
                                ui.label(egui::RichText::new("📑").size(14.0));
                                ui.strong(egui::RichText::new("Projets actifs").color(Theme::TEXT_TITLE).size(13.0));
                                
                                Frame::none()
                                    .fill(Theme::TEXT_MUTED)
                                    .rounding(Rounding::same(10.0))
                                    .inner_margin(Margin::symmetric(6.0, 1.0))
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new(projects.len().to_string()).color(Color32::WHITE).size(10.5));
                                    });

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_space(10.0);
                                    ui.label(egui::RichText::new("▲").color(Theme::TEXT_MUTED).size(11.0));
                                });
                            });

                            ui.add_space(4.0);

                            // En-tête vert du tableau
                            let (header_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 26.0), egui::Sense::hover());
                            ui.painter().rect_filled(header_rect, Rounding::ZERO, Theme::HEADER_GREEN);

                            let origin = header_rect.min;
                            let text_y = origin.y + 13.0;

                            ui.painter().text(Pos2::new(origin.x + 12.0, text_y), egui::Align2::LEFT_CENTER, "Projet", egui::FontId::proportional(11.5), Color32::WHITE);
                            ui.painter().text(Pos2::new(origin.x + 220.0, text_y), egui::Align2::CENTER_CENTER, "Criticité", egui::FontId::proportional(11.0), Color32::WHITE);
                            ui.painter().text(Pos2::new(origin.x + 270.0, text_y), egui::Align2::CENTER_CENTER, "Météo", egui::FontId::proportional(11.0), Color32::WHITE);
                            ui.painter().text(Pos2::new(origin.x + 320.0, text_y), egui::Align2::CENTER_CENTER, "Tendance", egui::FontId::proportional(11.0), Color32::WHITE);
                            ui.painter().text(Pos2::new(origin.x + 370.0, text_y), egui::Align2::CENTER_CENTER, "%", egui::FontId::proportional(11.0), Color32::WHITE);
                            ui.painter().text(Pos2::new(origin.x + 440.0, text_y), egui::Align2::LEFT_CENTER, "Statut", egui::FontId::proportional(11.0), Color32::WHITE);

                            // Lignes des projets
                            for (idx, p) in projects.iter().enumerate() {
                                let is_active = active_project_id.as_deref() == Some(&p.id);
                                let row_bg = if is_active {
                                    Theme::SIDEBAR_BG.gamma_multiply(0.08)
                                } else if idx % 2 == 0 {
                                    Theme::PANEL_BG
                                } else {
                                    Theme::CARD_HOVER.gamma_multiply(0.5)
                                };

                                Frame::none()
                                    .fill(row_bg)
                                    .inner_margin(Margin::symmetric(10.0, 7.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            let p_colors = [Color32::from_rgb(46, 204, 113), Color32::from_rgb(155, 89, 182), Color32::from_rgb(52, 152, 219), Color32::from_rgb(230, 126, 34), Color32::from_rgb(231, 76, 60)];
                                            let color = p_colors[idx % p_colors.len()];

                                            let (sq, _) = ui.allocate_exact_size(Vec2::new(8.0, 8.0), egui::Sense::hover());
                                            ui.painter().rect_filled(sq, Rounding::ZERO, color);

                                            if ui.selectable_label(is_active, egui::RichText::new(&p.name).color(Theme::TEXT_TITLE).size(12.0)).clicked() {
                                                *active_project_id = Some(p.id.clone());
                                                *on_select_project = Some(p.id.clone());
                                            }

                                            ui.add_space(20.0);
                                            let (crit_icon, crit_color) = match idx % 3 {
                                                0 => ("🌡️", Theme::SIDEBAR_BG),
                                                1 => ("🌡️", Theme::WARNING),
                                                _ => ("🌡️", Theme::CRITICAL_PATH),
                                            };
                                            ui.colored_label(crit_color, crit_icon);

                                            ui.add_space(22.0);
                                            let meteo_icon = match idx % 4 {
                                                0 => "☀️",
                                                1 => "⛅",
                                                2 => "🌧️",
                                                _ => "☀️",
                                            };
                                            ui.label(meteo_icon);

                                            ui.add_space(22.0);
                                            let (trend_icon, trend_color) = match idx % 3 {
                                                0 => ("➡️", Theme::TEXT_MUTED),
                                                1 => ("↗️", Theme::SUCCESS),
                                                _ => ("↘️", Theme::CRITICAL_PATH),
                                            };
                                            ui.colored_label(trend_color, trend_icon);

                                            ui.add_space(18.0);
                                            let pct = ((idx * 27 + 18) % 95) + 5;
                                            let pct_bg = if pct > 50 { Color32::from_rgb(52, 152, 219) } else { Color32::from_rgb(230, 126, 34) };
                                            Frame::none()
                                                .fill(pct_bg.gamma_multiply(0.15))
                                                .stroke(Stroke::new(1.0, pct_bg))
                                                .rounding(Rounding::same(3.0))
                                                .inner_margin(Margin::symmetric(4.0, 1.0))
                                                .show(ui, |ui| {
                                                    ui.label(egui::RichText::new(format!("{}%", pct)).color(pct_bg).strong().size(10.0));
                                                });

                                            ui.add_space(16.0);
                                            let (status_text, status_color) = match idx % 5 {
                                                0 => ("En cours de validation", Theme::STATUS_VALIDATION),
                                                1 => ("Demande de lancement", Theme::STATUS_LAUNCH),
                                                2 => ("En cours de réalisation", Theme::STATUS_PROGRESS),
                                                3 => ("En phase d'initialisation", Theme::STATUS_INIT),
                                                _ => ("Terminé", Theme::STATUS_DONE),
                                            };

                                            Frame::none()
                                                .fill(status_color)
                                                .rounding(Rounding::same(3.0))
                                                .inner_margin(Margin::symmetric(6.0, 2.0))
                                                .show(ui, |ui| {
                                                    ui.label(egui::RichText::new(status_text).color(Color32::WHITE).size(10.0));
                                                });
                                        });
                                    });
                            }

                            // Pagination
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                ui.add_space(ui.available_width() * 0.35);
                                for num in 1..=4 {
                                    let is_first = num == 1;
                                    let (bg, txt) = if is_first { (Theme::STATUS_LAUNCH.gamma_multiply(0.3), Theme::STATUS_LAUNCH) } else { (Color32::TRANSPARENT, Theme::TEXT_MUTED) };
                                    Frame::none()
                                        .fill(bg)
                                        .rounding(Rounding::same(3.0))
                                        .inner_margin(Margin::symmetric(6.0, 2.0))
                                        .show(ui, |ui| {
                                            ui.label(egui::RichText::new(num.to_string()).color(txt).size(11.0));
                                        });
                                }
                            });
                            ui.add_space(8.0);
                        });

                    // Cartouche inférieur "Mes tâches"
                    left_ui.add_space(12.0);
                    Frame::none()
                        .fill(Theme::PANEL_BG)
                        .stroke(Stroke::new(1.0, Theme::BORDER))
                        .rounding(Rounding::same(6.0))
                        .inner_margin(Margin::symmetric(12.0, 10.0))
                        .show(left_ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("📝").size(14.0));
                                ui.strong(egui::RichText::new("Mes tâches").color(Theme::TEXT_TITLE).size(13.0));
                                Frame::none()
                                    .fill(Theme::TEXT_MUTED)
                                    .rounding(Rounding::same(10.0))
                                    .inner_margin(Margin::symmetric(6.0, 1.0))
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new(tasks.len().to_string()).color(Color32::WHITE).size(10.5));
                                    });

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(egui::RichText::new("▲").color(Theme::TEXT_MUTED).size(11.0));
                                });
                            });
                            
                            ui.add_space(6.0);
                            for t in tasks.iter().take(4) {
                                ui.horizontal(|ui| {
                                    ui.colored_label(Theme::SIDEBAR_BG, "●");
                                    if ui.selectable_label(selected_task_id.as_deref() == Some(&t.id), egui::RichText::new(&t.title).size(11.5)).clicked() {
                                        *selected_task_id = Some(t.id.clone());
                                        *on_open_task_modal = true;
                                    }
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(egui::RichText::new(format!("{}h | {}", t.duration_hours, t.status.label_fr())).color(Theme::TEXT_MUTED).size(10.5));
                                    });
                                });
                            }
                        });

                    // --- COLONNE DROITE (35% Largeur) : WIDGETS NOTIFICATIONS / PROBLÈMES / JALONS ---
                    let right_ui = &mut cols[1];

                    // 1. Widget Notifications non lues
                    Frame::none()
                        .fill(Theme::PANEL_BG)
                        .stroke(Stroke::new(1.0, Theme::BORDER))
                        .rounding(Rounding::same(6.0))
                        .inner_margin(Margin::symmetric(12.0, 10.0))
                        .show(right_ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("✉️").size(14.0));
                                ui.strong(egui::RichText::new("Notifications non lues").color(Theme::TEXT_TITLE).size(12.5));
                                Frame::none()
                                    .fill(Theme::TEXT_MUTED)
                                    .rounding(Rounding::same(10.0))
                                    .inner_margin(Margin::symmetric(6.0, 1.0))
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new("0").color(Color32::WHITE).size(10.5));
                                    });
                            });
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new("Aucune notification non lue.").color(Theme::TEXT_MUTED).size(11.0));
                        });

                    // 2. Widget "Mes problèmes / Tâches critiques"
                    right_ui.add_space(10.0);
                    Frame::none()
                        .fill(Theme::PANEL_BG)
                        .stroke(Stroke::new(1.0, Theme::BORDER))
                        .rounding(Rounding::same(6.0))
                        .inner_margin(Margin::same(0.0))
                        .show(right_ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(10.0);
                                ui.colored_label(Theme::CRITICAL_PATH, "⚠️");
                                ui.strong(egui::RichText::new("Mes problèmes").color(Theme::TEXT_TITLE).size(12.5));
                                
                                let critical_count = tasks.iter().filter(|t| t.is_critical || t.priority == TaskPriority::Urgent).count();
                                Frame::none()
                                    .fill(Theme::CRITICAL_PATH)
                                    .rounding(Rounding::same(10.0))
                                    .inner_margin(Margin::symmetric(6.0, 1.0))
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new(critical_count.to_string()).color(Color32::WHITE).size(10.5));
                                    });
                            });
                            ui.add_space(4.0);

                            let (header_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 24.0), egui::Sense::hover());
                            ui.painter().rect_filled(header_rect, Rounding::ZERO, Theme::HEADER_RED);
                            let o = header_rect.min;
                            ui.painter().text(Pos2::new(o.x + 8.0, o.y + 12.0), egui::Align2::LEFT_CENTER, "Projet", egui::FontId::proportional(11.0), Color32::WHITE);
                            ui.painter().text(Pos2::new(o.x + 100.0, o.y + 12.0), egui::Align2::LEFT_CENTER, "Sujet", egui::FontId::proportional(11.0), Color32::WHITE);
                            ui.painter().text(Pos2::new(o.x + ui.available_width() - 8.0, o.y + 12.0), egui::Align2::RIGHT_CENTER, "Priorité", egui::FontId::proportional(11.0), Color32::WHITE);

                            for t in tasks.iter().filter(|t| t.is_critical || t.priority == TaskPriority::Urgent).take(3) {
                                Frame::none()
                                    .inner_margin(Margin::symmetric(8.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("Projet Démo").color(Theme::TEXT_MUTED).size(10.5));
                                            ui.add_space(8.0);
                                            ui.label(egui::RichText::new(&t.title).color(Theme::TEXT_TITLE).size(11.0));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.colored_label(Theme::CRITICAL_PATH, t.priority.label_fr());
                                            });
                                        });
                                    });
                            }
                        });

                    // 3. Widget "Prochains jalons"
                    right_ui.add_space(10.0);
                    Frame::none()
                        .fill(Theme::PANEL_BG)
                        .stroke(Stroke::new(1.0, Theme::BORDER))
                        .rounding(Rounding::same(6.0))
                        .inner_margin(Margin::same(0.0))
                        .show(right_ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(10.0);
                                ui.label(egui::RichText::new("📌").size(14.0));
                                ui.strong(egui::RichText::new("Prochains jalons").color(Theme::TEXT_TITLE).size(12.5));
                                Frame::none()
                                    .fill(Theme::TEXT_MUTED)
                                    .rounding(Rounding::same(10.0))
                                    .inner_margin(Margin::symmetric(6.0, 1.0))
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new(tasks.len().to_string()).color(Color32::WHITE).size(10.5));
                                    });
                            });
                            ui.add_space(4.0);

                            let (header_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 24.0), egui::Sense::hover());
                            ui.painter().rect_filled(header_rect, Rounding::ZERO, Theme::HEADER_GREEN);
                            let o = header_rect.min;
                            ui.painter().text(Pos2::new(o.x + 8.0, o.y + 12.0), egui::Align2::LEFT_CENTER, "Jalon", egui::FontId::proportional(11.0), Color32::WHITE);
                            ui.painter().text(Pos2::new(o.x + 110.0, o.y + 12.0), egui::Align2::LEFT_CENTER, "Projet", egui::FontId::proportional(11.0), Color32::WHITE);
                            ui.painter().text(Pos2::new(o.x + ui.available_width() - 8.0, o.y + 12.0), egui::Align2::RIGHT_CENTER, "Date", egui::FontId::proportional(11.0), Color32::WHITE);

                            for t in tasks.iter().take(4) {
                                Frame::none()
                                    .inner_margin(Margin::symmetric(8.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.colored_label(Theme::CRITICAL_PATH, "⚑");
                                            ui.label(egui::RichText::new(&t.title).color(Theme::TEXT_TITLE).size(11.0));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                let d = t.early_start.unwrap_or(t.start_date);
                                                ui.label(egui::RichText::new(d.format("%d/%m/%Y").to_string()).color(Theme::TEXT_MUTED).size(10.5));
                                            });
                                        });
                                    });
                            }
                        });
                });
            });
    }
}

fn render_filter_pill(ui: &mut egui::Ui, label: &str, color: Color32) {
    Frame::none()
        .fill(color)
        .rounding(Rounding::same(3.0))
        .inner_margin(Margin::symmetric(8.0, 3.0))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(label).color(Color32::WHITE).size(11.0));
        });
}
