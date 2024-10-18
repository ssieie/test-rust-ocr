use crate::global;
use crate::ocr;
use image::{ImageBuffer, Rgba};
use rdev::{listen, Event, EventType};
use regex::Regex;
use scrap::{Capturer, Display};
use std::process::Command;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

// Physical size: 1080x2400

const ADB_PATH: &str = "D:/developmentTools/androidSdk/platform-tools/adb.exe";

const PICTURN_PATH: &str = "D:/Download/xiao_yuan_kou_suan/1.png";

const BASIC_W: i32 = 60;
const BASIC_H: i32 = 120;

const GAP: i32 = 80;

const DURATION: &str = "0";

type Setper = Vec<(i32, i32, i32, i32)>;

pub fn key_listener() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => match key {
            // rdev::Key::Kp0 => {
            //     let start = std::time::Instant::now();
            //     let _ = draw_result("012".into());
            //     let duration = start.elapsed();
            //     println!("耗时:{:?}",duration);
            // },
            rdev::Key::Kp0 => start_task(),
            rdev::Key::KeyR => reset_formula(),
            rdev::Key::Space => stop_task(),
            rdev::Key::KeyQ => {
                println!("退出中...");
                std::process::exit(0);
            }
            _ => (),
        },
        _ => (),
    }
}

fn start_task() {
    let state = global::APP_STATE.clone();
    if state.lock().unwrap().running {
        return;
    }
    println!("已开始...");

    thread::spawn(move || {
        let mut state_lock = state.lock().unwrap();
        state_lock.running = true;
        while state_lock.running {
            drop(state_lock);
            loop_task();
            state_lock = state.lock().unwrap();
        }
    });
}

fn stop_task() {
    println!("已停止...");
    let mut state = global::APP_STATE.lock().unwrap();
    state.running = false;
}

fn loop_task() {
    match capture_screen() {
        Ok(_) => {
            match ocr::picture_ocr(&[PICTURN_PATH, "-", "-l", "eng"]) {
                Ok(output) => {
                    if let Some(res) = cpt_basic_arithmetic(&output) {
                        println!("识别结果:{}计算结果:{}", output, res);
                        match draw_result(res) {
                            Err(err) => {
                                println!("{}", err);
                            }
                            _ => (),
                        }
                    };
                }
                Err(error) => println!("failed with error: {}", error),
            }
        }
        Err(err) => {
            println!("截图错误: {err}");
        }
    }
}

fn capture_screen() -> Result<(), Box<dyn std::error::Error>> {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;
    let display = Display::primary()?;
    let mut capturer = Capturer::new(display)?;

    let (_, height) = (capturer.width(), capturer.height());

    // 等待帧可用
    let frame = loop {
        match capturer.frame() {
            Ok(frame) => break frame, // 成功捕获到帧，退出循环
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // 等待一小段时间，等待下一帧
                thread::sleep(one_frame);
                continue;
            }
            Err(e) => return Err(Box::new(e)), // 其他错误直接返回
        }
    };

    let x = 1580; // 起始X坐标
    let y = 400; // 起始Y坐标
    let capture_width = 300; // 截取区域的宽度, 计算模式
    let capture_height = 110; // 截取区域的高度

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

    // 处理图像以进行灰度化和二值化
    // let processed_img = process_image(&img_buf);

    // 保存图像为 PNG 文件
    img_buf
        .save(PICTURN_PATH)
        .expect("保存图片错误");

    Ok(())
}

// fn process_image(img_buf: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> GrayImage {
//     let (width, height) = img_buf.dimensions();
//     let mut gray_img = ImageBuffer::new(width, height);

//     for (x, y, pixel) in img_buf.enumerate_pixels() {
//         let Rgba([r, g, b, _]) = *pixel; // 提取RGBA值
//         let gray = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8; // 计算灰度值
//         gray_img.put_pixel(x, y, Luma([gray])); // 将灰度值存入灰度图像
//     }

//     // 应用二值化
//     let binary_img: GrayImage = ImageBuffer::from_fn(width, height, |x, y| {
//         let Luma([luma]) = gray_img.get_pixel(x, y);
//         if luma > &150u8 {
//             Luma([255]) // 白色
//         } else {
//             Luma([0]) // 黑色
//         }
//     });

