use chrono::Local;
use eframe::egui;
use egui::{Color32, Frame, Margin, Rounding, Stroke, Vec2};
use parking_lot::Mutex;
use std::sync::Arc;

use crate::domain::models::*;
use crate::services::SchedulerService;
use crate::storage::Database;
use crate::ui::calendar_view::CalendarView;
use crate::ui::dashboard_view::DashboardView;
use crate::ui::gantt_view::GanttView;
use crate::ui::kanban_view::KanbanView;
use crate::ui::modals::*;
use crate::ui::theme::Theme;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum NavigationTab {
    Dashboard,      // Tableau de bord / Mon travail
    Portfolio,      // Portefeuille projets
    GanttPlanning,  // Planning / Gantt du projet actif
    Kanban,         // Kanban du projet actif
    Calendar,       // Calendrier chronologique
    Timesheet,      // Feuille de temps
    KnowledgeBase,  // Base de connaissance
    Members,        // Intervenants / Gestion
}

pub struct PmApp {
    db: Arc<Mutex<Database>>,
    scheduler: SchedulerService,

    // Données chargées
    projects: Vec<Project>,
    active_project_id: Option<String>,
    tasks: Vec<Task>,
    dependencies: Vec<TaskDependency>,
    members: Vec<UserMember>,
    current_schedule: Option<ScheduleResult>,

    // Navigation
    current_tab: NavigationTab,
    selected_task_id: Option<String>,

    // États des modales
    project_modal: ProjectModalState,
    task_modal: TaskModalState,
    error_alert: ErrorAlertState,

    open_task_modal_flag: bool,
}

impl PmApp {
    pub fn new(cc: &eframe::CreationContext<'_>, db: Arc<Mutex<Database>>) -> Self {
        let mut style = (*cc.egui_ctx.style()).clone();
        Theme::apply_to_style(&mut style);
        cc.egui_ctx.set_style(style);

        let scheduler = SchedulerService::new(db.clone());
        let mut app = Self {
            db,
            scheduler,
            projects: Vec::new(),
            active_project_id: None,
            tasks: Vec::new(),
            dependencies: Vec::new(),
            members: Vec::new(),
            current_schedule: None,
            current_tab: NavigationTab::Dashboard,
            selected_task_id: None,
            project_modal: ProjectModalState::default(),
            task_modal: TaskModalState::default(),
            error_alert: ErrorAlertState::default(),
            open_task_modal_flag: false,
        };

        app.refresh_all();

        if app.projects.is_empty() {
            app.create_sample_project();
        }

        app
    }

    fn refresh_all(&mut self) {
        let db = self.db.lock();
        self.projects = db.get_projects(false).unwrap_or_default();
        self.members = db.get_members().unwrap_or_default();

        if self.active_project_id.is_none() && !self.projects.is_empty() {
            self.active_project_id = Some(self.projects[0].id.clone());
        }

        if let Some(ref p_id) = self.active_project_id {
            self.tasks = db.get_tasks_by_project(p_id).unwrap_or_default();
            self.dependencies = db.get_dependencies_by_project(p_id).unwrap_or_default();
            drop(db);

            if let Ok(sched) = self.scheduler.recalculate_project_schedule(p_id) {
                self.current_schedule = Some(sched);
                let db = self.db.lock();
                self.tasks = db.get_tasks_by_project(p_id).unwrap_or_default();
            }
        }
    }

