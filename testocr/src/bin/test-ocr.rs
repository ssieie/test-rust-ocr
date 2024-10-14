use key_listener::key_listener;


#[path = "../ocr.rs"]
mod ocr;
#[path = "../key-listener.rs"]
mod key_listener;

fn main() -> std::io::Result<()> {
    key_listener()
}
