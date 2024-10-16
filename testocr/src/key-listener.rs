use crate::ocr;
use crossterm::event::{KeyEvent, KeyEventKind};
use crossterm::{event, execute, terminal};
use enigo::{
    Button,
    Direction::{Press, Release},
    Enigo, Mouse, Settings,
    {Coordinate::Abs, Coordinate::Rel},
};
use image::{ImageBuffer, Rgba};
use regex::Regex;
use scrap::{Capturer, Display};
use std::io::stdout;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

pub fn key_listener() -> std::io::Result<()> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
    terminal::enable_raw_mode()?;

    loop {
        // 监听键盘输入
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                match code {
                    event::KeyCode::Char('0') => match capture_screen() {
                        Ok(_) => match ocr::picture_ocr(&["D:/Download/1.png", "-", "-l", "eng"]) {
                            Ok(output) => {
                                if let Some(res) = cpt_basic_arithmetic(output) {
                                    println!("{res}");
                                    draw_result();
                                };
                            }
                            Err(error) => println!("failed with error: {}", error),
                        },
                        Err(err) => {
                            println!("123132{err}");
                        }
                    },
                    event::KeyCode::Char('q') => {
                        println!("Exiting...");
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
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
        let y = 200; // 起始Y坐标
        let capture_width = 300; // 截取区域的宽度
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

fn cpt_basic_arithmetic(formula: String) -> Option<i32> {
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
        return Some(result);
    } else {
        None
    }
}

fn draw_result() {
    let wait_time = Duration::from_millis(10);
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    enigo.move_mouse(100, 300, Abs).unwrap();
    thread::sleep(wait_time);
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(50, 0, Rel).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(-50, 50, Rel).unwrap();
    thread::sleep(wait_time);
    enigo.move_mouse(50, 0, Rel).unwrap();
    thread::sleep(wait_time);
    enigo.button(Button::Left, Release).unwrap();
}
