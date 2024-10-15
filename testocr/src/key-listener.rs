use crate::ocr;
use enigo::{
    Button,
    Coordinate::Abs,
    Direction::{Press, Release},
    Enigo, Mouse, Settings,
};
use image::{ImageBuffer, Rgba};
use rdev::{listen, Event, EventType};
use regex::Regex;
use scrap::{Capturer, Display};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

pub fn key_listener() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => match key {
            rdev::Key::Kp0 => match capture_screen() {
                Ok(_) => match ocr::picture_ocr(&["D:/Download/1.png", "-", "-l", "eng"]) {
                    Ok(output) => {
                        println!("output: {output}");
                        if let Some(res) = cpt_basic_arithmetic(output) {
                            println!("{res}");
                            draw_result(res);
                        };
                    }
                    Err(error) => println!("failed with error: {}", error),
                },
                Err(err) => {
                    println!("123132{err}");
                }
            },
            rdev::Key::KeyQ => {
                println!("Exiting...");
                std::process::exit(0);
            }
            _ => (),
        },
        _ => (),
    }
}

fn capture_screen() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(display) = Display::primary() {
        let (_, height) = (display.width(), display.height());
        let mut capturer = Capturer::new(display)?;

        // 等待帧可用
        let frame = loop {
            match capturer.frame() {
                Ok(frame) => break frame, // 成功捕获到帧，退出循环
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // 等待一小段时间，等待下一帧
                    thread::sleep(Duration::from_millis(50));
                    continue;
                }
                Err(e) => return Err(Box::new(e)), // 其他错误直接返回
            }
        };

        let x = 1580; // 起始X坐标
        let y = 490; // 起始Y坐标
        let capture_width = 310; // 截取区域的宽度
        let capture_height = 80; // 截取区域的高度

        let mut img_buf = ImageBuffer::new(capture_width as u32, capture_height as u32);

        // 将捕获到的帧转换为图像缓冲区
        let stride = frame.len() / height; // 每行的字节数
        for row in 0..capture_height {
            for col in 0..capture_width {
                // 计算像素在帧中的位置
                let pixel_offset = (y + row) * stride + (x + col) * 4; // 每个像素是4个字节 (BGRA)
                let b = frame[pixel_offset];
                let g = frame[pixel_offset + 1];
                let r = frame[pixel_offset + 2];
                let a = 255; // 不透明度

                // 将像素存入图像缓冲区
                img_buf.put_pixel(col as u32, row as u32, Rgba([r, g, b, a]));
            }
        }

        // 保存图像为 PNG 文件
        img_buf.save("D:/Download/1.png")?;
        Ok(())
    } else {
        Err("123".into())
    }
}

fn cpt_basic_arithmetic(formula: String) -> Option<String> {
    let re = Regex::new(r"(\d+)\s*([-+*/])\s*(\d+)\s*=").unwrap();

    // 如果匹配到表达式
    if let Some(captures) = re.captures(&formula) {
        let num1 = i32::from_str(&captures[1]).unwrap();
        let operator = &captures[2];
        let num2 = i32::from_str(&captures[3]).unwrap();

        // 计算结果
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            _ => panic!("Unsupported operator"),
        };
        return Some(result.to_string());
    } else {
        None
    }
}

