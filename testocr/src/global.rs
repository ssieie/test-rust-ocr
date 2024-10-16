use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub running: bool,
}

lazy_static! {
    pub static ref LAST_NUMS: Mutex<(i32, String, i32)> = Mutex::new((0, "".into(), 0));
    pub static ref APP_STATE: Arc<Mutex<AppState>> =
        Arc::new(Mutex::new(AppState { running: false }));
}
