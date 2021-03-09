use crate::game::Game;
use crate::traits::Draw;
use std::ffi::{CString, OsStr};
use std::io;
use std::lazy::SyncLazy;
use std::os::windows::ffi::OsStrExt;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use winapi::{
    shared::{
        minwindef::{ATOM, HMODULE, LPARAM, LRESULT, PROC, UINT, WORD, WPARAM},
        windef::{HDC, HWND, RECT},
    },
    um::{
        libloaderapi::{GetModuleHandleW, GetProcAddress, LoadLibraryW},
        profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency},
        wingdi::{
            wglCreateContext, wglGetProcAddress, wglMakeCurrent, ChoosePixelFormat,
            DescribePixelFormat, SetPixelFormat, SwapBuffers, PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW,
            PFD_MAIN_PLANE, PFD_SUPPORT_OPENGL, PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR,
        },
        winnt::LARGE_INTEGER,
        winuser::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetClientRect, GetDC, PeekMessageW,
            PostQuitMessage, RegisterClassW, ReleaseDC, TranslateMessage, CS_HREDRAW, CS_OWNDC,
            CS_VREDRAW, CW_USEDEFAULT, MSG, PM_REMOVE, WM_CLOSE, WM_DESTROY, WM_QUIT, WM_SIZE,
            WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

static ASPECT_RATIO: SyncLazy<Mutex<f32>> = SyncLazy::new(|| Mutex::new(1.0));
static INITIALIZED_OPEN_GL: SyncLazy<AtomicBool> = SyncLazy::new(|| AtomicBool::new(false));

pub fn initialize() {
    // Create module name
    let module_name = OsStr::new("opengl32.dll\0")
        .encode_wide()
        .collect::<Vec<u16>>();

    // Load module
    let module = unsafe { LoadLibraryW(module_name.as_ptr()) };

    if module.is_null() {
        // TODO: Error handling
        eprintln!(
            "OpenGL module is null! (os error: {})",
            io::Error::last_os_error()
        );
        return;
    }
}

pub fn get_open_gl_address(function_name: &str) -> *const std::ffi::c_void {
    // Create null-terminated function name
    let null_terminated_function_name = CString::new(function_name).unwrap();

    // Get address (via wglGetProcAddress)
    let mut address = unsafe { wglGetProcAddress(null_terminated_function_name.as_ptr()) };

    if address.is_null()
        || address == 1 as PROC
        || address == 2 as PROC
        || address == 3 as PROC
        || address == -1_isize as PROC
    {
        // Get address (via GetProcAddress)
        address = unsafe { GetProcAddress(module, null_terminated_function_name.as_ptr()) };
    }

    if address.is_null() {
        // TODO: Error handling
        eprintln!(
            "OpenGL address ({}) is null! (os error: {})",
            function_name,
            io::Error::last_os_error()
        );
    }

    address as *const std::ffi::c_void
}

pub fn create_window() -> Option<HWND> {
    // Create window class name
    let mut window_class_name = OsStr::new("KoalaChessWindowClass\0")
        .encode_wide()
        .collect::<Vec<u16>>();
    window_class_name.push(0);

    // Create window class
    let window_class = WNDCLASSW {
        // CS_OWNDC - Allocates a unique device context for each window in the class
        // CS_HREDRAW - Redraws the entire window if a movement or size adjustment changes the width of the client area
        // CS_VREDRAW - Redraws the entire window if a movement or size adjustment changes the height of the client area
        style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_proc),
        hInstance: unsafe { GetModuleHandleW(std::ptr::null()) },
        lpszClassName: window_class_name.as_ptr(),
        ..Default::default()
    };

    // Register window class
    let error_code: ATOM;

    unsafe {
        error_code = RegisterClassW(&window_class);
    }

    if error_code == 0 {
        // TODO: Error handling
        eprintln!(
            "Could not register window class! (os error: {})",
            io::Error::last_os_error()
        );
        return None;
    }

    // Create window name
    let mut window_name = OsStr::new("Koala chess\0")
        .encode_wide()
        .collect::<Vec<u16>>();
    window_name.push(0);

    // Create window
    let window: HWND;

    unsafe {
        window = CreateWindowExW(
            0,                                // dwExStyle
            window_class.lpszClassName,       // lpClassName
            window_name.as_ptr(),             // lpWindowName
            WS_OVERLAPPEDWINDOW | WS_VISIBLE, // dwStyle
            CW_USEDEFAULT,                    // X
            CW_USEDEFAULT,                    // Y
            CW_USEDEFAULT,                    // nWidth
            CW_USEDEFAULT,                    // nHeight
            std::ptr::null_mut(),             // hWndParent
            std::ptr::null_mut(),             // hMenu
            window_class.hInstance,           // hInstance
            std::ptr::null_mut(),             // lpParam
        );
    }

    if window.is_null() {
        // TODO: Error handling
        eprintln!(
            "Could not create window! (os error: {})",
            io::Error::last_os_error()
        );
        return None;
    }

    // Initialize OpenGL
    initialize_open_gl(window);

    Some(window)
}