//     binary_img
// }

fn cpt_basic_arithmetic(formula: &str) -> Option<String> {
    let re = Regex::new(r"(\d+)\s*([-+x/])\s*(\d+)\s*").unwrap();

    // 如果匹配到表达式
    if let Some(captures) = re.captures(&formula) {
        let num1 = i32::from_str(&captures[1]).unwrap();
        let operator = &captures[2];
        let num2 = i32::from_str(&captures[3]).unwrap();

        //
        let mut last_formula = global::LAST_FORMULA.lock().unwrap();
        if last_formula.0 == num1 && last_formula.1 == operator && last_formula.2 == num2 {
            return None;
        } else {
            *last_formula = (num1, operator.to_string(), num2);
        }

        // let mut last_nums = global::LAST_NUMS.lock().unwrap();
        // let key = (num1, operator.to_string(), num2);
        // match last_nums.get(&key) {
        //     Some(&n) if n < 2 => {
        //         last_nums.insert(key, n + 1);
        //     }
        //     None => {
        //         last_nums.insert((num1, operator.into(), num2), 1);
        //     }
        //     Some(_) => {
        //         return None;
        //     }
        // };

        // 计算结果
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "x" => num1 * num2,
            "/" => num1 / num2,
            _ => panic!("Unsupported operator"),
        };
        return Some(result.to_string());
    } else {
        None
    }
}

fn reset_formula() {
    let mut last_formula = global::LAST_FORMULA.lock().unwrap();
    *last_formula = (0, "".to_string(), 0);
}

fn draw_result(res: String) -> Result<(), Box<dyn std::error::Error>> {
    let h: std::sync::MutexGuard<'_, i32> = global::DEVICE_H.lock()?;
    let mut start_x = 90;
    let start_y = (*h) / 2 + 100;

    let mut swipe_command = String::from("");

    for c in res.chars() {
        match c {
            '0' => swipe_command.push_str(&draw_zero(start_x, start_y)),
            '1' => swipe_command.push_str(&draw_one(start_x, start_y)),
            '2' => swipe_command.push_str(&draw_two(start_x, start_y)),
            '3' => swipe_command.push_str(&draw_three(start_x, start_y)),
            '4' => swipe_command.push_str(&draw_four(start_x, start_y)),
            '5' => swipe_command.push_str(&draw_five(start_x, start_y)),
            '6' => swipe_command.push_str(&draw_six(start_x, start_y)),
            '7' => swipe_command.push_str(&draw_seven(start_x, start_y)),
            '8' => swipe_command.push_str(&draw_eight(start_x, start_y)),
            '9' => swipe_command.push_str(&draw_nine(start_x, start_y)),
            _ => (),
        }
        start_x += BASIC_W + GAP + 20;
    }

    execute_adb_commands(swipe_command)?;

    Ok(())
}

fn draw_zero(start_x: i32, start_y: i32) -> String {
    let steps = vec![
        (start_x, start_y, start_x + BASIC_W, start_y),
        (
            start_x + BASIC_W,
            start_y,
            start_x + BASIC_W / 2,
            start_y + BASIC_H,
        ),
        (start_x + BASIC_W / 2, start_y + BASIC_H, start_x, start_y),
    ];

    vec_to_string(steps)
}

fn draw_one(start_x: i32, start_y: i32) -> String {
    let steps = vec![(
        start_x + BASIC_W / 2,
        start_y,
        start_x + BASIC_W / 2,
        start_y + BASIC_H,
    )];

    vec_to_string(steps)
}

fn draw_two(start_x: i32, start_y: i32) -> String {
    let steps = vec![
        (start_x, start_y, start_x + BASIC_W, start_y),
        (start_x + BASIC_W, start_y, start_x, start_y + BASIC_H),
        (
            start_x,
            start_y + BASIC_H,
            start_x + BASIC_W,
            start_y + BASIC_H,
        ),
    ];

    vec_to_string(steps)
}

