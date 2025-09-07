use log;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct TaskManager {
    cancel_flag: Arc<AtomicBool>,
    handle_type: Arc<AtomicBool>,
    task_handle: Option<JoinHandle<()>>,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self {
            cancel_flag: Arc::new(AtomicBool::new(false)),
            handle_type: Arc::new(AtomicBool::new(false)),
            task_handle: None,
        }
    }
}

impl TaskManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_task(&mut self) {
        log::info!("创建新的串口任务");
        let cancel_flag = self.cancel_flag.clone();
        let handle = tokio::spawn(async move {
            log::info!("串口任务启动");
            while !cancel_flag.load(Ordering::Relaxed) {
                // 任务逻辑
                println!("串口运行中");
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            log::info!("串口任务退出");
        });
        self.task_handle = Some(handle);
        log::info!("串口任务创建成功");
    }

    pub fn delete_task(&mut self) {
        log::info!("删除串口任务");
        if let Some(handle) = self.task_handle.take() {
            log::info!("设置取消标志，等待任务退出");
            self.cancel_flag.store(true, Ordering::Relaxed);
            handle.abort();
            log::info!("串口任务删除成功");
        } else {
            log::info!("没有正在运行的串口任务");
        }
    }

    pub fn recreate_task(&mut self) {
        log::info!("重新创建串口任务");
        self.delete_task();
        log::info!("创建新的取消标志");
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        self.create_task();
        log::info!("串口任务重新创建成功");
    }

    pub fn set_handle_type(&mut self, value: bool) {
        self.handle_type.store(value, Ordering::Relaxed);
        log::info!("设置 handle_type: {}", value);
    }

    pub fn get_handle_type(&self) -> bool {
        let value = self.handle_type.load(Ordering::Relaxed);
        log::info!("获取 handle_type: {}", value);
        value
    }
}
