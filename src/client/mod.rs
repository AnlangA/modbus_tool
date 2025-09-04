use eframe::*;

pub struct Client {}

impl Default for Client {
    fn default() -> Self {
        Client {}
    }
}

impl Client {
    pub fn show(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(_ctx, |ui| {
            ui.label("Modbus 主机界面");
        });
    }
}
