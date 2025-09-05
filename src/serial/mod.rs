use eframe::*;
use std::time::Duration;
use tokio_serial::*;

pub struct SerialPort {
    list: Vec<(String, String)>,
    selected: String,
    settings: PortSettings,
}

pub struct PortSettings {
    /// The port name, usually the device path
    pub path: String,
    /// The baud rate in symbols-per-second
    pub baud_rate: u32,
    /// Number of bits used to represent a character sent on the line
    pub data_bits: DataBits,
    /// The type of signalling to use for controlling data transfer
    pub flow_control: FlowControl,
    /// The type of parity to use for error checking
    pub parity: Parity,
    /// Number of bits to use to signal the end of a character
    pub stop_bits: StopBits,
    /// Amount of time to wait to receive data before timing out
    pub timeout: Duration,
    /// The state to set DTR to when opening the device
    pub dtr_on_open: Option<bool>,
}

impl Default for PortSettings {
    fn default() -> Self {
        PortSettings {
            path: String::new(),
            baud_rate: 9600,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(100),
            dtr_on_open: None,
        }
    }
}

impl Default for SerialPort {
    fn default() -> Self {
        let mut port = SerialPort {
            list: Vec::new(),
            selected: String::new(),
            settings: PortSettings::default(),
        };
        port.list_ports();
        port.selected = port
            .list
            .first()
            .map(|(name, _)| name.clone())
            .unwrap_or_default();
        port
    }
}

impl SerialPort {
    pub fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("串口设置");
            ui.separator();

            egui::Grid::new("serial_settings_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    self.show_port_selector(ui);
                    self.show_baud_rate_selector(ui);
                    self.show_data_bits_selector(ui);
                    self.show_stop_bits_selector(ui);
                    self.show_parity_selector(ui);
                    self.show_flow_control_selector(ui);
                    self.show_timeout_input(ui);
                    self.show_dtr_checkbox(ui);
                });

            ui.add_space(10.0);
            self.show_connection_buttons(ui);
        });
    }

    fn show_port_selector(&mut self, ui: &mut egui::Ui) {
        ui.label("端口:");
        let combo_response = egui::ComboBox::from_id_salt("port_selector")
            .selected_text(&self.selected)
            .show_ui(ui, |ui| {
                for (port_name, port_info) in &self.list {
                    ui.selectable_value(
                        &mut self.selected,
                        port_name.clone(),
                        format!("{} - {}", port_name, port_info),
                    );
                }
            });

        if combo_response.response.clicked() {
            self.list_ports();
        }
        ui.end_row();
    }

    fn show_baud_rate_selector(&mut self, ui: &mut egui::Ui) {
        ui.label("波特率:");
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut self.settings.baud_rate).speed(100));
            ui.label("bps");
        });
        ui.end_row();
    }

    fn show_data_bits_selector(&mut self, ui: &mut egui::Ui) {
        ui.label("数据位:");
        egui::ComboBox::from_id_salt("data_bits_selector")
            .selected_text(format!("{:?}", self.settings.data_bits))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.settings.data_bits, DataBits::Five, "5");
                ui.selectable_value(&mut self.settings.data_bits, DataBits::Six, "6");
                ui.selectable_value(&mut self.settings.data_bits, DataBits::Seven, "7");
                ui.selectable_value(&mut self.settings.data_bits, DataBits::Eight, "8");
            });
        ui.end_row();
    }

    fn show_stop_bits_selector(&mut self, ui: &mut egui::Ui) {
        ui.label("停止位:");
        egui::ComboBox::from_id_salt("stop_bits_selector")
            .selected_text(format!("{:?}", self.settings.stop_bits))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.settings.stop_bits, StopBits::One, "1");
                ui.selectable_value(&mut self.settings.stop_bits, StopBits::Two, "2");
            });
        ui.end_row();
    }

    fn show_parity_selector(&mut self, ui: &mut egui::Ui) {
        ui.label("校验位:");
        egui::ComboBox::from_id_salt("parity_selector")
            .selected_text(format!("{:?}", self.settings.parity))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.settings.parity, Parity::None, "None");
                ui.selectable_value(&mut self.settings.parity, Parity::Odd, "Odd");
                ui.selectable_value(&mut self.settings.parity, Parity::Even, "Even");
            });
        ui.end_row();
    }

    fn show_flow_control_selector(&mut self, ui: &mut egui::Ui) {
        ui.label("流控制:");
        egui::ComboBox::from_id_salt("flow_control_selector")
            .selected_text(format!("{:?}", self.settings.flow_control))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.settings.flow_control, FlowControl::None, "None");
                ui.selectable_value(
                    &mut self.settings.flow_control,
                    FlowControl::Software,
                    "Software",
                );
                ui.selectable_value(
                    &mut self.settings.flow_control,
                    FlowControl::Hardware,
                    "Hardware",
                );
            });
        ui.end_row();
    }

    fn show_timeout_input(&mut self, ui: &mut egui::Ui) {
        ui.label("超时时间:");
        let mut timeout_ms = self.settings.timeout.as_millis() as u32;
        if ui
            .add(egui::DragValue::new(&mut timeout_ms).speed(10).suffix("ms"))
            .changed()
        {
            self.settings.timeout = Duration::from_millis(timeout_ms as u64);
        }
        ui.end_row();
    }

    fn show_dtr_checkbox(&mut self, ui: &mut egui::Ui) {
        ui.label("DTR状态:");
        let mut dtr_enabled = self.settings.dtr_on_open.unwrap_or(false);
        if ui.checkbox(&mut dtr_enabled, "").changed() {
            self.settings.dtr_on_open = Some(dtr_enabled);
        }
        ui.end_row();
    }

    fn show_connection_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("连接").clicked() {
                // TODO: 实现连接逻辑
            }
            if ui.button("断开").clicked() {
                // TODO: 实现断开逻辑
            }
        });
    }

    pub fn list_ports(&mut self) {
        self.list = serialport::available_ports()
            .unwrap_or_default()
            .into_iter()
            .filter(|port_info| {
                matches!(port_info.port_type, serialport::SerialPortType::UsbPort(_))
            })
            .map(|port_info| {
                if let serialport::SerialPortType::UsbPort(usb_info) = port_info.port_type {
                    let port_info_str = format!(
                        "{}",
                        usb_info.product.unwrap_or_else(|| "Unknown".to_string())
                    );
                    (port_info.port_name, port_info_str)
                } else {
                    (port_info.port_name, "Unknown".to_string())
                }
            })
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_ports() {
        let mut port = SerialPort::default();
        port.list_ports();
        println!("{:?}", port.list);
    }
}
