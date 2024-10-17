use key_listener::key_listener;
use regex::Regex;
use std::process::Command;

#[path = "../global.rs"]
mod global;
#[path = "../key-listener.rs"]
mod key_listener;
#[path = "../ocr.rs"]
mod ocr;

const ADB_PATH: &str = "D:/developmentTools/androidSdk/platform-tools/adb.exe";

fn main() {
    let output = Command::new(ADB_PATH).arg("devices").output().unwrap();

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.len() > 28 {
            println!("模拟设备在线...\r\n");
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("执行'adb devices'失败: {}", stderr)
    }

    let output = Command::new(ADB_PATH)
        .args(&["shell", "wm", "size"])
        .output()
        .unwrap();

    let output_str = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"(\d+)x(\d+)").unwrap();
    if let Some(captures) = re.captures(&output_str) {
        let mut w = global::DEVICE_W.lock().unwrap();
        let mut h = global::DEVICE_H.lock().unwrap();

        *w = captures[1].parse().unwrap();
        *h = captures[2].parse().unwrap();
    }

    println!("按0启动程序\r\n按空格停止程序\r\n按R重置输入\r\n按Q退出程序");

    key_listener();
}
