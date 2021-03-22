use crate::game::Game;
use crate::renderer::open_gl;
use crate::traits::Draw;
use logger::*;
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

struct ModuleHandle(HMODULE);

// Implement Send for the HMODULE wrapper, because HMODULE does not implement it
unsafe impl Send for ModuleHandle {}

static OPEN_GL_MODULE: SyncLazy<Mutex<ModuleHandle>> =
    SyncLazy::new(|| Mutex::new(ModuleHandle(std::ptr::null_mut())));
static INITIALIZED_OPEN_GL: SyncLazy<AtomicBool> = SyncLazy::new(|| AtomicBool::new(false));
static ASPECT_RATIO: SyncLazy<Mutex<f32>> = SyncLazy::new(|| Mutex::new(1.0));

pub fn create_window() -> HWND {
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
        logger::fatal!(
            "Could not register window class! (os error: {})",
            io::Error::last_os_error()
        );
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
        logger::fatal!(
            "Could not create window! (os error: {})",
            io::Error::last_os_error()
        );
    }

    // Initialize OpenGL
    initialize_open_gl(window);

    window
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
                logger::info!("window_proc: WM_QUIT");
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
            game.draw(
                *ASPECT_RATIO
                    .lock()
                    .expect("Could not lock aspect ratio mutex!"),
            );

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

fn initialize_open_gl(window: HWND) {
    let device_context = unsafe { GetDC(window) };

    if device_context.is_null() {
        logger::fatal!("Could not get device context!");
    }

    negotiate_pixel_format(device_context);

    let rendering_context = unsafe { wglCreateContext(device_context) };

    if rendering_context.is_null() {
        logger::fatal!("Could not create OpenGL rendering context!");
    }

    if unsafe { wglMakeCurrent(device_context, rendering_context) } == 0 {
        logger::fatal!(
            "wglMakeCurrent failed! (os error: {})",
            io::Error::last_os_error()
        );
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

    if suggested_pixel_format_index == 0 {
        logger::fatal!(
            "Could not choose pixel format! (os error: {})",
            io::Error::last_os_error()
        );
    }

    let mut suggested_pixel_format = PIXELFORMATDESCRIPTOR::default();

    unsafe {
        if DescribePixelFormat(
            device_context,
            suggested_pixel_format_index,
            std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as UINT,
            &mut suggested_pixel_format,
        ) == 0
        {
            logger::fatal!(
                "Could not get pixel format description! (os error: {})",
                io::Error::last_os_error()
            );
        }

        if SetPixelFormat(
            device_context,
            suggested_pixel_format_index,
            &suggested_pixel_format,
        ) == 0
        {
            logger::fatal!(
                "Could not set pixel format! (os error: {})",
                io::Error::last_os_error()
            );
        }
    };
}

fn initialize_open_gl_addresses() {
    // Create module name
    let module_name = OsStr::new("opengl32.dll\0")
        .encode_wide()
        .collect::<Vec<u16>>();

    // Load module
    let module = unsafe { LoadLibraryW(module_name.as_ptr()) };

    if module.is_null() {
        logger::fatal!(
            "Could not load OpenGL module! (os error: {})",
            io::Error::last_os_error()
        );
    }

    match OPEN_GL_MODULE.lock() {
        Ok(mut module_handle_mutex) => module_handle_mutex.0 = module,
        Err(_) => logger::fatal!("Could not lock OpenGL module handle mutex!"),
    }

    open_gl::initialize_open_gl_addresses(get_open_gl_address)
}

fn get_open_gl_address(function_name: &str) -> *const std::ffi::c_void {
    // Create null-terminated function name
    let null_terminated_function_name = CString::new(function_name)
        .unwrap_or_else(|_| logger::fatal!("Could not create CString! ({})", function_name));

    // Get address (via wglGetProcAddress)
    let mut address = unsafe { wglGetProcAddress(null_terminated_function_name.as_ptr()) };

    if address.is_null()
        || address == 1 as PROC
        || address == 2 as PROC
        || address == 3 as PROC
        || address == -1_isize as PROC
    {
        // Get address (via GetProcAddress)
        address = unsafe {
            let module_handle = OPEN_GL_MODULE
                .lock()
                .unwrap_or_else(|_| logger::fatal!("Could not lock OpenGL module handle mutex!"));

            GetProcAddress(module_handle.0, null_terminated_function_name.as_ptr())
        };
    }

    if address.is_null() {
        logger::fatal!(
            "Could not get OpenGL address ({})! (os error: {})",
            function_name,
            io::Error::last_os_error()
        );
    }

    address as *const std::ffi::c_void
}

unsafe extern "system" fn window_proc(
    window: HWND,
    message: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match message {
        WM_SIZE => {
            logger::info!("window_proc: WM_SIZE");
            let mut rect = RECT::default();

            if GetClientRect(window, &mut rect) == 0 {
                logger::fatal!(
                    "Could not get client rect! (os error: {})",
                    io::Error::last_os_error()
                );
            }

            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;
            let aspect_ratio = width as f32 / height as f32;

            logger::info!(
                "WM_SIZE: width: {} / height: {} / aspect_ratio: {}",
                width,
                height,
                aspect_ratio
            );

            match ASPECT_RATIO.lock() {
                Ok(mut aspect_ratio_mutex) => *aspect_ratio_mutex = aspect_ratio,
                Err(_) => logger::fatal!("Could not lock aspect ratio mutex!"),
            }

            if INITIALIZED_OPEN_GL.load(Ordering::SeqCst) {
                // Set viewport
                gl::Viewport(0, 0, width, height);
            }
        }
        WM_DESTROY => {
            logger::info!("window_proc: WM_DESTROY");
            PostQuitMessage(0);
        }
        WM_CLOSE => {
            logger::info!("window_proc: WM_CLOSE");
            PostQuitMessage(0);
        }
        _ => (),
    };

    DefWindowProcW(window, message, w_param, l_param)
}