pub fn r#loop(window: HWND, game: Game) {
    let device_context = unsafe { GetDC(window) };

    // The frequency of the performance counter is fixed at system boot and is consistent across all processors
    let mut performance_frequency = LARGE_INTEGER::default();
    unsafe { QueryPerformanceFrequency(&mut performance_frequency) };

    let mut last_performance_counter = LARGE_INTEGER::default();
    unsafe { QueryPerformanceCounter(&mut last_performance_counter) };

    let mut running = true;

    while running {
        let mut message = MSG::default();

        // Window loop
        while unsafe { PeekMessageW(&mut message, std::ptr::null_mut(), 0, 0, PM_REMOVE) } != 0 {
            if message.message == WM_QUIT {
                println!("window_proc: WM_QUIT");
                running = false;
                break;
            }

            unsafe {
                // INFO: These calls could fail, but we can't really handle those fails
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }

        // Rendering
        unsafe {
            // Draw game
            game.draw(*ASPECT_RATIO.lock().unwrap());

            SwapBuffers(device_context);
        }

        let mut end_performance_counter = LARGE_INTEGER::default();
        unsafe { QueryPerformanceCounter(&mut end_performance_counter) };

        let elapsed_performance_counter =
            unsafe { end_performance_counter.QuadPart() - last_performance_counter.QuadPart() };

        // ms = 1000 * counter / (counter / s) = 1000 * counter * (s / counter)
        let elapsed_milliseconds = 1000f64 * elapsed_performance_counter as f64
            / unsafe { *performance_frequency.QuadPart() as f64 };

        // 1/s = (counter / s) / counter
        let frames_per_second = unsafe { *performance_frequency.QuadPart() as f64 }
            / elapsed_performance_counter as f64;

        println!(
            "frames per second: {} / frame time: {}ms",
            frames_per_second, elapsed_milliseconds
        );

        last_performance_counter = end_performance_counter;
    }
}

unsafe extern "system" fn window_proc(
    window: HWND,
    message: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match message {
        WM_SIZE => {
            println!("window_proc: WM_SIZE");
            let mut rect = RECT::default();
            GetClientRect(window, &mut rect);
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;
            let aspect_ratio = width as f32 / height as f32;
            println!(
                "WM_SIZE: width: {} / height: {} / aspect_ratio: {}",
                width, height, aspect_ratio
            );

            *ASPECT_RATIO.lock().unwrap() = aspect_ratio;

            if INITIALIZED_OPEN_GL.load(Ordering::SeqCst) {
                // Set viewport
                gl::Viewport(0, 0, width, height);
            }
        }
        WM_DESTROY => {
            println!("window_proc: WM_DESTROY");
            PostQuitMessage(0);
        }
        WM_CLOSE => {
            println!("window_proc: WM_CLOSE");
            PostQuitMessage(0);
        }
        _ => (),
    };

    DefWindowProcW(window, message, w_param, l_param)
}

fn initialize_open_gl(window: HWND) {
    let device_context = unsafe { GetDC(window) };

    negotiate_pixel_format(device_context);

    let rendering_context = unsafe { wglCreateContext(device_context) };

    if unsafe { wglMakeCurrent(device_context, rendering_context) } == 0 {
        // TODO: Error handling
        eprintln!("wglMakeCurrent failed!");
    }

    unsafe { ReleaseDC(window, device_context) };

    initialize_open_gl_addresses();

    INITIALIZED_OPEN_GL.store(true, Ordering::SeqCst);
}

fn negotiate_pixel_format(device_context: HDC) {
    // Create desired pixel format
    let desired_pixel_format = PIXELFORMATDESCRIPTOR {
        nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as WORD,
        nVersion: 1,
        iPixelType: PFD_TYPE_RGBA,
        dwFlags: PFD_SUPPORT_OPENGL | PFD_DRAW_TO_WINDOW | PFD_DOUBLEBUFFER,

        // RGBA
        cColorBits: 32,

        // Alpha
        cAlphaBits: 8,

        iLayerType: PFD_MAIN_PLANE,

        ..Default::default()
    };

    let suggested_pixel_format_index =
        unsafe { ChoosePixelFormat(device_context, &desired_pixel_format) };
    let mut suggested_pixel_format = PIXELFORMATDESCRIPTOR::default();
    unsafe {
        DescribePixelFormat(
            device_context,
            suggested_pixel_format_index,
            std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as UINT,
            &mut suggested_pixel_format,
        );
        SetPixelFormat(
            device_context,
            suggested_pixel_format_index,
            &suggested_pixel_format,
        );
    };
}
