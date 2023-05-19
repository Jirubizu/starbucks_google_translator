use eframe::egui::Vec2;
use translator::ui::TranslateUi;

mod translator;

#[tokio::main]
async fn main() {
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(Vec2::new(960.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Translator",
        options,
        Box::new(|cc| Box::new(TranslateUi::new(cc))),
    )
    .expect("failed?");
}
