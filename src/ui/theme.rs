use egui::{epaint::Shadow, Color32, Margin, Rounding, Stroke, Style, Vec2, Visuals};

pub struct Theme;

#[allow(dead_code)]
impl Theme {
    // --- PALETTE PRINCIPALE INSPIRÉE DU DESIGN CORPORATE (Gouti/PM) ---
    // Sidebar Bleue
    pub const SIDEBAR_BG: Color32 = Color32::from_rgb(30, 107, 184);       // #1e6bb8 - Bleu Royal Sidebar
    pub const SIDEBAR_DARK: Color32 = Color32::from_rgb(22, 90, 158);      // #165a9e - Bleu foncé hover/actif
    pub const SIDEBAR_TEXT: Color32 = Color32::from_rgb(255, 255, 255);     // Blanc pur
    pub const SIDEBAR_MUTED: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 210);

    // Fond Principal et Cartes (Thème Clair Moderne Haute Lisibilité)
    pub const BG_MAIN: Color32 = Color32::from_rgb(240, 243, 248);         // #f0f3f8 - Fond de travail clair
    pub const PANEL_BG: Color32 = Color32::from_rgb(255, 255, 255);        // #ffffff - Cartes & Panneaux blancs
    pub const CARD_BG: Color32 = Color32::from_rgb(255, 255, 255);
    pub const CARD_HOVER: Color32 = Color32::from_rgb(245, 248, 252);
    pub const HEADER_BG: Color32 = Color32::from_rgb(248, 250, 253);

    // En-têtes Spécifiques des Cartouches (Style Gouti)
    pub const HEADER_GREEN: Color32 = Color32::from_rgb(46, 155, 72);       // #2e9b48 - Vert "Projets actifs" & "Jalons"
    pub const HEADER_RED: Color32 = Color32::from_rgb(211, 30, 42);         // #d31e2a - Rouge "Mes problèmes"
    pub const HEADER_BLUE: Color32 = Color32::from_rgb(33, 117, 201);       // #2175c9 - Bleu standard

    // Badges de Statuts
    pub const STATUS_VALIDATION: Color32 = Color32::from_rgb(27, 73, 130);   // Bleu nuit "En cours de validation"
    pub const STATUS_LAUNCH: Color32 = Color32::from_rgb(230, 126, 34);      // Orange "Demande de lancement"
    pub const STATUS_INIT: Color32 = Color32::from_rgb(75, 163, 227);        // Bleu ciel "En phase d'initialisation"
    pub const STATUS_PROGRESS: Color32 = Color32::from_rgb(41, 128, 185);    // Bleu royal "En cours de réalisation"
    pub const STATUS_DONE: Color32 = Color32::from_rgb(46, 204, 113);        // Vert "Terminé"
    pub const STATUS_BLOCKED: Color32 = Color32::from_rgb(231, 76, 60);      // Rouge "Bloqué"

    // Filtres Pilules
    pub const PILL_BLUE: Color32 = Color32::from_rgb(33, 117, 201);         // #2175c9
    pub const PILL_GREEN: Color32 = Color32::from_rgb(39, 174, 96);         // #27ae60
    pub const PILL_BURGUNDY: Color32 = Color32::from_rgb(142, 40, 68);      // #8e2844

    // Accents & Criticité
    pub const ACCENT_PRIMARY: Color32 = Color32::from_rgb(30, 107, 184);
    pub const ACCENT_HOVER: Color32 = Color32::from_rgb(45, 128, 212);
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(6, 182, 212);
    pub const CRITICAL_PATH: Color32 = Color32::from_rgb(211, 30, 42);      // Rouge critique
    pub const CRITICAL_GLOW: Color32 = Color32::from_rgb(245, 108, 118);
    pub const SUCCESS: Color32 = Color32::from_rgb(46, 155, 72);
    pub const WARNING: Color32 = Color32::from_rgb(243, 156, 18);

    // Typographie
    pub const TEXT_TITLE: Color32 = Color32::from_rgb(33, 43, 54);          // #212b36 - Noir charbon doux
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(51, 65, 85);        // #334155 - Texte principal lisible
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(100, 116, 139);       // #64748b - Sous-titres
    pub const TEXT_SUBTLE: Color32 = Color32::from_rgb(148, 163, 184);      // #94a3b8 - Métadonnées

    // Bordures
    pub const BORDER: Color32 = Color32::from_rgb(220, 227, 236);           // #dce3ec - Bordure claire
    pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(238, 242, 246);

    // Dépendances Gantt
    pub const DEP_ARROW_NORMAL: Color32 = Color32::from_rgb(30, 107, 184);
    pub const DEP_ARROW_CRITICAL: Color32 = Color32::from_rgb(211, 30, 42);

    /// Applique le thème clair / corporate professionnel
    pub fn apply_to_style(style: &mut Style) {
        let mut visuals = Visuals::light();
        visuals.panel_fill = Self::BG_MAIN;
        visuals.window_fill = Self::PANEL_BG;
        visuals.window_stroke = Stroke::new(1.0, Self::BORDER);
        visuals.window_rounding = Rounding::same(8.0);
        visuals.window_shadow = Shadow {
            offset: Vec2::new(0.0, 6.0),
            blur: 18.0,
            spread: 0.0,
            color: Color32::from_black_alpha(35),
        };

        visuals.widgets.noninteractive.bg_fill = Self::PANEL_BG;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Self::TEXT_PRIMARY);
        visuals.widgets.noninteractive.rounding = Rounding::same(5.0);

        visuals.widgets.inactive.bg_fill = Self::PANEL_BG;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Self::TEXT_PRIMARY);
        visuals.widgets.inactive.rounding = Rounding::same(5.0);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Self::BORDER);

        visuals.widgets.hovered.bg_fill = Self::CARD_HOVER;
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Self::TEXT_TITLE);
        visuals.widgets.hovered.rounding = Rounding::same(5.0);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Self::ACCENT_PRIMARY);

        visuals.widgets.active.bg_fill = Self::ACCENT_PRIMARY;
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.active.rounding = Rounding::same(5.0);

        visuals.selection.bg_fill = Self::ACCENT_PRIMARY.gamma_multiply(0.2);
        visuals.selection.stroke = Stroke::new(1.0, Self::ACCENT_PRIMARY);

        style.visuals = visuals;

        style.spacing.item_spacing = Vec2::new(8.0, 6.0);
        style.spacing.button_padding = Vec2::new(10.0, 6.0);
        style.spacing.window_margin = Margin::same(14.0);
    }
}
