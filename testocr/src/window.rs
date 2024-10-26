// use std::{
//     cmp::{max, min},
//     ffi::c_void,
//     fs::File,
//     io::Write,
//     ptr,
// };

// use windows::{
//     core::*,
//     Win32::{
//         Foundation::*,
//         Graphics::Gdi::*,
//         System::{DataExchange::*, LibraryLoader::*, Ole::CF_BITMAP, SystemServices::*},
//         UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
//     },
// };

// static mut START: POINT = POINT { x: 0, y: 0 };
// static mut NOW: POINT = POINT { x: 0, y: 0 };
// static mut I_COUNT: i32 = 0;
// static mut I_ALPHA: u8 = 0;

// pub fn create_window() {
//     unsafe {
//         let hinstance = GetModuleHandleW(None).expect("Failed to get module handle");

//         let wndclass = WNDCLASSW {
//             cbClsExtra: 0,                                           //窗口扩展
//             cbWndExtra: 0,                                           //窗口实例扩展
//             hbrBackground: CreateSolidBrush(COLORREF(rgb(0, 0, 0))), //窗口背景色
//             hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),          //窗口鼠标光标
//             hInstance: hinstance.into(),                             //实例句柄
//             lpfnWndProc: Some(wnd_proc),                             //定义窗口处理函数
//             lpszClassName: w!("window_title"),                       //窗口类名为“窗口”
//             style: CS_HREDRAW | CS_VREDRAW, //窗口类的风格 | CS_HREDRAW: 当水平长度改变或移动窗口时，重画整个窗口 | CS_VREDRAW: 当垂直长度改变或移动窗口时，重画整个窗口
//             ..Default::default()
//         };

//         let atom: u16 = RegisterClassW(&wndclass);
//         debug_assert!(atom != 0);

//         let hwnd = CreateWindowExW(
//             WS_EX_LAYERED | WS_EX_TOOLWINDOW, //WS_EX_LAYERED：创建一个分层窗口 | WS_EX_TOOLWINDOW：创建工具窗口，即窗口是一个游动的工具条。
//             w!("window_title"),               //窗口类名
//             w!("window_title"),               //窗口名
//             WS_POPUP,                         //WS_POPUP：创建一个弹出式窗口。
//             CW_USEDEFAULT,
//             CW_USEDEFAULT,
//             CW_USEDEFAULT,
//             CW_USEDEFAULT,
//             None, //父窗口句柄
//             None,
//             hinstance, //模块句柄
//             Some(std::ptr::null()),
//         )
//         .expect("ASD");

//         let _ = ShowWindow(hwnd, SW_SHOWNORMAL);

//         UpdateWindow(hwnd);

//         let mut msg = MSG::default();
//         while GetMessageW(&mut msg, None, 0, 0).as_bool() {
//             TranslateMessage(&msg);
//             DispatchMessageW(&msg);
//         }
//     }
// }

// //WM_CREATE窗口创建
// //WM_TIMER定时器
// //WM_LBUTTONDOWN鼠标左键按下
// //WM_MOUSEMOVE鼠标移动
// //WM_LBUTTONUP鼠标左键松开
// //WM_PAINT绘图
// //WM_DESTROY销毁
// unsafe extern "system" fn wnd_proc(
//     hwnd: HWND,
//     message: u32,
//     w_param: WPARAM,
//     l_param: LPARAM,
// ) -> LRESULT {
//     match message {
//         WM_CREATE => {
//             SetTimer(hwnd, 1, 1, None);
//             SetCapture(hwnd);
//             LRESULT(0)
//         }
//         WM_TIMER => {
//             SetLayeredWindowAttributes(hwnd, COLORREF(0), I_ALPHA, LWA_ALPHA);
//             if I_ALPHA > 128 {
//                 KillTimer(hwnd, 1);
//             }
//             I_ALPHA += 10;
//             LRESULT(0)
//         }
//         WM_LBUTTONDOWN => {
//             GetCursorPos(&mut START);
//             LRESULT(0)
//         }
//         WM_MOUSEMOVE => {
//             if (w_param.0 as u32 & MK_LBUTTON.0 as u32) != 0 {
//                 NOW.x = l_param.0 as i32 & 0xFFFF;
//                 NOW.y = (l_param.0 as i32 >> 16) & 0xFFFF;
//                 I_COUNT += 1;
//                 if I_COUNT % 4 == 0 {
//                     InvalidateRect(hwnd, Some(ptr::null()), true);
//                 }
//             }
//             LRESULT(0)
//         }
//         WM_LBUTTONUP => {
//             capture_screenshot(hwnd);
//             ReleaseCapture();
//             DestroyWindow(hwnd);
//             LRESULT(0)
//         }
//         WM_PAINT => {
//             let mut ps = PAINTSTRUCT::default();
//             let hdc = BeginPaint(hwnd, &mut ps);
//             let h_brush_rect = CreateSolidBrush(COLORREF(rgb(233, 233, 233)));
//             let h_brush_frame = CreateSolidBrush(COLORREF(rgb(255, 255, 255)));
//             let mut rect = Default::default();
//             SetRect(
//                 &mut rect,
//                 min(START.x, NOW.x),
//                 min(START.y, NOW.y),
//                 max(START.x, NOW.x),
//                 max(START.y, NOW.y),
//             );
//             FillRect(hdc, &rect, h_brush_rect);
//             FrameRect(hdc, &rect, h_brush_frame);
//             EndPaint(hwnd, &ps);
//             DeleteObject(h_brush_frame);
//             DeleteObject(h_brush_rect);
//             LRESULT(0)
//         }
//         WM_DESTROY => {
//             PostQuitMessage(0);
//             LRESULT(0)
//         }
//         _ => DefWindowProcW(hwnd, message, w_param, l_param),
//     }
// }