fn draw_three(start_x: i32, start_y: i32) -> String {
    let steps = vec![
        (start_x, start_y, start_x + BASIC_W, start_y + BASIC_H / 3),
        (
            start_x + BASIC_W,
            start_y + BASIC_H / 3,
            start_x,
            start_y + BASIC_H / 2,
        ),
        (
            start_x,
            start_y + BASIC_H / 2,
            start_x + BASIC_W,
            start_y + BASIC_H,
        ),
        (
            start_x + BASIC_W,
            start_y + BASIC_H,
            start_x,
            start_y + BASIC_H,
        ),
    ];

    vec_to_string(steps)
}

fn draw_four(start_x: i32, start_y: i32) -> String {
    let steps = vec![
        (start_x, start_y, start_x, start_y + BASIC_H / 2),
        (
            start_x,
            start_y + BASIC_H / 2,
            start_x + BASIC_W,
            start_y + BASIC_H / 2,
        ),
        (
            start_x + BASIC_W / 2,
            start_y,
            start_x + BASIC_W / 2,
            start_y + BASIC_H,
        ),
    ];

    vec_to_string(steps)
}

fn draw_five(start_x: i32, start_y: i32) -> String {
    let steps = vec![
        (start_x, start_y, start_x, start_y + BASIC_H / 2),
        (
            start_x,
            start_y + BASIC_H / 2,
            start_x + BASIC_W,
            start_y + BASIC_H / 2,
        ),
        (
            start_x + BASIC_W,
            start_y + BASIC_H / 2,
            start_x + BASIC_W,
            start_y + BASIC_H,
        ),
        (
            start_x + BASIC_W,
            start_y + BASIC_H,
            start_x,
            start_y + BASIC_H,
        ),
        (start_x, start_y, start_x + BASIC_W, start_y),
    ];

    vec_to_string(steps)
}

fn draw_six(start_x: i32, start_y: i32) -> String {
    let steps = vec![
        (start_x, start_y, start_x, start_y + BASIC_H),
        (
            start_x,
            start_y + BASIC_H,
            start_x + BASIC_W,
            start_y + BASIC_H / 2,
        ),
        (
            start_x + BASIC_W,
            start_y + BASIC_H / 2,
            start_x,
            start_y + BASIC_H / 2,
        ),
    ];

    vec_to_string(steps)
}

fn draw_seven(start_x: i32, start_y: i32) -> String {
    let steps = vec![
        (start_x, start_y, start_x + BASIC_W, start_y),
        (start_x + BASIC_W, start_y, start_x, start_y + BASIC_H),
    ];

    vec_to_string(steps)
}

fn draw_eight(start_x: i32, start_y: i32) -> String {
    let steps = vec![
        (
            start_x + BASIC_W / 2,
            start_y + BASIC_H / 2,
            start_x,
            start_y,
        ),
        (start_x, start_y, start_x + BASIC_W, start_y),
        (start_x + BASIC_W, start_y, start_x, start_y + BASIC_H),
        (
            start_x,
            start_y + BASIC_H,
            start_x + BASIC_W,
            start_y + BASIC_H,
        ),
        (start_x + BASIC_W, start_y + BASIC_H, start_x, start_y),
    ];

    vec_to_string(steps)
}

fn draw_nine(start_x: i32, start_y: i32) -> String {
    let steps = vec![
        (
            start_x + BASIC_W,
            start_y + BASIC_H / 3,
            start_x,
            start_y + BASIC_H / 3,
        ),
        (start_x, start_y + BASIC_H / 3, start_x, start_y),
        (start_x, start_y, start_x + BASIC_W, start_y),
        (start_x + BASIC_W, start_y, start_x, start_y + BASIC_H),
    ];

    vec_to_string(steps)
}

fn vec_to_string(steps: Setper) -> String {
    let commands: Vec<Vec<String>> = steps
        .iter()
        .map(|&(x1, y1, x2, y2)| {
            vec![
                "input".to_string(),
                "swipe".to_string(),
                x1.to_string(),
                y1.to_string(),
                x2.to_string(),
                y2.to_string(),
                DURATION.to_string(),
            ]
        })
        .collect();

    let swipe_commands: Vec<String> = commands.iter().map(|cmd| cmd.join(" ")).collect();

    format!("{}; ", swipe_commands.join("; "))
}

fn execute_adb_commands(arg: String) -> Result<(), Box<dyn std::error::Error>> {
    Command::new(ADB_PATH).arg("shell").arg(arg).output()?;

    Ok(())
}
