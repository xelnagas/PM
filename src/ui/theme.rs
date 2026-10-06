use egui::{epaint::Shadow, Color32, Margin, Rounding, Stroke, Style, Vec2, Visuals};

pub struct Theme;

#[allow(dead_code)]
impl Theme {
    // Arrière-plans profonds modernes (Style Dark Slate & Obsidian)
    pub const BG_BASE: Color32 = Color32::from_rgb(11, 15, 25);      // #0b0f19 - Fond de fenêtre
    pub const PANEL_BG: Color32 = Color32::from_rgb(17, 24, 39);     // #111827 - Barres & Panneaux
    pub const CARD_BG: Color32 = Color32::from_rgb(30, 41, 59);      // #1e293b - Cartes de tâches
    pub const CARD_HOVER: Color32 = Color32::from_rgb(45, 59, 82);   // #2d3b52 - Survol carte
    pub const HEADER_BG: Color32 = Color32::from_rgb(15, 23, 42);    // #0f172a - En-têtes

    // Accents & Statuts vibrants
    pub const ACCENT_PRIMARY: Color32 = Color32::from_rgb(99, 102, 241);   // #6366f1 - Indigo moderne
    pub const ACCENT_HOVER: Color32 = Color32::from_rgb(129, 140, 248);   // #818cf8
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(6, 182, 212);      // #06b6d4 - Cyan éclatant
    pub const CRITICAL_PATH: Color32 = Color32::from_rgb(244, 63, 94);     // #f43f5e - Rose/Rouge vif néon
    pub const CRITICAL_GLOW: Color32 = Color32::from_rgb(251, 113, 133);  // #fb7185
    pub const SUCCESS: Color32 = Color32::from_rgb(16, 185, 129);          // #10b981 - Émeraude
    pub const WARNING: Color32 = Color32::from_rgb(245, 158, 11);          // #f59e0b - Ambre doré
    pub const BADGE_BG: Color32 = Color32::from_rgb(30, 27, 75);          // Indigo foncé pour badge

    // Typographie & Hiérarchie de texte
    pub const TEXT_TITLE: Color32 = Color32::from_rgb(248, 250, 252);     // #f8fafc - Blanc éclatant
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(226, 232, 240);   // #e2e8f0 - Gris clair haute lisibilité
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(148, 163, 184);     // #94a3b8 - Gris moyen doux
    pub const TEXT_SUBTLE: Color32 = Color32::from_rgb(100, 116, 139);    // #64748b - Gris discret

    // Bordures & Délimitations ultra-fines
    pub const BORDER: Color32 = Color32::from_rgb(51, 65, 85);             // #334155 - Bordure nette
    pub const BORDER_SUBTLE: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 18);

    // Dépendances Gantt
    pub const DEP_ARROW_NORMAL: Color32 = Color32::from_rgb(96, 165, 250); // Bleu clair
    pub const DEP_ARROW_CRITICAL: Color32 = Color32::from_rgb(244, 63, 94); // Rouge néon

    /// Configure l'ensemble du style global de l'application
    pub fn apply_to_style(style: &mut Style) {
        let mut visuals = Visuals::dark();
        visuals.panel_fill = Self::BG_BASE;
        visuals.window_fill = Self::PANEL_BG;
        visuals.window_stroke = Stroke::new(1.0, Self::BORDER);
        visuals.window_rounding = Rounding::same(10.0);
        visuals.window_shadow = Shadow {
            offset: Vec2::new(0.0, 8.0),
            blur: 24.0,
            spread: 0.0,
            color: Color32::from_black_alpha(140),
        };

        // Boutons et contrôles interactifs
        visuals.widgets.noninteractive.bg_fill = Self::CARD_BG;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Self::TEXT_PRIMARY);
        visuals.widgets.noninteractive.rounding = Rounding::same(6.0);

        visuals.widgets.inactive.bg_fill = Self::CARD_BG;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Self::TEXT_PRIMARY);
        visuals.widgets.inactive.rounding = Rounding::same(6.0);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Self::BORDER);

        visuals.widgets.hovered.bg_fill = Self::CARD_HOVER;
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Self::TEXT_TITLE);
        visuals.widgets.hovered.rounding = Rounding::same(6.0);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Self::ACCENT_PRIMARY);

        visuals.widgets.active.bg_fill = Self::ACCENT_PRIMARY;
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.active.rounding = Rounding::same(6.0);

        visuals.selection.bg_fill = Self::ACCENT_PRIMARY.gamma_multiply(0.4);
        visuals.selection.stroke = Stroke::new(1.0, Self::ACCENT_PRIMARY);

        style.visuals = visuals;

        // Espacements aérés et confortables
        style.spacing.item_spacing = Vec2::new(10.0, 8.0);
        style.spacing.button_padding = Vec2::new(12.0, 7.0);
        style.spacing.window_margin = Margin::same(16.0);
    }
}
