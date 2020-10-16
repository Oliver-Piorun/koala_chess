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

pub fn create_window() {
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
            "Could not register window class! (os error: {})",
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
        eprintln!(
            "Could not create window! (os error: {})",
            io::Error::last_os_error()
        );
        return;
    }

    // Initialize OpenGL
    initialize_open_gl(window);
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
