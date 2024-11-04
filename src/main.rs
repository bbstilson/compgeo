use eframe::egui;

pub mod algorithms;
mod app;
pub mod color;
pub mod data;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 1080.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CompGeo Viewer",
        options,
        Box::new(|_| Ok(Box::new(app::EguiApp::default()))),
    )
}
