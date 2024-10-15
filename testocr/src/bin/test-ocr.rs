use key_listener::key_listener;

#[path = "../key-listener.rs"]
mod key_listener;
#[path = "../ocr.rs"]
mod ocr;

fn main() {
    key_listener();
}