// fn rgb(r: u32, g: u32, b: u32) -> u32 {
//     r + g * 256 + b * 256 * 256
// }

// unsafe fn capture_screenshot(hwnd: HWND) {
//     let hdc = GetDC(None);
//     let absx = (START.x - NOW.x).abs();
//     let absy = (START.y - NOW.y).abs();
//     let minx = min(START.x, NOW.x);
//     let miny = min(START.y, NOW.y);
//     let h_bitmap = CreateCompatibleBitmap(hdc, absx, absy);
//     let h_memdc = CreateCompatibleDC(hdc);
//     let holdbmp = SelectObject(h_memdc, h_bitmap);
//     BitBlt(h_memdc, 0, 0, absx, absy, hdc, minx, miny, SRCCOPY);

//     let mut bitmap_info = BITMAPINFO::default();
//     bitmap_info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as _;
//     GetDIBits(
//         h_memdc,
//         h_bitmap,
//         0,
//         absy as _,
//         Some(ptr::null_mut()),
//         &mut bitmap_info,
//         DIB_RGB_COLORS,
//     );
//     bitmap_info.bmiHeader.biCompression = 0;
//     let mut bitmap_bits: Vec<u8> = vec![0; bitmap_info.bmiHeader.biSizeImage as _];
//     GetDIBits(
//         h_memdc,
//         h_bitmap,
//         0,
//         absy as _,
//         Some(bitmap_bits.as_mut_ptr() as *mut c_void),
//         &mut bitmap_info,
//         DIB_RGB_COLORS,
//     );

//     create_bmp_file(bitmap_info, bitmap_bits);

//     OpenClipboard(hwnd);
//     EmptyClipboard();
//     SetClipboardData(CF_BITMAP.0.into(), HANDLE(h_bitmap.0)).unwrap();
//     CloseClipboard();

//     SelectObject(h_memdc, holdbmp);
//     DeleteDC(h_memdc);
//     DeleteObject(h_bitmap);
//     ReleaseDC(None, hdc);
// }

// fn create_bmp_file(bitmap_info: BITMAPINFO, bitmap_bits: Vec<u8>) {
//     // let temp_path = std::env::temp_dir().to_str().unwrap().to_string();
//     let temp_path = String::from("D:/Download/xiao_yuan_kou_suan");
//     let file_path = temp_path.clone() + "/jietutemp/jietutemp.bmp";
//     if !std::path::Path::new(&file_path).exists() {
//         std::fs::create_dir(temp_path.to_owned() + "/jietutemp").unwrap();
//     }
//     let mut f = File::create(file_path).unwrap();
//     f.write_all(b"BM").unwrap();
//     f.write_all(
//         &(bitmap_info.bmiHeader.biSizeImage
//             + std::mem::size_of::<BITMAPFILEHEADER>() as u32
//             + std::mem::size_of::<BITMAPINFOHEADER>() as u32)
//             .to_le_bytes(),
//     )
//     .unwrap();
//     f.write_all(&[0u16.to_le_bytes(), 0u16.to_le_bytes()].concat())
//         .unwrap();
//     f.write_all(
//         &(std::mem::size_of::<BITMAPFILEHEADER>() as u32
//             + std::mem::size_of::<BITMAPINFOHEADER>() as u32)
//             .to_le_bytes(),
//     )
//     .unwrap();
//     f.write_all(&40u32.to_le_bytes()).unwrap();
//     f.write_all(&bitmap_info.bmiHeader.biWidth.to_le_bytes())
//         .unwrap();
//     f.write_all(&bitmap_info.bmiHeader.biHeight.to_le_bytes())
//         .unwrap();
//     f.write_all(&1u16.to_le_bytes()).unwrap();
//     f.write_all(&bitmap_info.bmiHeader.biBitCount.to_le_bytes())
//         .unwrap();
//     f.write_all(&bitmap_info.bmiHeader.biCompression.to_le_bytes())
//         .unwrap();
//     f.write_all(&bitmap_info.bmiHeader.biSizeImage.to_le_bytes())
//         .unwrap();
//     f.write_all(&bitmap_info.bmiHeader.biXPelsPerMeter.to_le_bytes())
//         .unwrap();
//     f.write_all(&bitmap_info.bmiHeader.biYPelsPerMeter.to_le_bytes())
//         .unwrap();
//     f.write_all(&[0u32.to_le_bytes(), 0u32.to_le_bytes()].concat())
//         .unwrap();
//     f.write_all(&bitmap_bits).unwrap();
// }
