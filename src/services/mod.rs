use crate::models::Task;
use std::collections::HashMap;
use std::sync::Mutex;

pub mod platform;
pub mod swagger;
pub mod wechatpay;

pub struct AppState {
    pub tasks: Mutex<HashMap<usize, Task>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            tasks: Mutex::new(HashMap::new()),
        }
    }
}
