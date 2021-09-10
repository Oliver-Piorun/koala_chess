use crate::game::Game;
use crate::renderer::open_gl;
use logger::*;
use std::lazy::SyncLazy;
use std::os::raw::{c_int, c_uint};
use std::sync::Mutex;
use std::{error::Error, mem::MaybeUninit};
use std::{
    ffi::{c_void, CStr, CString},
    ptr::addr_of_mut,
};
use x11::xlib;

static ASPECT_RATIO: SyncLazy<Mutex<f32>> = SyncLazy::new(|| Mutex::new(1.0));

pub fn create_window() -> (*mut xlib::Display, glx::types::Window) {
    initialize_glx_addresses();
    open_gl::initialize_open_gl_addresses(get_address);

    let display = unsafe {
        xlib::XOpenDisplay(
            std::ptr::null(), // display_name
        )
    };

    if display.is_null() {
        fatal!("Could not open display!");
    }

    let mut major_glx: glx::types::GLint = 0;
    let mut minor_glx: glx::types::GLint = 0;

    unsafe {
        // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXQueryVersion.xml
        glx::QueryVersion(
            display as *mut glx::types::Display, // dpy
            &mut major_glx,                      // major
            &mut minor_glx,                      // minor
        );
    }

    info!("GLX version: {}.{}", major_glx, minor_glx);

    unsafe {
        // Reference: https://tronche.com/gui/x/xlib/display/display-macros.html#DefaultScreen
        let screen_id = xlib::XDefaultScreen(
            display, // display
        );

        // Reference: https://tronche.com/gui/x/xlib/display/display-macros.html#RootWindow
        let root = xlib::XRootWindow(
            display,   // display
            screen_id, // screen_number
        );

        #[rustfmt::skip]
        let framebuffer_attributes = vec![
            /* 0x0005 */ glx::DOUBLEBUFFER as glx::types::GLint,  true as glx::types::GLint,
            /* 0x0008 */ glx::RED_SIZE as glx::types::GLint,      8,
            /* 0x0009 */ glx::GREEN_SIZE as glx::types::GLint,    8,
            /* 0x000a */ glx::BLUE_SIZE as glx::types::GLint,     8,
            /* 0x000b */ glx::ALPHA_SIZE as glx::types::GLint,    8,
            /* 0x000c */ glx::DEPTH_SIZE as glx::types::GLint,    24,
            /* 0x000d */ glx::STENCIL_SIZE as glx::types::GLint,  8,
            /* 0x0022 */ glx::X_VISUAL_TYPE as glx::types::GLint, glx::TRUE_COLOR as glx::types::GLint,
            /* 0x8010 */ glx::DRAWABLE_TYPE as glx::types::GLint, glx::WINDOW_BIT as glx::types::GLint,
            /* 0x8011 */ glx::RENDER_TYPE as glx::types::GLint,   glx::RGBA_BIT as glx::types::GLint,
            /* 0x8012 */ glx::X_RENDERABLE as glx::types::GLint,  true as glx::types::GLint,
            /* 0x8000 */ glx::NONE as glx::types::GLint, // This has to be the last item
        ];

        let mut framebuffer_count = 0;

        // Get framebuffer configs which match the specified attributes
        // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXChooseFBConfig.xml
        let framebuffer_configs: *mut glx::types::GLXFBConfig = glx::ChooseFBConfig(
            display as *mut glx::types::Display, // dpy
            screen_id,                           // screen
            framebuffer_attributes.as_ptr(),     // attrib_list
            &mut framebuffer_count,              // nelements
        );

        if framebuffer_count == 0 {
            fatal!("Could not get framebuffer configs which satisfy the specified attributes!");
        }

        // Find the best framebuffer config
        let mut best_framebuffer_config_index = Option::<isize>::None;
        let mut max_num_samples = -1;

        for i in 0..framebuffer_count as isize {
            let framebuffer_config = *framebuffer_configs.offset(i);

            // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXGetVisualFromFBConfig.xml
            let visual_info = glx::GetVisualFromFBConfig(
                display as *mut glx::types::Display, // dpy
                framebuffer_config,                  // config
            );

            if visual_info.is_null() {
                continue;
            }

            let mut num_sample_buffers = 0;
            let mut num_samples = 0;

            // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXGetFBConfigAttrib.xml
            glx::GetFBConfigAttrib(
                display as *mut glx::types::Display, // dpy
                framebuffer_config,                  // config
                glx::SAMPLE_BUFFERS as i32,          // attribute
                &mut num_sample_buffers as *mut i32, // value
            );
            glx::GetFBConfigAttrib(
                display as *mut glx::types::Display, // dpy
                framebuffer_config,                  // config
                glx::SAMPLES as i32,                 // attribute
                &mut num_samples as *mut i32,        // value
            );

            if num_sample_buffers > 0 && num_samples > max_num_samples {
                best_framebuffer_config_index = Some(i);
                max_num_samples = num_samples;
            }

            // Free visual info
            xlib::XFree(visual_info as *mut c_void);
        }

        let best_framebuffer_config_index = best_framebuffer_config_index
            .unwrap_or_else(|| fatal!("Could not find the best framebuffer config!"));

        // Get the best framebuffer config
        let best_framebuffer_config = *framebuffer_configs.offset(best_framebuffer_config_index);

        // Free framebuffer configs
        xlib::XFree(framebuffer_configs as *mut c_void);

        // Get a visual info from the framebuffer config
        // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXGetVisualFromFBConfig.xml
        let visual_info = glx::GetVisualFromFBConfig(
            display as *mut glx::types::Display, // dpy
            best_framebuffer_config,             // config
        );

        if visual_info.is_null() {
            fatal!("Could not get a visual info from the framebuffer config!");
        }

        // Flush the output buffer and wait until all request have been received and processed by the X server
        // Reference: https://tronche.com/gui/x/xlib/event-handling/XSync.html
        xlib::XSync(
            display,     // display
            xlib::False, // discard
        );

        let mut attributes = {
            let mut attributes_uninit: MaybeUninit<xlib::XSetWindowAttributes> =
                MaybeUninit::uninit();
            let ptr = attributes_uninit.as_mut_ptr();

            addr_of_mut!((*ptr).border_pixel).write(xlib::XBlackPixel(display, screen_id));
            addr_of_mut!((*ptr).background_pixel).write(xlib::XWhitePixel(display, screen_id));
            addr_of_mut!((*ptr).override_redirect).write(xlib::True);
            addr_of_mut!((*ptr).colormap).write(xlib::XCreateColormap(
                display,
                root,
                (*visual_info).visual as *mut xlib::Visual,
                xlib::AllocNone,
            ));
            addr_of_mut!((*ptr).event_mask).write(xlib::ExposureMask);

            attributes_uninit.assume_init()
        };

        // Create window
        // Reference: https://tronche.com/gui/x/xlib/window/XCreateWindow.html
        let window = xlib::XCreateWindow(
            display,                                                                        // display
            root,                                       // parent
            0,                                          // x
            0,                                          // y
            400,                                        // width
            300,                                        // height
            0,                                          // border_width
            (*visual_info).depth,                       // depth
            xlib::InputOutput as c_uint,                // class
            (*visual_info).visual as *mut xlib::Visual, // visual
            xlib::CWBackPixel | xlib::CWColormap | xlib::CWBorderPixel | xlib::CWEventMask, // valuemask
            &mut attributes, // attributes
        );

        // Create window name
        let window_name =
            CString::new("Koala chess").unwrap_or_else(|_| fatal!("Could not create CString!"));

        // Set window name
        xlib::XStoreName(display, window, window_name.as_ptr());

        // Reference: https://tronche.com/gui/x/xlib/window/XMapWindow.html
        xlib::XMapWindow(
            display, // display
            window,  // w
        );

        // Initialize OpenGL
        initialize_open_gl(display, screen_id, best_framebuffer_config, window);

        (display, window)
    }
}

