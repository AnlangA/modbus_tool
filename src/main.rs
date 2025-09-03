#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use modbus_tool::home_page::*;

fn main() -> eframe::Result {
    env_logger::init();

    let icon_data = load_icon_data();
    let viewport = egui::ViewportBuilder::default()
        .with_icon(icon_data)
        .with_inner_size([900.0, 600.0]);

    let options = eframe::NativeOptions {
        viewport: viewport,
        ..Default::default()
    };
    eframe::run_native(
        "Modbus tool",
        options,
        Box::new(|cc| Ok(Box::new(ModbusTool::new(cc)))),
    )
}
