mod app;
mod controller_diagram;
mod gamepad_types;
mod key_codes;
mod keyboard_interceptor;
mod mapping_manager;
mod virtual_gamepad;

use app::OpenWasdApp;
use eframe::egui::{vec2, ViewportBuilder};
use std::sync::Arc;

fn ensure_desktop_integration() {
    if let Some(base_dirs) = directories::BaseDirs::new() {
        let data_dir = base_dirs.data_dir(); // ~/.local/share
        let apps_dir = data_dir.join("applications");
        let icons_dir = data_dir.join("icons");
        let hicolor_dir = data_dir.join("icons/hicolor/256x256/apps");

        let _ = std::fs::create_dir_all(&apps_dir);
        let _ = std::fs::create_dir_all(&icons_dir);
        let _ = std::fs::create_dir_all(&hicolor_dir);

        let icon_bytes = include_bytes!("../icon.png");
        let _ = std::fs::write(icons_dir.join("openwasd.png"), icon_bytes);
        let _ = std::fs::write(hicolor_dir.join("openwasd.png"), icon_bytes);

        // Remover atalho desktop legado se existir
        let _ = std::fs::remove_file(apps_dir.join("wasd-anywhere.desktop"));

        let desktop_path = apps_dir.join("openwasd.desktop");
        let current_exe = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "openwasd".to_string());

        let desktop_content = format!(
            "[Desktop Entry]\n\
            Name=openWASD\n\
            Comment=Virtual Gamepad Mapper for Keyboard\n\
            Exec=\"{}\"\n\
            Icon=openwasd\n\
            Terminal=false\n\
            Type=Application\n\
            Categories=Game;Utility;\n\
            Keywords=gamepad;controller;xbox;virtual;mapper;wasd;openwasd;\n\
            StartupWMClass=openwasd\n",
            current_exe
        );
        let _ = std::fs::write(desktop_path, desktop_content);
    }
}

fn main() -> eframe::Result<()> {
    // Garante que o ícone e o .desktop com app_id estejam registrados no KDE/GNOME
    ensure_desktop_integration();

    let mut viewport = ViewportBuilder::default()
        .with_app_id("openwasd")
        .with_title("openWASD — Virtual Gamepad Mapper")
        .with_inner_size(vec2(1200.0, 780.0))
        .with_min_inner_size(vec2(950.0, 650.0))
        .with_decorations(false)
        .with_transparent(true)
        .with_resizable(true);

    // Carregar ícone embutido do aplicativo (para X11 / fallback)
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
        "openwasd",
        options,
        Box::new(|cc| Ok(Box::new(OpenWasdApp::new(cc)))),
    )
}
