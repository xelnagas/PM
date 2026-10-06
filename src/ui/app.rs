use chrono::Local;
use eframe::egui;
use egui::{Rounding, Stroke};
use parking_lot::Mutex;
use std::sync::Arc;

use crate::domain::models::*;
use crate::services::SchedulerService;
use crate::storage::Database;
use crate::ui::calendar_view::CalendarView;
use crate::ui::gantt_view::GanttView;
use crate::ui::kanban_view::KanbanView;
use crate::ui::modals::*;
use crate::ui::theme::Theme;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ViewMode {
    Gantt,
    Kanban,
    Calendar,
    Members,
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

    // Navigation & Vues
    current_view: ViewMode,
    selected_task_id: Option<String>,

    // États des modales
    project_modal: ProjectModalState,
    task_modal: TaskModalState,
    error_alert: ErrorAlertState,

    // Flag pour double-clic
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
            current_view: ViewMode::Gantt,
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
            description: "Implémentation du graphe DAG et de la méthode CPM".to_string(),
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
        let mut switch_to_project_id = None;

        // 1. Barre Supérieure : Navigation & Onglets Multi-Projets
        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::none()
                .fill(Theme::PANEL_BG)
                .stroke(Stroke::new(1.0, Theme::BORDER))
                .inner_margin(egui::Margin::symmetric(16.0, 12.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Logo & Marque
                    egui::Frame::none()
                        .fill(Theme::ACCENT_PRIMARY)
                        .rounding(Rounding::same(6.0))
                        .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("🦀 PM").color(egui::Color32::WHITE).strong().size(13.0));
                        });

                    ui.add_space(8.0);

                    // Onglets en pilules
                    for p in &self.projects {
                        let is_active = self.active_project_id.as_deref() == Some(&p.id);
                        let tab_label = format!("📁 [{}] {}", p.key_prefix, p.name);
                        
                        let (bg, text_color, stroke) = if is_active {
                            (Theme::CARD_BG, Theme::TEXT_TITLE, Stroke::new(1.0, Theme::ACCENT_PRIMARY))
                        } else {
                            (egui::Color32::TRANSPARENT, Theme::TEXT_MUTED, Stroke::new(1.0, Theme::BORDER_SUBTLE))
                        };

                        let btn = egui::Button::new(egui::RichText::new(tab_label).color(text_color).size(12.0))
                            .fill(bg)
                            .stroke(stroke);

                        if ui.add(btn).clicked() {
                            switch_to_project_id = Some(p.id.clone());
                        }
                    }

                    if ui.button(egui::RichText::new("+ Nouveau Projet").color(Theme::ACCENT_CYAN).size(12.0)).clicked() {
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

                    // Indicateur de Date de Fin Estimée (Style Badge Élégant)
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if let Some(ref sched) = self.current_schedule {
                            egui::Frame::none()
                                .fill(Theme::BADGE_BG)
                                .stroke(Stroke::new(1.0, Theme::ACCENT_PRIMARY.gamma_multiply(0.5)))
                                .rounding(Rounding::same(6.0))
                                .inner_margin(egui::Margin::symmetric(10.0, 5.0))
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new(format!("⏳ Fin Estimée : {} ({}j)", sched.estimated_end_date.format("%d/%m/%Y"), sched.total_duration_days))
                                        .color(Theme::ACCENT_HOVER)
                                        .strong()
                                        .size(12.0));
                                });
                        }

                        if self.active_project_id.is_some() {
                            let add_task_btn = egui::Button::new(egui::RichText::new("+ Nouvelle Tâche").color(egui::Color32::WHITE).strong().size(12.0))
                                .fill(Theme::ACCENT_PRIMARY);

                            if ui.add(add_task_btn).clicked() {
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
                        }
                    });
                });

                ui.add_space(8.0);

                // Sous-barre : Segmented Control pour le Sélecteur de Vues
                ui.horizontal(|ui| {
                    egui::Frame::none()
                        .fill(Theme::BG_BASE)
                        .stroke(Stroke::new(1.0, Theme::BORDER))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(egui::Margin::symmetric(4.0, 3.0))
                        .show(ui, |ui| {
                            let views = [
                                (ViewMode::Gantt, "📊 Gantt & Dépendances"),
                                (ViewMode::Kanban, "📋 Tableau Kanban"),
                                (ViewMode::Calendar, "📅 Planning"),
                                (ViewMode::Members, "👥 Intervenants"),
                            ];

                            for (mode, label) in views {
                                let is_active = self.current_view == mode;
                                let bg = if is_active { Theme::CARD_BG } else { egui::Color32::TRANSPARENT };
                                let text_color = if is_active { Theme::TEXT_TITLE } else { Theme::TEXT_MUTED };

                                let btn = egui::Button::new(egui::RichText::new(label).color(text_color).size(12.0))
                                    .fill(bg);

                                if ui.add(btn).clicked() {
                                    self.current_view = mode;
                                }
                            }
                        });

                    if let Some(ref task_id) = self.selected_task_id {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(egui::RichText::new("✏️ Modifier Tâche").size(12.0)).clicked() {
                                if let Some(t) = self.tasks.iter().find(|t| &t.id == task_id) {
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
                        });
                    }
                });
            });

        if let Some(pid) = switch_to_project_id {
            self.active_project_id = Some(pid);
            self.selected_task_id = None;
            self.refresh_all();
        }

        // 2. Zone Principale d'Affichage selon la Vue Active
        egui::CentralPanel::default().show(ctx, |ui| {
            let active_project = self.projects.iter().find(|p| Some(&p.id) == self.active_project_id.as_ref());
            let start_date = active_project.map(|p| p.start_date).unwrap_or_else(|| Local::now().date_naive());

            match self.current_view {
                ViewMode::Gantt => {
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
                ViewMode::Kanban => {
                    KanbanView::render(
                        ui,
                        &self.tasks,
                        &mut self.selected_task_id,
                        &mut status_change_opt,
                        &mut self.open_task_modal_flag,
                    );
                }
                ViewMode::Calendar => {
                    CalendarView::render(
                        ui,
                        start_date,
                        &self.tasks,
                        &mut self.selected_task_id,
                        &mut self.open_task_modal_flag,
                    );
                }
                ViewMode::Members => {
                    ui.add_space(8.0);
                    ui.heading(egui::RichText::new("👥 Intervenants & Équipe").color(Theme::TEXT_TITLE));
                    ui.label(egui::RichText::new("Gestion des membres du workspace et des affectations.").color(Theme::TEXT_MUTED));
                    ui.add_space(14.0);
                    for m in &self.members {
                        egui::Frame::none()
                            .fill(Theme::CARD_BG)
                            .stroke(Stroke::new(1.0, Theme::BORDER))
                            .rounding(Rounding::same(8.0))
                            .inner_margin(egui::Margin::symmetric(16.0, 10.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let initials: String = m.full_name.split_whitespace().filter_map(|w| w.chars().next()).collect();
                                    egui::Frame::none()
                                        .fill(Theme::ACCENT_PRIMARY.gamma_multiply(0.25))
                                        .rounding(Rounding::same(14.0))
                                        .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                        .show(ui, |ui| {
                                            ui.label(egui::RichText::new(initials).color(Theme::ACCENT_HOVER).strong().size(12.0));
                                        });

                                    ui.add_space(6.0);
                                    ui.strong(egui::RichText::new(&m.full_name).color(Theme::TEXT_TITLE));
                                    ui.label(egui::RichText::new(format!("({})", m.email)).color(Theme::TEXT_MUTED));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.colored_label(Theme::ACCENT_CYAN, &m.role);
                                    });
                                });
                            });
                        ui.add_space(6.0);
                    }
                }
            }
        });

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

        // 3. Rendu des Modales
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

        // 4. Traitement des Actions Utilisateur & Recalcul
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
