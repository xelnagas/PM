mod domain;
mod services;
mod storage;
mod ui;

use parking_lot::Mutex;
use std::sync::Arc;
use storage::Database;
use ui::PmApp;

fn main() -> eframe::Result<()> {
    env_logger::init();

    // Emplacement de la base SQLite locale
    let db_path = "pm_data.db";
    let db = Database::open(db_path).expect("Impossible d'initialiser la base de données SQLite");
    let db_arc = Arc::new(Mutex::new(db));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("🦀 PM - Gestion de Projets & Planification Temporelle"),
        ..Default::default()
    };

    eframe::run_native(
        "PM Desktop",
        options,
        Box::new(|cc| Ok(Box::new(PmApp::new(cc, db_arc)))),
    )
}
