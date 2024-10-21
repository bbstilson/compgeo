use eframe::egui;

pub mod algorithms;
mod app;
pub mod color;
pub mod data;
mod graham_scan;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 1080.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CompGeo Viewer",
        options,
        Box::new(|cc| Ok(Box::new(WrapApp::new(cc)))),
    )
}

struct WrapApp {
    app: app::EguiApp,
}

impl WrapApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            app: app::EguiApp::default(),
        }
    }
}

impl eframe::App for WrapApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.app.update(ctx, frame);
    }
}
