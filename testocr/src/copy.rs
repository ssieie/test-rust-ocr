use key_listener::key_listener;
use std::ptr::null_mut;
use windows::{
    core::*,
    Win32::{
        Foundation::{BOOL, COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::{CreateSolidBrush, Rectangle, UpdateWindow, HBRUSH},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetSystemMetrics,
            LoadCursorW, RegisterClassW, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW,
            CW_USEDEFAULT, IDC_ARROW, MSG, SM_CXSCREEN, SM_CYSCREEN, SW_SHOWNORMAL, WM_PAINT,
            WNDCLASSW, WS_EX_LAYERED, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_VISIBLE,
        },
    },
};

#[path = "../key-listener.rs"]
mod key_listener;
#[path = "../ocr.rs"]
mod ocr;

const RECT_WIDTH: i32 = 300;
const RECT_HEIGHT: i32 = 150;

extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_PAINT => {
                let hdc = windows::Win32::Graphics::Gdi::BeginPaint(hwnd, &mut std::mem::zeroed());
                let color = rgb(0, 0, 255);
                let brush: HBRUSH = CreateSolidBrush(COLORREF(color));
                let rect_left = 100;
                let rect_top = 100;
                let result = Rectangle(
                    hdc,
                    rect_left,
                    rect_top,
                    rect_left + RECT_WIDTH,
                    rect_top + RECT_HEIGHT,
                );
                if result == BOOL(0) {
                    // 处理错误
                    eprintln!("SetLayeredWindowAttributes failed");
                }
                let _ = windows::Win32::Graphics::Gdi::EndPaint(hwnd, &mut std::mem::zeroed());
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

fn main() -> Result<()> {
    unsafe {
        // 注册窗口类
        let class_name = w!("my_window");
        let instance = GetModuleHandleW(None)?;
        let wc = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            lpfnWndProc: Some(window_proc),
            hInstance: instance.into(),
            lpszClassName: class_name,
            style: CS_HREDRAW | CS_VREDRAW,
            ..Default::default()
        };

        let atom = RegisterClassW(&wc);
        debug_assert!(atom != 0);

        //SM_CXSCREEN,SM_CYSCREEN 以像素为单位计算的屏幕尺寸
        let cx = GetSystemMetrics(SM_CXSCREEN);
        let cy = GetSystemMetrics(SM_CYSCREEN);

        // 创建窗口（在桌面上显示）
        let hwnd = CreateWindowExW(
            windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE::default(), // 透明窗口
            class_name,
            w!("Desktop Overlay"),
            WS_POPUP | WS_VISIBLE, // 无边框和全屏显示
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            cx,
            cy,
            None,
            None,
            instance,
            None,
        );

        // println!("HWND: {:?}", hwnd);
        let _ = ShowWindow(hwnd.clone().unwrap(), SW_SHOWNORMAL);

        let _ = UpdateWindow(hwnd.clone().unwrap());

        // 消息循环
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    // key_listener();

    Ok(())
}

fn rgb(r: u32, g: u32, b: u32) -> u32 {
    (b << 16) | (g << 8) | r
}
