use eframe::*;

#[derive(Debug, Default)]
pub struct Slave {}

impl Slave {
    pub fn show(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(_ctx, |ui| {
            ui.label("Modbus 从机界面");
        });
    }
}