    fn create_sample_project(&mut self) {
        let today = Local::now().date_naive();
        let proj = Project {
            id: "prj-demo".to_string(),
            workspace_id: "ws-default".to_string(),
            key_prefix: "DEMO".to_string(),
            name: "Lancement Produit Alpha".to_string(),
            description: "Projet de démonstration avec chemin critique et dépendances".to_string(),
            status: "ACTIVE".to_string(),
            start_date: today,
            estimated_end_date: None,
            is_archived: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        let proj2 = Project {
            id: "prj-dupais".to_string(),
            workspace_id: "ws-default".to_string(),
            key_prefix: "AUDIT".to_string(),
            name: "Audit client Dupuis".to_string(),
            description: "Audit de conformité et sécurité".to_string(),
            status: "ACTIVE".to_string(),
            start_date: today,
            estimated_end_date: None,
            is_archived: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        let proj3 = Project {
            id: "prj-rise".to_string(),
            workspace_id: "ws-default".to_string(),
            key_prefix: "B12".to_string(),
            name: "B12 Rise Up 2026".to_string(),
            description: "Campagne de scaling".to_string(),
            status: "ACTIVE".to_string(),
            start_date: today,
            estimated_end_date: None,
            is_archived: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        let t1 = Task {
            id: "t-1".to_string(),
            project_id: "prj-demo".to_string(),
            assignee_id: Some("user-1".to_string()),
            title: "Cadrage & Spécifications".to_string(),
            description: "Définition des exigences fonctionnelles".to_string(),
            status: TaskStatus::Done,
            priority: TaskPriority::High,
            duration_hours: 16,
            start_date: today,
            end_date: today,
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

        let t2 = Task {
            id: "t-2".to_string(),
            project_id: "prj-demo".to_string(),
            assignee_id: Some("user-2".to_string()),
            title: "Développement Core Engine".to_string(),
            description: "Implémentation du graphe DAG et CPM".to_string(),
            status: TaskStatus::InProgress,
            priority: TaskPriority::Urgent,
            duration_hours: 32,
            start_date: today,
            end_date: today,
            early_start: None,
            early_finish: None,
            late_start: None,
            late_finish: None,
            is_critical: false,
            position_order: 1,
            is_deleted: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        let t3 = Task {
            id: "t-3".to_string(),
            project_id: "prj-demo".to_string(),
            assignee_id: Some("user-3".to_string()),
            title: "Interface Graphique Gantt".to_string(),
            description: "Rendu vectoriel des dépendances en courbes de Bézier".to_string(),
            status: TaskStatus::Todo,
            priority: TaskPriority::High,
            duration_hours: 24,
            start_date: today,
            end_date: today,
            early_start: None,
            early_finish: None,
            late_start: None,
            late_finish: None,
            is_critical: false,
            position_order: 2,
            is_deleted: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        let t4 = Task {
            id: "t-4".to_string(),
            project_id: "prj-demo".to_string(),
            assignee_id: Some("user-1".to_string()),
            title: "Recette & Déploiement".to_string(),
            description: "Validation globale et packaging".to_string(),
            status: TaskStatus::Todo,
            priority: TaskPriority::Normal,
            duration_hours: 16,
            start_date: today,
            end_date: today,
            early_start: None,
            early_finish: None,
            late_start: None,
            late_finish: None,
            is_critical: false,
            position_order: 3,
            is_deleted: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        let dep1 = TaskDependency {
            id: "dep-1".to_string(),
            project_id: "prj-demo".to_string(),
            predecessor_task_id: "t-1".to_string(),
            successor_task_id: "t-2".to_string(),
            dependency_type: DependencyType::FinishToStart,
            lag_days: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let dep2 = TaskDependency {
            id: "dep-2".to_string(),
            project_id: "prj-demo".to_string(),
            predecessor_task_id: "t-2".to_string(),
            successor_task_id: "t-3".to_string(),
            dependency_type: DependencyType::FinishToStart,
            lag_days: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let dep3 = TaskDependency {
            id: "dep-3".to_string(),
            project_id: "prj-demo".to_string(),
            predecessor_task_id: "t-3".to_string(),
            successor_task_id: "t-4".to_string(),
            dependency_type: DependencyType::FinishToStart,
            lag_days: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        {
            let db = self.db.lock();
            let _ = db.insert_project(&proj);
            let _ = db.insert_project(&proj2);
            let _ = db.insert_project(&proj3);
            let _ = db.insert_task(&t1);
            let _ = db.insert_task(&t2);
            let _ = db.insert_task(&t3);
            let _ = db.insert_task(&t4);
            let _ = db.insert_dependency(&dep1);
            let _ = db.insert_dependency(&dep2);
            let _ = db.insert_dependency(&dep3);
        }

        self.active_project_id = Some("prj-demo".to_string());
        self.refresh_all();
    }
}

impl eframe::App for PmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut save_project_opt = None;
        let mut save_task_opt = None;
        let mut add_dependency_opt = None;
        let mut delete_task_opt = None;
        let mut status_change_opt = None;
        let mut select_project_opt = None;

        // 1. SIDEBAR NAVIGATION GAUCHE (Style Bleu Corporate Gouti)
        egui::SidePanel::left("sidebar_panel")
            .exact_width(220.0)
            .frame(Frame::none().fill(Theme::SIDEBAR_BG).inner_margin(Margin::same(12.0)))
            .show(ctx, |ui| {
                // En-tête Logo
                ui.horizontal(|ui| {
                    // Icône 4 points
                    let (rect, _) = ui.allocate_exact_size(Vec2::new(18.0, 18.0), egui::Sense::hover());
                    let p = rect.min;
                    ui.painter().circle_filled(egui::Pos2::new(p.x + 4.0, p.y + 4.0), 3.0, Color32::WHITE);
                    ui.painter().circle_filled(egui::Pos2::new(p.x + 14.0, p.y + 4.0), 3.0, Color32::WHITE);
                    ui.painter().circle_filled(egui::Pos2::new(p.x + 4.0, p.y + 14.0), 3.0, Color32::WHITE);
                    ui.painter().circle_filled(egui::Pos2::new(p.x + 14.0, p.y + 14.0), 3.0, Color32::WHITE);

                    ui.add_space(8.0);
                    ui.heading(egui::RichText::new("Gouti / PM").color(Theme::SIDEBAR_TEXT).size(18.0).strong());
                });

                ui.add_space(16.0);

                // Menu Général
                render_sidebar_nav_item(ui, "💼 Mon travail", self.current_tab == NavigationTab::Dashboard, || {
                    self.current_tab = NavigationTab::Dashboard;
                });
                render_sidebar_nav_item(ui, "📁 Portefeuille projets", self.current_tab == NavigationTab::Portfolio, || {
                    self.current_tab = NavigationTab::Portfolio;
                });
                render_sidebar_nav_item(ui, "📋 Mes projets", false, || {
                    self.current_tab = NavigationTab::Dashboard;
                });
                render_sidebar_nav_item(ui, "⚙️ Gestion", self.current_tab == NavigationTab::Members, || {
                    self.current_tab = NavigationTab::Members;
                });
                render_sidebar_nav_item(ui, "🎓 Base de connaissance", self.current_tab == NavigationTab::KnowledgeBase, || {
                    self.current_tab = NavigationTab::KnowledgeBase;
                });
                render_sidebar_nav_item(ui, "⏱️ Feuille de temps", self.current_tab == NavigationTab::Timesheet, || {
                    self.current_tab = NavigationTab::Timesheet;
                });

                ui.add_space(14.0);
                ui.separator();
                ui.add_space(8.0);

                // Section Projet Actif
                let active_name = self.projects.iter().find(|p| Some(&p.id) == self.active_project_id.as_ref())
                    .map(|p| format!("[{}] {}", p.key_prefix, p.name))
                    .unwrap_or_else(|| "Aucun projet".to_string());

                // Encadré de sélection de projet actif
                Frame::none()
                    .fill(Theme::SIDEBAR_DARK)
                    .stroke(Stroke::new(1.0, Color32::from_white_alpha(40)))
                    .rounding(Rounding::same(4.0))
                    .inner_margin(Margin::symmetric(10.0, 7.0))
                    .show(ui, |ui| {
                        egui::ComboBox::from_id_salt("sidebar_project_combo")
                            .selected_text(egui::RichText::new(&active_name).color(Color32::WHITE).size(11.5))
                            .width(170.0)
                            .show_ui(ui, |ui| {
                                for p in &self.projects {
                                    let is_cur = self.active_project_id.as_deref() == Some(&p.id);
                                    let label = format!("[{}] {}", p.key_prefix, p.name);
                                    if ui.selectable_label(is_cur, label).clicked() {
                                        select_project_opt = Some(p.id.clone());
                                    }
                                }
                            });
                    });

                ui.add_space(10.0);

                // Sous-menus du projet actif
                render_sidebar_sub_item(ui, "🤝 Cadrage", false, || {});
                render_sidebar_sub_item(ui, "📈 Progression & contrôle", false, || {});
                render_sidebar_sub_item(ui, "📊 Planning (Gantt)", self.current_tab == NavigationTab::GanttPlanning, || {
                    self.current_tab = NavigationTab::GanttPlanning;
                });
                render_sidebar_sub_item(ui, "📋 Kanban", self.current_tab == NavigationTab::Kanban, || {
                    self.current_tab = NavigationTab::Kanban;
                });
                render_sidebar_sub_item(ui, "📅 Calendrier", self.current_tab == NavigationTab::Calendar, || {
                    self.current_tab = NavigationTab::Calendar;
                });
                render_sidebar_sub_item(ui, "📑 Rapports", false, || {});
            });

        // 2. TOP BAR HEADER (Blanc épuré avec boutons d'actions rapides et profil)
        egui::TopBottomPanel::top("top_panel")
            .frame(Frame::none().fill(Theme::PANEL_BG).stroke(Stroke::new(1.0, Theme::BORDER)).inner_margin(Margin::symmetric(16.0, 8.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Icône de réduction sidebar & raccourcis icônes
                    ui.label(egui::RichText::new("◀").color(Theme::SIDEBAR_BG).size(14.0));
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("💼 📁 📑 📅 🏷️").size(13.0));
                    ui.add_space(6.0);
                    ui.strong(egui::RichText::new("Mon travail").color(Theme::TEXT_TITLE).size(13.5));

                    // Actions rapides cartes blanches en haut à droite
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Drapeau & Profil
                        ui.label(egui::RichText::new("🇫🇷").size(16.0));
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("👤 Chloé ▼").color(Theme::TEXT_TITLE).size(12.0));
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new("✉️").size(14.0));
                        
                        // Badge notification rouge "10"
                        Frame::none()
                            .fill(Theme::HEADER_RED)
                            .rounding(Rounding::same(10.0))
                            .inner_margin(Margin::symmetric(5.0, 1.0))
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("10").color(Color32::WHITE).size(10.0));
                            });

                        ui.add_space(16.0);

                        // Boutons cartes d'actions rapides
                        render_top_action_card(ui, "📅 Mon calendrier", || {
                            self.current_tab = NavigationTab::Calendar;
                        });
                        render_top_action_card(ui, "🗂️ Mes tâches", || {
                            self.current_tab = NavigationTab::Kanban;
                        });
                        render_top_action_card(ui, "📑 Rapport d'activités", || {});
                        
                        // Bouton Nouveau Projet
                        if render_top_action_card(ui, "📄 Nouveau projet", || {}) {
                            self.project_modal = ProjectModalState {
                                is_open: true,
                                is_editing: false,
                                project_id: String::new(),
                                key_prefix: String::new(),
                                name: String::new(),
                                description: String::new(),
                                start_date_str: Local::now().date_naive().format("%Y-%m-%d").to_string(),
                            };
                        }
                    });
                });
            });