pub fn r#loop(display: *mut xlib::Display, window: u64, game: &mut Game) {
    unsafe {
        let wm_protocols_str =
            CString::new("WM_PROTOCOLS").unwrap_or_else(|_| fatal!("Could not create CString!"));
        let wm_delete_window_str = CString::new("WM_DELETE_WINDOW")
            .unwrap_or_else(|_| fatal!("Could not create CString!"));

        let wm_protocols = xlib::XInternAtom(
            display,                   // display
            wm_protocols_str.as_ptr(), // atom_name
            xlib::False,               // only_if_exists
        );
        let wm_delete_window = xlib::XInternAtom(
            display,                       // display
            wm_delete_window_str.as_ptr(), // atom_name
            xlib::False,                   // only_if_exists
        );

        let mut protocols = [wm_delete_window];

        // Reference: https://tronche.com/gui/x/xlib/ICC/client-to-window-manager/XSetWMProtocols.html
        xlib::XSetWMProtocols(
            display,                  // display
            window,                   // w
            protocols.as_mut_ptr(),   // protocols
            protocols.len() as c_int, // count
        );

        let mut event = {
            let event_uninit = MaybeUninit::uninit();

            event_uninit.assume_init()
        };

        let mut last_time = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };

        if libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut last_time) == -1 {
            fatal!("clock_gettime failed!");
        }

        'outer: loop {
            // Window loop
            while { xlib::XPending(display) } > 0 {
                xlib::XNextEvent(display, &mut event);

                if event.type_ == xlib::Expose {
                    let mut attributes = {
                        let attributes_uninit = MaybeUninit::uninit();

                        attributes_uninit.assume_init()
                    };
                    xlib::XGetWindowAttributes(display, window, &mut attributes);
                    let width = attributes.width;
                    let height = attributes.height;
                    let aspect_ratio = width as f32 / height as f32;
                    info!(
                        "Expose: width: {} / height: {} / aspect_ratio: {}",
                        width, height, aspect_ratio
                    );

                    *ASPECT_RATIO
                        .lock()
                        .unwrap_or_else(|e| fatal!("Could not lock aspect ratio mutex! ({})", e)) =
                        aspect_ratio;

                    // Set viewport
                    gl::Viewport(0, 0, width, height);
                }

                if let xlib::ClientMessage = event.get_type() {
                    let xclient = xlib::XClientMessageEvent::from(event);

                    if xclient.message_type == wm_protocols && xclient.format == 32 {
                        let protocol = xclient.data.get_long(0) as xlib::Atom;

                        if protocol == wm_delete_window {
                            break 'outer;
                        }
                    }
                }
            }

            // Rendering
            let aspect_ratio = *ASPECT_RATIO
                .lock()
                .unwrap_or_else(|e| fatal!("Could not lock aspect ratio mutex! ({})", e));

            // Draw game
            if let Err(e) = game.draw(aspect_ratio) {
                error!("{}", e);
            }

            glx::SwapBuffers(display as *mut glx::types::Display, window);

            // Metrics
            let mut end_time = libc::timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };

            if libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut end_time) == -1 {
                fatal!("clock_gettime failed!");
            }

            // ms = end time - last time
            let elapsed_milliseconds = end_time.tv_sec as f64 * 1_000f64
                + end_time.tv_nsec as f64 * 1e-6
                - last_time.tv_sec as f64 * 1_000f64
                - last_time.tv_nsec as f64 * 1e-6;

            // 1/s = 1000 / elapsed milliseconds
            let frames_per_second = 1_000f64 / elapsed_milliseconds;

            println!(
                "frames per second: {} / frame time: {}ms",
                frames_per_second, elapsed_milliseconds
            );

            last_time = end_time;
        }
    }
}

