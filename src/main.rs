use std::{ffi::OsStr, os::windows::ffi::OsStrExt};
use winapi::{
    shared::{
        minwindef::{ATOM, BOOL, LPARAM, LRESULT, UINT, WPARAM},
        windef::{HWND, RECT},
    },
    um::{
        libloaderapi::GetModuleHandleW,
        wingdi::{PatBlt, BLACKNESS},
        winuser::{
            BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetClientRect,
            GetMessageW, PostQuitMessage, RegisterClassW, TranslateMessage, CS_HREDRAW, CS_OWNDC,
            CS_VREDRAW, CW_USEDEFAULT, MSG, PAINTSTRUCT, WM_ACTIVATEAPP, WM_CLOSE, WM_DESTROY,
            WM_PAINT, WM_SIZE, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

fn main() {
    // Create window class name
    let mut window_class_name = OsStr::new("KoalaChessWindowClass")
        .encode_wide()
        .collect::<Vec<u16>>();
    window_class_name.push(0);

    // Create window class
    let mut window_class = WNDCLASSW::default();

    // CS_OWNDC - Allocates a unique device context for each window in the class
    // CS_HREDRAW - Redraws the entire window if a movement or size adjustment changes the width of the client area
    // CS_VREDRAW - Redraws the entire window if a movement or size adjustment changes the height of the client area
    window_class.style = CS_OWNDC | CS_HREDRAW | CS_VREDRAW;
    window_class.lpfnWndProc = Some(window_proc);
    unsafe {
        window_class.hInstance = GetModuleHandleW(std::ptr::null());
    }
    window_class.lpszClassName = window_class_name.as_ptr();

    // Register window class
    let error_code: ATOM;

    unsafe {
        error_code = RegisterClassW(&window_class);
    }

    if error_code == 0 {
        eprintln!("Could not register window class!");
        return;
    }

    // Create window name
    let mut window_name = OsStr::new("Koala chess")
        .encode_wide()
        .collect::<Vec<u16>>();
    window_name.push(0);

    // Create window
    let window_handle: HWND;

    unsafe {
        window_handle = CreateWindowExW(
            0,
            window_class.lpszClassName,
            window_name.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            window_class.hInstance,
            std::ptr::null_mut(),
        );
    }

    if window_handle.is_null() {
        eprintln!("Could not create window!");
        return;
    }

    // Window loop
    loop {
        let mut message = MSG::default();
        let message_result: BOOL;

        unsafe {
            message_result = GetMessageW(&mut message, std::ptr::null_mut(), 0, 0);
        }

        if message_result == -1 {
            eprintln!("Could not retrieve message!");
            return;
        } else if message_result == 0 {
            // WM_CLOSE message
            return;
        }

        unsafe {
            // INFO: These calls could fail, but we can't really handle those fails
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match u_msg {
        WM_SIZE => {
            println!("WM_SIZE");
            let mut rect = RECT::default();
            GetClientRect(hwnd, &mut rect);
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;
            println!("width: {} / height: {}", width, height);
        }
        WM_DESTROY => {
            println!("WM_DESTROY");
            PostQuitMessage(0);
        }
        WM_CLOSE => {
            println!("WM_CLOSE");
            PostQuitMessage(0);
        }
        WM_ACTIVATEAPP => println!("WM_ACTIVATEAPP"),
        WM_PAINT => {
            let mut paint = PAINTSTRUCT::default();
            let device_context = BeginPaint(hwnd, &mut paint);
            let x = paint.rcPaint.left;
            let y = paint.rcPaint.top;
            let width = paint.rcPaint.right - paint.rcPaint.left;
            let height = paint.rcPaint.bottom - paint.rcPaint.top;
            PatBlt(device_context, x, y, width, height, BLACKNESS);
            EndPaint(hwnd, &paint);
        }
        _ => (),
    };

    DefWindowProcW(hwnd, u_msg, w_param, l_param)
}
