use crate::page::Page;
use eframe::egui;
use eframe::epaint::text::{FontInsert, InsertFontFamily};
use eframe::icon_data;

pub fn load_icon_data() -> egui::IconData {
    icon_data::from_png_bytes(&include_bytes!("./ui/data/cat.png")[..]).unwrap()
}

pub fn add_font(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        "my_font",
        egui::FontData::from_static(include_bytes!("./font/STSong.ttf")),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Lowest,
            },
        ],
    ));
}

pub fn show_top_menu(ui: &mut egui::Ui, current_page: &mut Page) {
    ui.horizontal(|ui| {
        egui::widgets::global_theme_preference_switch(ui);
        ui.separator();
        ui.selectable_value(current_page, Page::Home, "主页");
        ui.selectable_value(current_page, Page::Slave, "从机");
        ui.selectable_value(current_page, Page::Master, "主机");
    });
}
