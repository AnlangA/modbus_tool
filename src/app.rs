//! 主应用模块
//!
//! 包含 ModbusTool 主应用结构体和实现，
//! 整合页面管理、任务管理、串口管理等功能。

use crate::app_ui::{add_font, show_top_menu};
use crate::master::Master;
use crate::page::{Page, PageManager};
use crate::serial::SerialPort;
use crate::slave::Slave;
use crate::task::TaskManager;
use eframe::{App, egui};
use log;

#[derive(Debug)]
pub struct ModbusTool {
    page_manager: PageManager,
    task_manager: TaskManager,
    serial: SerialPort,
    slave: Slave,
    master: Master,
}

impl Default for ModbusTool {
    fn default() -> Self {
        Self {
            page_manager: PageManager::new(),
            task_manager: TaskManager::new(),
            serial: SerialPort::default(),
            slave: Slave::default(),
            master: Master::default(),
        }
    }
}

impl ModbusTool {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        add_font(&cc.egui_ctx);
        Self::default()
    }

    fn handle_page_change(&mut self) {
        let previous_page = self.page_manager.previous_page();
        let current_page = self.page_manager.current_page();

        if previous_page != current_page {
            match current_page {
                Page::Slave => {
                    self.task_manager.set_handle_type(false);
                    log::info!("页面切换到Slave，设置handle_type为false");
                }
                Page::Master => {
                    self.task_manager.set_handle_type(true);
                    log::info!("页面切换到Master，设置handle_type为true");
                }
                Page::Home => {
                    log::info!("页面切换到Home，保持handle_type不变");
                }
            }
        }
    }

    fn show_current_page(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.page_manager.current_page() {
            Page::Home => self.serial.show(ctx, frame),
            Page::Slave => self.slave.show(ctx, frame),
            Page::Master => self.master.show(ctx, frame),
        }
    }

    // Task management methods (delegated to TaskManager)
    pub fn create_task(&mut self) {
        self.task_manager.create_task();
    }

    pub fn delete_task(&mut self) {
        self.task_manager.delete_task();
    }

    pub fn recreate_task(&mut self) {
        self.task_manager.recreate_task();
    }

    pub fn set_handle_type(&mut self, value: bool) {
        self.task_manager.set_handle_type(value);
    }

    pub fn get_handle_type(&self) -> bool {
        self.task_manager.get_handle_type()
    }
}

impl App for ModbusTool {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 显示顶部菜单并检测页面变化
        egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            let mut current_page = self.page_manager.current_page();
            show_top_menu(ui, &mut current_page);

            // 只有当页面真的发生变化时才设置新页面
            if current_page != self.page_manager.current_page() {
                self.page_manager.set_page(current_page);
                // 处理页面变化
                self.handle_page_change();
            }
        });

        // 显示当前页面
        self.show_current_page(ctx, frame);
    }
}
