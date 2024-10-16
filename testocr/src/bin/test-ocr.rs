use key_listener::key_listener;

#[path = "../global.rs"]
mod global;
#[path = "../key-listener.rs"]
mod key_listener;
#[path = "../ocr.rs"]
mod ocr;

fn main() {
    key_listener();
}