        // 3. ZONE CENTRALE DE TRAVAIL (Gris clair de fond)
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Theme::BG_MAIN).inner_margin(Margin::same(16.0)))
            .show(ctx, |ui| {
                let active_project = self.projects.iter().find(|p| Some(&p.id) == self.active_project_id.as_ref());
                let start_date = active_project.map(|p| p.start_date).unwrap_or_else(|| Local::now().date_naive());

                match self.current_tab {
                    NavigationTab::Dashboard | NavigationTab::Portfolio => {
                        DashboardView::render(
                            ui,
                            &self.projects,
                            &self.tasks,
                            &mut self.active_project_id,
                            &mut select_project_opt,
                            &mut self.open_task_modal_flag,
                            &mut self.selected_task_id,
                        );
                    }
                    NavigationTab::GanttPlanning => {
                        ui.horizontal(|ui| {
                            ui.heading(egui::RichText::new("📊 Planning Gantt & Dépendances").color(Theme::TEXT_TITLE));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(egui::RichText::new("➕ Nouvelle Tâche").color(Color32::WHITE)).clicked() {
                                    self.task_modal = TaskModalState {
                                        is_open: true,
                                        is_editing: false,
                                        task_id: String::new(),
                                        title: String::new(),
                                        description: String::new(),
                                        duration_hours: 8,
                                        priority: TaskPriority::Normal,
                                        status: TaskStatus::Todo,
                                        assignee_id: None,
                                        selected_dependency_pred_id: String::new(),
                                    };
                                }
                            });
                        });
                        ui.add_space(8.0);
                        GanttView::render(
                            ui,
                            start_date,
                            &self.tasks,
                            &self.dependencies,
                            self.current_schedule.as_ref(),
                            &mut self.selected_task_id,
                            &mut self.open_task_modal_flag,
                        );
                    }
                    NavigationTab::Kanban => {
                        KanbanView::render(
                            ui,
                            &self.tasks,
                            &mut self.selected_task_id,
                            &mut status_change_opt,
                            &mut self.open_task_modal_flag,
                        );
                    }
                    NavigationTab::Calendar => {
                        CalendarView::render(
                            ui,
                            start_date,
                            &self.tasks,
                            &mut self.selected_task_id,
                            &mut self.open_task_modal_flag,
                        );
                    }
                    NavigationTab::Members | NavigationTab::Timesheet | NavigationTab::KnowledgeBase => {
                        ui.heading(egui::RichText::new("👥 Gestion des Membres & Équipe").color(Theme::TEXT_TITLE));
                        ui.label(egui::RichText::new("Gestion des utilisateurs, droits et disponibilités.").color(Theme::TEXT_MUTED));
                        ui.add_space(14.0);
                        for m in &self.members {
                            Frame::none()
                                .fill(Theme::PANEL_BG)
                                .stroke(Stroke::new(1.0, Theme::BORDER))
                                .rounding(Rounding::same(6.0))
                                .inner_margin(Margin::symmetric(14.0, 10.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.strong(egui::RichText::new(&m.full_name).color(Theme::TEXT_TITLE));
                                        ui.label(egui::RichText::new(format!("({})", m.email)).color(Theme::TEXT_MUTED));
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.colored_label(Theme::SIDEBAR_BG, &m.role);
                                        });
                                    });
                                });
                            ui.add_space(6.0);
                        }
                    }
                }
            });

        // Double-clic déclenché depuis une vue
        if self.open_task_modal_flag {
            self.open_task_modal_flag = false;
            if let Some(ref tid) = self.selected_task_id {
                if let Some(t) = self.tasks.iter().find(|t| &t.id == tid) {
                    self.task_modal = TaskModalState {
                        is_open: true,
                        is_editing: true,
                        task_id: t.id.clone(),
                        title: t.title.clone(),
                        description: t.description.clone(),
                        duration_hours: t.duration_hours,
                        priority: t.priority,
                        status: t.status,
                        assignee_id: t.assignee_id.clone(),
                        selected_dependency_pred_id: String::new(),
                    };
                }
            }
        }

        // 4. Modales
        Modals::render_project_modal(ctx, &mut self.project_modal, &mut save_project_opt);

        let cur_proj_id = self.active_project_id.clone().unwrap_or_default();
        let cur_proj_start = self.projects.iter().find(|p| p.id == cur_proj_id).map(|p| p.start_date).unwrap_or_else(|| Local::now().date_naive());

        Modals::render_task_modal(
            ctx,
            &mut self.task_modal,
            &cur_proj_id,
            cur_proj_start,
            &self.members,
            &self.tasks,
            &mut save_task_opt,
            &mut add_dependency_opt,
            &mut delete_task_opt,
        );

        Modals::render_error_modal(ctx, &mut self.error_alert);

        // 5. Traitement des Actions Utilisateur & Recalcul
        if let Some(p) = save_project_opt {
            let db = self.db.lock();
            let _ = db.insert_project(&p);
            drop(db);
            self.active_project_id = Some(p.id.clone());
            self.refresh_all();
        }

        if let Some(t) = save_task_opt {
            let db = self.db.lock();
            if self.tasks.iter().any(|existing| existing.id == t.id) {
                let _ = db.update_task(&t);
            } else {
                let _ = db.insert_task(&t);
            }
            drop(db);
            self.refresh_all();
        }

        if let Some((pred, succ)) = add_dependency_opt {
            if let Some(ref pid) = self.active_project_id {
                match self.scheduler.add_dependency(pid, &pred, &succ) {
                    Ok(_) => self.refresh_all(),
                    Err(e) => {
                        self.error_alert = ErrorAlertState {
                            is_open: true,
                            title: "Conflit de Planification / Dépendance".to_string(),
                            message: e.to_string(),
                        };
                    }
                }
            }
        }

        if let Some(pid) = select_project_opt {
            self.active_project_id = Some(pid);
            self.selected_task_id = None;
            self.refresh_all();
        }

        if let Some(tid) = delete_task_opt {
            let db = self.db.lock();
            let _ = db.soft_delete_task(&tid);
            drop(db);
            self.selected_task_id = None;
            self.refresh_all();
        }

        if let Some((task_id, new_status)) = status_change_opt {
            if let Some(mut t) = self.tasks.iter().find(|t| t.id == task_id).cloned() {
                t.status = new_status;
                let db = self.db.lock();
                let _ = db.update_task(&t);
                drop(db);
                self.refresh_all();
            }
        }
    }
}

