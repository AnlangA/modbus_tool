use eframe::epaint::text::{FontInsert, InsertFontFamily};
use eframe::{App, egui, icon_data};

pub struct ModbusTool {
    name: String,
    age: u32,
}

impl ModbusTool {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        add_font(&cc.egui_ctx);
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl App for ModbusTool {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("你好");
            ui.label(format!("Name: {}", self.name));
            ui.label(format!("Age: {}", self.age));
        });
    }
}

pub fn load_icon_data() -> egui::IconData {
    icon_data::from_png_bytes(&include_bytes!("./ui/data/cat.png")[..]).unwrap()
}

fn add_font(ctx: &egui::Context) {
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
