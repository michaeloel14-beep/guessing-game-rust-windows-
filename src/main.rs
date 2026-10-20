#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hides console window on Windows release builds

use guessing_game::game::GuessingGame;

fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([400.0, 650.0]); // Mobile-like window size on PC

    eframe::run_native(
        "Gæt et tal",
        options,
        Box::new(|_cc| Ok(Box::new(GuessingGame::default()))),
    )
}
