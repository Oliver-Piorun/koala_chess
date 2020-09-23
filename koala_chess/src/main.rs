mod bitmap;
mod shader;

use bitmap::Bitmap;
use lazy_static::lazy_static;
use std::{
    ffi::{CString, OsStr},
    io,
    os::windows::ffi::OsStrExt,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
    time::SystemTime,
    time::UNIX_EPOCH,
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

lazy_static! {
    static ref ASPECT_RATIO: Mutex<f32> = Mutex::new(1.0);
    static ref INITIALIZED_OPEN_GL: AtomicBool = AtomicBool::new(false);
    static ref CHESSBOARD: Mutex<Bitmap> = Mutex::new(Bitmap::default());
}

fn main() {
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

    *CHESSBOARD.lock().unwrap() = bitmap::load_bitmap("textures/chessboard.bmp");

    let shader = shader::Shader::new("shaders/vertex.vert", "shaders/fragment.frag");

    #[rustfmt::skip]
    let vertices: [f32; 32] = [
        // positions,    colors,        texture coordinates
         0.8,  0.8, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
         0.8, -0.8, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
        -0.8, -0.8, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
        -0.8,  0.8, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
    ];

    let indices: [u32; 6] = [
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
    ];

    let mut vertex_array_object: gl::types::GLuint = 0;
    let mut vertex_buffer_object: gl::types::GLuint = 0;
    let mut element_buffer_object: gl::types::GLuint = 0;

    let mut texture: gl::types::GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_object);
        gl::GenBuffers(1, &mut vertex_buffer_object);
        gl::GenBuffers(1, &mut element_buffer_object);

        gl::BindVertexArray(vertex_array_object);

        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_object);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&indices) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        // Position attribute
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            32,
            std::ptr::null::<std::ffi::c_void>(),
        );
        gl::EnableVertexAttribArray(0);

        // Color attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            32,
            12 as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(1);

        // Texture coordinates attribute
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            32,
            24 as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(2);

        gl::Enable(gl::TEXTURE_2D);

        // Generate texture
        gl::GenTextures(1, &mut texture);

        // Bind texture
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Parameterize texture
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST as gl::types::GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::NEAREST as gl::types::GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_EDGE as gl::types::GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_EDGE as gl::types::GLint,
        );

        // Setup texture
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as gl::types::GLint,
            2048,
            2048,
            0,
            gl::BGRA_EXT,
            gl::UNSIGNED_BYTE,
            CHESSBOARD.lock().unwrap().data.as_ptr() as *const std::ffi::c_void,
        );

        // Generate mipmap
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

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
        let time = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            % 10000) as f32;

        unsafe {
            // Set the clear color
            gl::ClearColor(1.0, 0.0, 1.0, 0.0);

            // Clear the viewport with the clear color
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Bind texture
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // Use specific shader
            shader.r#use();
            shader.set_float("aspect_ratio\0", *ASPECT_RATIO.lock().unwrap());
            shader.set_float("time\0", time);

            // Bind vertex array
            gl::BindVertexArray(vertex_array_object);

            // Draw elements
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

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
        eprintln!(
            "OpenGL module is null! (os error: {})",
            io::Error::last_os_error()
        );
        return;
    }

    // Get and assign addresses

    // OpenGL <=1.1
    let _ = gl::Viewport::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::GenTextures::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::BindTexture::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::TexImage2D::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ =
        gl::TexParameteri::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::Enable::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::ClearColor::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::Clear::load_with(|function_name| get_open_gl_address(module, function_name));

    // OpenGL >1.1
    let _ = gl::CreateShader::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::ShaderSource::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ =
        gl::CompileShader::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ =
        gl::CreateProgram::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::AttachShader::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::LinkProgram::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::DeleteShader::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::UseProgram::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::GetUniformLocation::load_with(|function_name| {
        get_open_gl_address(module, function_name)
    });
    let _ = gl::Uniform1f::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::GetShaderiv::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::GetProgramiv::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ =
        gl::GetShaderInfoLog::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::GetProgramInfoLog::load_with(|function_name| {
        get_open_gl_address(module, function_name)
    });

    let _ =
        gl::GenVertexArrays::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::GenBuffers::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ =
        gl::BindVertexArray::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::BindBuffer::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::BufferData::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::VertexAttribPointer::load_with(|function_name| {
        get_open_gl_address(module, function_name)
    });
    let _ = gl::EnableVertexAttribArray::load_with(|function_name| {
        get_open_gl_address(module, function_name)
    });
    let _ =
        gl::GenerateMipmap::load_with(|function_name| get_open_gl_address(module, function_name));
    let _ = gl::DrawElements::load_with(|function_name| get_open_gl_address(module, function_name));
}

fn get_open_gl_address(module: HMODULE, function_name: &str) -> *const std::ffi::c_void {
    // Create null-terminated function name
    let null_terminated_function_name = CString::new(function_name).unwrap();

    // Get address (via wglGetProcAddress)
    let mut address = unsafe { wglGetProcAddress(null_terminated_function_name.as_ptr()) };

    if address.is_null()
        || address == 1 as PROC
        || address == 2 as PROC
        || address == 3 as PROC
        || address == -1 as isize as PROC
    {
        // Get address (via GetProcAddress)
        address = unsafe { GetProcAddress(module, null_terminated_function_name.as_ptr()) };
    }

    if address.is_null() {
        // TODO: Error handling
        eprintln!(
            "OpenGL address is null! (os error: {})",
            io::Error::last_os_error()
        );
    }

    address as *const std::ffi::c_void
}
