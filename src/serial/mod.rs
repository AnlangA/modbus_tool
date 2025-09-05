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
    path: String,
    /// The baud rate in symbols-per-second
    baud_rate: u32,
    /// Number of bits used to represent a character sent on the line
    data_bits: DataBits,
    /// The type of signalling to use for controlling data transfer
    flow_control: FlowControl,
    /// The type of parity to use for error checking
    parity: Parity,
    /// Number of bits to use to signal the end of a character
    stop_bits: StopBits,
    /// Amount of time to wait to receive data before timing out
    timeout: Duration,
    /// The state to set DTR to when opening the device
    dtr_on_open: Option<bool>,
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

            // 端口选择
            ui.horizontal(|ui| {
                ui.label("端口:");
                egui::ComboBox::from_label("端口")
                    .selected_text(&self.selected)
                    .show_ui(ui, |ui| {
                        self.list_ports();
                        for (port_name, port_info) in &self.list {
                            ui.selectable_value(
                                &mut self.selected,
                                port_name.clone(),
                                format!("{} - {}", port_name, port_info),
                            );
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("波特率:");
                ui.add(egui::DragValue::new(&mut self.settings.baud_rate).speed(100));
                ui.label("bps");
            });

            ui.horizontal(|ui| {
                ui.label("数据位:");
                egui::ComboBox::from_label("数据位")
                    .selected_text(format!("{:?}", self.settings.data_bits))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.settings.data_bits, DataBits::Five, "5");
                        ui.selectable_value(&mut self.settings.data_bits, DataBits::Six, "6");
                        ui.selectable_value(&mut self.settings.data_bits, DataBits::Seven, "7");
                        ui.selectable_value(&mut self.settings.data_bits, DataBits::Eight, "8");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("停止位:");
                egui::ComboBox::from_label("停止位")
                    .selected_text(format!("{:?}", self.settings.stop_bits))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.settings.stop_bits, StopBits::One, "1");
                        ui.selectable_value(&mut self.settings.stop_bits, StopBits::Two, "2");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("校验位:");
                egui::ComboBox::from_label("校验位")
                    .selected_text(format!("{:?}", self.settings.parity))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.settings.parity, Parity::None, "None");
                        ui.selectable_value(&mut self.settings.parity, Parity::Odd, "Odd");
                        ui.selectable_value(&mut self.settings.parity, Parity::Even, "Even");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("流控制:");
                egui::ComboBox::from_label("流控制")
                    .selected_text(format!("{:?}", self.settings.flow_control))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.settings.flow_control,
                            FlowControl::None,
                            "None",
                        );
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
            });

            ui.horizontal(|ui| {
                ui.label("超时时间:");
                let mut timeout_ms = self.settings.timeout.as_millis() as u32;
                if ui
                    .add(egui::DragValue::new(&mut timeout_ms).speed(10).suffix("ms"))
                    .changed()
                {
                    self.settings.timeout = Duration::from_millis(timeout_ms as u64);
                }
            });

            ui.horizontal(|ui| {
                ui.label("DTR状态:");
                let mut dtr_enabled = self.settings.dtr_on_open.unwrap_or(false);
                if ui.checkbox(&mut dtr_enabled, "").changed() {
                    self.settings.dtr_on_open = Some(dtr_enabled);
                }
            });

            // 连接按钮
            ui.horizontal(|ui| {
                if ui.button("连接").clicked() {
                    // TODO: 实现连接逻辑
                }
                if ui.button("断开").clicked() {
                    // TODO: 实现断开逻辑
                }
            });
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
