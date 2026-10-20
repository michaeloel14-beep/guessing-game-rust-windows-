pub mod game;
pub mod ui;

#[cfg(target_os = "android")]
use game::GuessingGame;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.android_app = Some(app);
    let _ = eframe::run_native(
        "Gæt et tal",
        options,
        Box::new(|_cc| Ok(Box::new(GuessingGame::default()))),
    );
}