fn initialize_open_gl(
    display: *mut xlib::Display,
    screen_id: i32,
    framebuffer_config: glx::types::GLXFBConfig,
    window: glx::types::Window,
) {
    let rendering_context;

    let extension_supported = unsafe {
        is_extension_supported(
            "GLX_ARB_create_context",
            display as *mut glx::types::Display,
            screen_id,
        )
    }
    .unwrap_or(false);

    if !extension_supported {
        // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXCreateNewContext.xml
        rendering_context = unsafe {
            glx::CreateNewContext(
                display as *mut glx::types::Display, // dpy
                framebuffer_config,                  // config
                glx::RGBA_TYPE as i32,               // render_type
                std::ptr::null::<c_void>(),          // share_list
                true as i32,                         // direct
            )
        };
    } else {
        #[rustfmt::skip]
        let context_attributes = vec![
            glx::CONTEXT_MAJOR_VERSION_ARB as glx::types::GLint, 3,
            glx::CONTEXT_MINOR_VERSION_ARB as glx::types::GLint, 2,
            0, // This has to be the last item
        ];

        // Reference: https://www.khronos.org/registry/OpenGL/extensions/ARB/GLX_ARB_create_context.txt
        rendering_context = unsafe {
            glx::CreateContextAttribsARB(
                display as *mut glx::types::Display, // dpy
                framebuffer_config,                  // config
                0 as glx::types::GLXContext,         // share_context
                true as glx::types::Bool,            // direct
                context_attributes.as_ptr(),         // attrib_list
            )
        };
    }

    if rendering_context.is_null() {
        fatal!("Could not create OpenGL rendering context!");
    }

    // Check if we obtained a direct context
    // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXIsDirect.xml
    if unsafe {
        glx::IsDirect(
            display as *mut glx::types::Display, // dpy
            rendering_context,                   // ctx
        )
    } == false as glx::types::Bool
    {
        fatal!("Created context is not a direct context!");
    }

    // Make context the current GLX rendering context of the calling thread and attach the context to the window
    // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXMakeCurrent.xml
    unsafe {
        glx::MakeCurrent(
            display as *mut glx::types::Display, // dpy
            window,                              // drawable
            rendering_context,                   // ctx
        )
    };

    let vendor_cstr = unsafe { CStr::from_ptr(gl::GetString(gl::VENDOR) as *mut i8) };
    let vendor = vendor_cstr
        .to_str()
        .unwrap_or_else(|e| fatal!("Could not create &str! ({})", e));
    info!("GL vendor: {}", vendor);

    let renderer_cstr = unsafe { CStr::from_ptr(gl::GetString(gl::RENDERER) as *mut i8) };
    let renderer = renderer_cstr
        .to_str()
        .unwrap_or_else(|e| fatal!("Could not create &str! ({})", e));
    info!("GL renderer: {}", renderer);

    let version_cstr = unsafe { CStr::from_ptr(gl::GetString(gl::VERSION) as *mut i8) };
    let version = version_cstr
        .to_str()
        .unwrap_or_else(|e| fatal!("Could not create &str! ({})", e));
    info!("GL version: {}", version);
}

