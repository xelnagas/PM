use egui::Color32;

pub struct Theme;

#[allow(dead_code)]
impl Theme {
    pub const BG_DARK: Color32 = Color32::from_rgb(18, 20, 24);
    pub const PANEL_BG: Color32 = Color32::from_rgb(26, 29, 36);
    pub const CARD_BG: Color32 = Color32::from_rgb(34, 38, 48);
    pub const CARD_HOVER: Color32 = Color32::from_rgb(42, 47, 60);

    // Accents
    pub const ACCENT_PRIMARY: Color32 = Color32::from_rgb(59, 130, 246);
    pub const ACCENT_HOVER: Color32 = Color32::from_rgb(96, 165, 250);
    pub const CRITICAL_PATH: Color32 = Color32::from_rgb(239, 68, 68);
    pub const CRITICAL_GLOW: Color32 = Color32::from_rgb(248, 113, 113);
    pub const SUCCESS: Color32 = Color32::from_rgb(34, 197, 94);
    pub const WARNING: Color32 = Color32::from_rgb(234, 179, 8);

    // Texte
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(243, 244, 246);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(156, 163, 175);
    pub const BORDER: Color32 = Color32::from_rgb(55, 65, 81);

    // Dépendances Gantt
    pub const DEP_ARROW_NORMAL: Color32 = Color32::from_rgb(147, 197, 253);
    pub const DEP_ARROW_CRITICAL: Color32 = Color32::from_rgb(239, 68, 68);
}
