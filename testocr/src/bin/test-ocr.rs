use key_listener::key_listener;

#[path = "../global.rs"]
mod global;
#[path = "../key-listener.rs"]
mod key_listener;
#[path = "../ocr.rs"]
mod ocr;

fn main() {
    println!("按0启动程序\n按空格停止程序\n按Q退出程序");
    key_listener();
}