fn draw_result(res: String) {
    //
    let mut start_x = 1580;
    let start_y = 750;

    let wait_time = Duration::from_millis(50);
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    enigo.move_mouse(start_x, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    for c in res.chars() {
        match c {
            '0' => {
                draw_zero(wait_time, &mut enigo, start_x, start_y);
                start_x += 20;
            }
            '1' => draw_one(wait_time, &mut enigo, start_x, start_y),
            '2' => {
                draw_two(wait_time, &mut enigo, start_x, start_y);
                start_x += 20;
            }
            '3' => {
                draw_three(wait_time, &mut enigo, start_x, start_y);
                start_x += 20;
            }
            '4' => {
                draw_four(wait_time, &mut enigo, start_x, start_y);
                start_x += 20;
            }
            '5' => {
                draw_five(wait_time, &mut enigo, start_x, start_y);
                start_x += 20;
            }
            '6' => {
                draw_six(wait_time, &mut enigo, start_x, start_y);
                start_x += 20;
            }
            '7' => {
                draw_seven(wait_time, &mut enigo, start_x, start_y);
                start_x += 20;
            }
            '8' => {
                draw_eight(wait_time, &mut enigo, start_x, start_y);
                start_x += 20;
            }
            '9' => {
                draw_nine(wait_time, &mut enigo, start_x, start_y);
                start_x += 20;
            }
            _ => (),
        }
        thread::sleep(wait_time);
        enigo.button(Button::Left, Release).unwrap();
        thread::sleep(wait_time);
        start_x += 50;
        enigo.move_mouse(start_x, start_y, Abs).unwrap();
        thread::sleep(wait_time);
    }
}

const BASIC_W: i32 = 30;
const BASIC_H: i32 = 50;

fn draw_zero(wait_time: Duration, enigo: &mut Enigo, start_x: i32, start_y: i32) {
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x + BASIC_W, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x, start_y + BASIC_H, Abs).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x + 1, start_y - 10, Abs).unwrap();
}
fn draw_one(wait_time: Duration, enigo: &mut Enigo, start_x: i32, start_y: i32) {
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x, start_y + BASIC_H, Abs).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x - 5, start_y + BASIC_H, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + 5, start_y + BASIC_H, Abs)
        .unwrap();
}
fn draw_two(wait_time: Duration, enigo: &mut Enigo, start_x: i32, start_y: i32) {
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x + BASIC_W, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x, start_y + BASIC_H, Abs).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W + 5, start_y + BASIC_H, Abs)
        .unwrap();
}
fn draw_three(wait_time: Duration, enigo: &mut Enigo, start_x: i32, start_y: i32) {
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x + BASIC_W, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x, start_y + BASIC_H, Abs).unwrap();
}
fn draw_four(wait_time: Duration, enigo: &mut Enigo, start_x: i32, start_y: i32) {
    enigo.move_mouse(start_x + 6, start_y + 6, Abs).unwrap();
    thread::sleep(wait_time);
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W + 4, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo.button(Button::Left, Release).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x + 14, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + 14, start_y + BASIC_H, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + 14, start_y + BASIC_H + 4, Abs)
        .unwrap();
}
fn draw_five(wait_time: Duration, enigo: &mut Enigo, start_x: i32, start_y: i32) {
    enigo.move_mouse(start_x + BASIC_W, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x, start_y + BASIC_H, Abs).unwrap();
}
fn draw_six(wait_time: Duration, enigo: &mut Enigo, start_x: i32, start_y: i32) {
    enigo.move_mouse(start_x + BASIC_W, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x, start_y + BASIC_H, Abs).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + 6, start_y + BASIC_H / 2, Abs)
        .unwrap();
}
fn draw_seven(wait_time: Duration, enigo: &mut Enigo, start_x: i32, start_y: i32) {
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x + BASIC_W, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x, start_y + BASIC_H, Abs).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x, start_y + BASIC_H + 8, Abs)
        .unwrap();
}
fn draw_eight(wait_time: Duration, enigo: &mut Enigo, start_x: i32, start_y: i32) {
    enigo
        .move_mouse(start_x + BASIC_W / 2, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x + BASIC_W, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W / 2, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W / 2, start_y + BASIC_H / 2 + 10, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x, start_y + BASIC_H / 2 + 10, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x, start_y + BASIC_H + 10, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H + 10, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H / 2 + 10, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W / 2, start_y + BASIC_H / 2 + 10, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W / 2 - 4, start_y + BASIC_H / 2 + 10, Abs)
        .unwrap();
}
fn draw_nine(wait_time: Duration, enigo: &mut Enigo, start_x: i32, start_y: i32) {
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x, start_y + BASIC_H / 2, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(start_x + BASIC_W, start_y, Abs).unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H, Abs)
        .unwrap();
    thread::sleep(wait_time);
    enigo
        .move_mouse(start_x + BASIC_W, start_y + BASIC_H + 8, Abs)
        .unwrap();
}
