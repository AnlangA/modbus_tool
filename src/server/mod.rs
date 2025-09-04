use eframe::*;

pub struct Server {}

impl Default for Server {
    fn default() -> Self {
        Server {}
    }
}

impl Server {
    pub fn show(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(_ctx, |ui| {
            ui.label("Modbus 从机界面");
        });
    }
}
