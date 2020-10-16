mod bitmap;
mod board;
mod game;
mod piece;
mod platform;
mod shader;
mod traits;

use game::Game;
use lazy_static::lazy_static;
use std::{
    ffi::{CString, OsStr},
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};
use traits::Draw;

lazy_static! {
    static ref ASPECT_RATIO: Mutex<f32> = Mutex::new(1.0);
    static ref INITIALIZED_OPEN_GL: AtomicBool = AtomicBool::new(false);
}

fn main() {
    platform::windows::create_window();

    Game::initialize();
    let game = Game::new();

    // let device_context = unsafe { GetDC(window) };

    // // The frequency of the performance counter is fixed at system boot and is consistent across all processors
    // let mut performance_frequency = LARGE_INTEGER::default();
    // unsafe { QueryPerformanceFrequency(&mut performance_frequency) };

    // let mut last_performance_counter = LARGE_INTEGER::default();
    // unsafe { QueryPerformanceCounter(&mut last_performance_counter) };

    // let mut running = true;

    // while running {
    //     let mut message = MSG::default();

    //     // Window loop
    //     while unsafe { PeekMessageW(&mut message, std::ptr::null_mut(), 0, 0, PM_REMOVE) } != 0 {
    //         if message.message == WM_QUIT {
    //             println!("window_proc: WM_QUIT");
    //             running = false;
    //             break;
    //         }

    //         unsafe {
    //             // INFO: These calls could fail, but we can't really handle those fails
    //             TranslateMessage(&message);
    //             DispatchMessageW(&message);
    //         }
    //     }

    //     // Rendering
    //     unsafe {
    //         // Set the clear color
    //         gl::ClearColor(0.17, 0.32, 0.59, 0.0);

    //         // Clear the viewport with the clear color
    //         gl::Clear(gl::COLOR_BUFFER_BIT);

    //         // Draw game
    //         game.draw(*ASPECT_RATIO.lock().unwrap());

    //         SwapBuffers(device_context);
    //     }

    //     let mut end_performance_counter = LARGE_INTEGER::default();
    //     unsafe { QueryPerformanceCounter(&mut end_performance_counter) };

    //     let elapsed_performance_counter =
    //         unsafe { end_performance_counter.QuadPart() - last_performance_counter.QuadPart() };

    //     // ms = 1000 * counter / (counter / s) = 1000 * counter * (s / counter)
    //     let elapsed_milliseconds = 1000f64 * elapsed_performance_counter as f64
    //         / unsafe { *performance_frequency.QuadPart() as f64 };

    //     // 1/s = (counter / s) / counter
    //     let frames_per_second = unsafe { *performance_frequency.QuadPart() as f64 }
    //         / elapsed_performance_counter as f64;

    //     println!(
    //         "frames per second: {} / frame time: {}ms",
    //         frames_per_second, elapsed_milliseconds
    //     );

    //     last_performance_counter = end_performance_counter;
    // }
}

// unsafe extern "system" fn window_proc(
//     window: HWND,
//     message: UINT,
//     w_param: WPARAM,
//     l_param: LPARAM,
// ) -> LRESULT {
//     match message {
//         WM_SIZE => {
//             println!("window_proc: WM_SIZE");
//             let mut rect = RECT::default();
//             GetClientRect(window, &mut rect);
//             let width = rect.right - rect.left;
//             let height = rect.bottom - rect.top;
//             let aspect_ratio = width as f32 / height as f32;
//             println!(
//                 "WM_SIZE: width: {} / height: {} / aspect_ratio: {}",
//                 width, height, aspect_ratio
//             );

//             *ASPECT_RATIO.lock().unwrap() = aspect_ratio;

//             if INITIALIZED_OPEN_GL.load(Ordering::SeqCst) {
//                 // Set viewport
//                 gl::Viewport(0, 0, width, height);
//             }
//         }
//         WM_DESTROY => {
//             println!("window_proc: WM_DESTROY");
//             PostQuitMessage(0);
//         }
//         WM_CLOSE => {
//             println!("window_proc: WM_CLOSE");
//             PostQuitMessage(0);
//         }
//         _ => (),
//     };

//     DefWindowProcW(window, message, w_param, l_param)
// }

// fn initialize_open_gl_addresses() {
//     // Create module name
//     let module_name = OsStr::new("opengl32.dll\0")
//         .encode_wide()
//         .collect::<Vec<u16>>();

//     // Load module
//     let module = unsafe { LoadLibraryW(module_name.as_ptr()) };

//     if module.is_null() {
//         // TODO: Error handling
//         eprintln!(
//             "OpenGL module is null! (os error: {})",
//             io::Error::last_os_error()
//         );
//         return;
//     }

//     // Get and assign addresses

//     // OpenGL <=1.1
//     let _ = gl::Viewport::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::GenTextures::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::BindTexture::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::TexImage2D::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ =
//         gl::TexParameteri::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::Enable::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::ClearColor::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::Clear::load_with(|function_name| get_open_gl_address(module, function_name));

//     // OpenGL >1.1
//     let _ = gl::CreateShader::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::ShaderSource::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ =
//         gl::CompileShader::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ =
//         gl::CreateProgram::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::AttachShader::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::LinkProgram::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::DeleteShader::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::UseProgram::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::GetUniformLocation::load_with(|function_name| {
//         get_open_gl_address(module, function_name)
//     });
//     let _ = gl::Uniform1f::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::GetShaderiv::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::GetProgramiv::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ =
//         gl::GetShaderInfoLog::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::GetProgramInfoLog::load_with(|function_name| {
//         get_open_gl_address(module, function_name)
//     });

//     let _ =
//         gl::GenVertexArrays::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::GenBuffers::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ =
//         gl::BindVertexArray::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::BindBuffer::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::BufferData::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::VertexAttribPointer::load_with(|function_name| {
//         get_open_gl_address(module, function_name)
//     });
//     let _ = gl::EnableVertexAttribArray::load_with(|function_name| {
//         get_open_gl_address(module, function_name)
//     });
//     let _ =
//         gl::GenerateMipmap::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::DrawElements::load_with(|function_name| get_open_gl_address(module, function_name));
//     let _ = gl::BlendFunc::load_with(|function_name| get_open_gl_address(module, function_name));
// }

// fn get_open_gl_address(module: HMODULE, function_name: &str) -> *const std::ffi::c_void {
//     // Create null-terminated function name
//     let null_terminated_function_name = CString::new(function_name).unwrap();

//     // Get address (via wglGetProcAddress)
//     let mut address = unsafe { wglGetProcAddress(null_terminated_function_name.as_ptr()) };

//     if address.is_null()
//         || address == 1 as PROC
//         || address == 2 as PROC
//         || address == 3 as PROC
//         || address == -1 as isize as PROC
//     {
//         // Get address (via GetProcAddress)
//         address = unsafe { GetProcAddress(module, null_terminated_function_name.as_ptr()) };
//     }

//     if address.is_null() {
//         // TODO: Error handling
//         eprintln!(
//             "OpenGL address is null! (os error: {})",
//             io::Error::last_os_error()
//         );
//     }

//     address as *const std::ffi::c_void
// }
