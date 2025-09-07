use crate::master::Master;
use crate::serial::SerialPort;
use crate::slave::Slave;
use eframe::epaint::text::{FontInsert, InsertFontFamily};
use eframe::{App, egui, icon_data};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Slave,
    Master,
}

impl Default for Page {
    fn default() -> Self {
        Page::Home
    }
}

#[derive(Default)]
pub struct ModbusTool {
    page: Page,
    serial: SerialPort,
    slave: Slave,
    master: Master,
}

impl ModbusTool {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        add_font(&cc.egui_ctx);
        Self::default()
    }
    fn home_page(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.serial.show(ctx, frame);
    }

    fn slave_page(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.slave.show(ctx, frame);
    }

    fn master_page(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.master.show(ctx, frame);
    }
}

impl App for ModbusTool {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 顶部菜单栏
        egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::widgets::global_theme_preference_switch(ui);
                ui.separator();
                ui.selectable_value(&mut self.page, Page::Home, "主页");
                ui.selectable_value(&mut self.page, Page::Slave, "从机");
                ui.selectable_value(&mut self.page, Page::Master, "主机");
            });
        });

        match self.page {
            Page::Home => self.home_page(ctx, frame),
            Page::Slave => self.slave_page(ctx, frame),
            Page::Master => self.master_page(ctx, frame),
        }
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

//创建顶部菜单
