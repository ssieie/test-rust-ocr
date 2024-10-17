use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct AppState {
    pub running: bool,
}

lazy_static! {
    pub static ref DEVICE_W: Mutex<i32> = Mutex::new(0);
    pub static ref DEVICE_H: Mutex<i32> = Mutex::new(0);
    pub static ref LAST_NUMS: Mutex<HashMap<(i32, String, i32), u8>> = Mutex::new(HashMap::new());
    pub static ref LAST_FORMULA: Mutex<(i32, String, i32)> = Mutex::new((0, String::from(""), 0));
    pub static ref APP_STATE: Arc<Mutex<AppState>> =
        Arc::new(Mutex::new(AppState { running: false }));
}