fn initialize_glx_addresses() {
    // Get and assign addresses
    let _ = glx::GetProcAddress::load_with(|function_name| unsafe {
        // Create null-terminated function name
        let null_terminated_function_name = CString::new(function_name)
            .unwrap_or_else(|_| fatal!("Could not create CString! ({})", function_name));

        // TODO: Don't use the x11 library to get the address of GetProcAddress but rather something like this:
        // https://stackoverflow.com/questions/38674176/manually-calling-opengl-functions
        x11::glx::glXGetProcAddress(
            null_terminated_function_name.as_ptr() as *const glx::types::GLubyte
        )
        .unwrap_or_else(|| fatal!("Could not get address! ({})", function_name))
            as *const std::ffi::c_void
    });
    let _ = glx::QueryVersion::load_with(|function_name| get_address(function_name));
    let _ = glx::ChooseFBConfig::load_with(|function_name| get_address(function_name));
    let _ = glx::GetVisualFromFBConfig::load_with(|function_name| get_address(function_name));
    let _ = glx::GetFBConfigAttrib::load_with(|function_name| get_address(function_name));
    let _ = glx::QueryExtensionsString::load_with(|function_name| get_address(function_name));
    let _ = glx::CreateNewContext::load_with(|function_name| get_address(function_name));
    let _ = glx::CreateContextAttribsARB::load_with(|function_name| get_address(function_name));
    let _ = glx::IsDirect::load_with(|function_name| get_address(function_name));
    let _ = glx::MakeCurrent::load_with(|function_name| get_address(function_name));
    let _ = glx::SwapBuffers::load_with(|function_name| get_address(function_name));
}

fn get_address(function_name: &str) -> *const std::ffi::c_void {
    // Create null-terminated function name
    let null_terminated_function_name = CString::new(function_name)
        .unwrap_or_else(|_| fatal!("Could not create CString! ({})", function_name));

    // Get address (via glXGetProcAddress)
    let address = unsafe {
        // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXGetProcAddress.xml
        glx::GetProcAddress(
            null_terminated_function_name.as_ptr() as *const glx::types::GLubyte, // proc_name
        )
    };

    if address.is_null() {
        fatal!("Could not get address! ({})", function_name);
    }

    address as *const std::ffi::c_void
}

unsafe fn is_extension_supported(
    extension: &str,
    display: *mut glx::types::Display,
    screen: glx::types::GLint,
) -> Result<bool, Box<dyn Error>> {
    // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXQueryExtensionsString.xml
    let query_extensions_string_raw = glx::QueryExtensionsString(
        display as *mut glx::types::Display, // dpy
        screen,                              // screen
    );

    let query_extensions_string_cstring = CString::from_raw(query_extensions_string_raw as *mut i8);
    let query_extensions_string_str = query_extensions_string_cstring.to_str()?;

    Ok(query_extensions_string_str.contains(extension))
}
