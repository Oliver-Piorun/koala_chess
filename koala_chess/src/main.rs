mod bitmap;

use bitmap::Bitmap;
use lazy_static::lazy_static;
use std::{
    ffi::{CString, OsStr},
    io,
    os::windows::ffi::OsStrExt,
    sync::Mutex,
};
use winapi::{
    shared::{
        minwindef::{ATOM, BOOL, HMODULE, LPARAM, LRESULT, UINT, WORD, WPARAM},
        windef::{HDC, HWND, RECT},
    },
    um::{
        libloaderapi::{GetModuleHandleW, GetProcAddress, LoadLibraryW},
        wingdi::{
            wglCreateContext, wglMakeCurrent, ChoosePixelFormat, DescribePixelFormat,
            SetPixelFormat, SwapBuffers, PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW, PFD_MAIN_PLANE,
            PFD_SUPPORT_OPENGL, PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR,
        },
        winuser::{
            BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetClientRect,
            GetDC, GetMessageW, PostQuitMessage, RegisterClassW, ReleaseDC, TranslateMessage,
            CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CW_USEDEFAULT, MSG, PAINTSTRUCT, WM_ACTIVATEAPP,
            WM_CLOSE, WM_DESTROY, WM_PAINT, WM_SIZE, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

lazy_static! {
    static ref CHESSBOARD: Mutex<Bitmap> = Mutex::new(Bitmap::default());
}

fn main() {
    *CHESSBOARD.lock().unwrap() = bitmap::load_bitmap("textures/chessboard.bmp");

    // Create window class name
    let mut window_class_name = OsStr::new("KoalaChessWindowClass\0")
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
        // TODO: Error handling
        eprintln!(
            "Could not register window class! ({})",
            io::Error::last_os_error()
        );
        return;
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

    if window.is_null() {
        // TODO: Error handling
        eprintln!("Could not create window! ({})", io::Error::last_os_error());
        return;
    }

    // Initialize OpenGL
    initialize_open_gl(window);

    // Window loop
    loop {
        let mut message = MSG::default();
        let message_result: BOOL;

        unsafe {
            message_result = GetMessageW(&mut message, std::ptr::null_mut(), 0, 0);
        }

        if message_result == -1 {
            // TODO: Error handling
            eprintln!(
                "Could not retrieve message! ({})",
                io::Error::last_os_error()
            );
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
    window: HWND,
    message: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match message {
        WM_SIZE => {
            println!("WM_SIZE");
            let mut rect = RECT::default();
            GetClientRect(window, &mut rect);
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
            let device_context = BeginPaint(window, &mut paint);
            let width = paint.rcPaint.right - paint.rcPaint.left;
            let height = paint.rcPaint.bottom - paint.rcPaint.top;

            gl::Viewport(0, 0, width, height);

            let mut texture: gl::types::GLuint = 0;

            // Generate texture
            gl::GenTextures(1, &mut texture);

            // Bind texture
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // Setup texture
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                2048,
                2048,
                0,
                gl::BGRA_EXT,
                gl::UNSIGNED_BYTE,
                CHESSBOARD.lock().unwrap().data.as_ptr() as *const std::ffi::c_void,
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::Enable(gl::TEXTURE_2D);

            gl::ClearColor(1.0, 0.0, 1.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            SwapBuffers(device_context);

            EndPaint(window, &paint);
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
}

fn negotiate_pixel_format(device_context: HDC) {
    // Create desired pixel format
    let mut desired_pixel_format = PIXELFORMATDESCRIPTOR::default();
    desired_pixel_format.nSize = std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as WORD;
    desired_pixel_format.nVersion = 1;
    desired_pixel_format.iPixelType = PFD_TYPE_RGBA;
    desired_pixel_format.dwFlags = PFD_SUPPORT_OPENGL | PFD_DRAW_TO_WINDOW | PFD_DOUBLEBUFFER;

    // RGBA
    desired_pixel_format.cColorBits = 32;

    // Alpha
    desired_pixel_format.cAlphaBits = 8;

    desired_pixel_format.iLayerType = PFD_MAIN_PLANE;

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

fn initialize_open_gl_addresses() {
    // Create module name
    let module_name = OsStr::new("opengl32.dll\0")
        .encode_wide()
        .collect::<Vec<u16>>();

    // Load module
    let module = unsafe { LoadLibraryW(module_name.as_ptr()) };

    if module.is_null() {
        // TODO: Error handling
        eprintln!("OpenGL module is null! ({})", io::Error::last_os_error());
        return;
    }

    // Get and assign addresses
    let _ = gl::Viewport::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::GenTextures::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::BindTexture::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::TexImage2D::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ =
        gl::TexParameteri::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::Enable::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::ClearColor::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::Clear::load_with(|function_name| get_open_gl_address(module, function_name));
}

fn get_open_gl_address(module: HMODULE, function_name: &str) -> *const std::ffi::c_void {
    // Create null-terminated function name
    let null_terminated_function_name = CString::new(function_name).unwrap();

    // Get address
    let address = unsafe { GetProcAddress(module, null_terminated_function_name.as_ptr()) };

    if address.is_null() {
        // TODO: Error handling
        eprintln!("OpenGL address is null! ({})", io::Error::last_os_error());
    }

    address as *const std::ffi::c_void
}
