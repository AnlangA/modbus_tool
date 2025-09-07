use eframe::*;

pub struct Master {}

impl Default for Master {
    fn default() -> Self {
        Master {}
    }
}

impl Master {
    pub fn show(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(_ctx, |ui| {
            ui.label("Modbus 主机界面");
        });
    }
}