fn render_sidebar_nav_item<F: FnOnce()>(ui: &mut egui::Ui, label: &str, is_active: bool, on_click: F) {
    let (bg, txt_color) = if is_active {
        (Theme::SIDEBAR_DARK, Color32::WHITE)
    } else {
        (Color32::TRANSPARENT, Theme::SIDEBAR_MUTED)
    };

    let btn = egui::Button::new(egui::RichText::new(label).color(txt_color).size(12.5))
        .fill(bg)
        .rounding(Rounding::same(4.0));

    if ui.add(btn).clicked() {
        on_click();
    }
    ui.add_space(2.0);
}

fn render_sidebar_sub_item<F: FnOnce()>(ui: &mut egui::Ui, label: &str, is_active: bool, on_click: F) {
    let (bg, txt_color) = if is_active {
        (Theme::SIDEBAR_DARK, Color32::WHITE)
    } else {
        (Color32::TRANSPARENT, Theme::SIDEBAR_MUTED)
    };

    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let btn = egui::Button::new(egui::RichText::new(format!("> {}", label)).color(txt_color).size(11.5))
            .fill(bg)
            .rounding(Rounding::same(4.0));

        if ui.add(btn).clicked() {
            on_click();
        }
    });
    ui.add_space(2.0);
}

fn render_top_action_card<F: FnOnce()>(ui: &mut egui::Ui, label: &str, on_click: F) -> bool {
    let btn = egui::Button::new(egui::RichText::new(label).color(Theme::TEXT_TITLE).size(11.5))
        .fill(Theme::PANEL_BG)
        .stroke(Stroke::new(1.0, Theme::BORDER))
        .rounding(Rounding::same(4.0));

    if ui.add(btn).clicked() {
        on_click();
        true
    } else {
        false
    }
}
