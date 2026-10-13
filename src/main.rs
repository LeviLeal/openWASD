mod app;
mod controller_diagram;
mod gamepad_types;
mod key_codes;
mod keyboard_interceptor;
mod mapping_manager;
mod virtual_gamepad;

use app::WasdApp;
use eframe::egui::{vec2, ViewportBuilder};
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let mut viewport = ViewportBuilder::default()
        .with_title("WASD Anywhere — Virtual Gamepad Mapper")
        .with_inner_size(vec2(1200.0, 780.0))
        .with_min_inner_size(vec2(950.0, 650.0));

    // Carregar ícone embutido do aplicativo
    if let Ok(icon_image) = image::load_from_memory(include_bytes!("../icon.png")) {
        let rgba = icon_image.to_rgba8();
        let (width, height) = rgba.dimensions();
        let icon_data = eframe::egui::IconData {
            rgba: rgba.into_raw(),
            width,
            height,
        };
        viewport = viewport.with_icon(Arc::new(icon_data));
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "WASD Anywhere",
        options,
        Box::new(|cc| Ok(Box::new(WasdApp::new(cc)))),
    )
}
