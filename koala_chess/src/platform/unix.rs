use std::{
    ffi::{c_void, CString},
    os::raw::{c_int, c_uint},
};
use x11::xlib;

pub fn create_window() {
    let display = unsafe {
        xlib::XOpenDisplay(
            std::ptr::null(), // display_name
        )
    };

    if display.is_null() {
        // TODO: Error handling
        eprintln!("Could not open display!");
        return;
    }

    let mut major_glx: glx::types::GLint = 0;
    let mut minor_glx: glx::types::GLint = 0;

    unsafe {
        // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXQueryVersion.xml
        glx::QueryVersion(
            display as *mut glx::types::Display,
            &mut major_glx,
            &mut minor_glx,
        );
    }

    println!("GLX version: {}.{}", major_glx, minor_glx);

    unsafe {
        let screen_id = xlib::XDefaultScreen(display);
        let root = xlib::XRootWindow(display, screen_id);

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
            // TODO: Error handling
            eprintln!("Could not get a framebuffer config which matches the specified attributes!");
            return;
        }

        // Find the best framebuffer config
        let mut best_framebuffer_config_index = Option::<isize>::None;
        let mut max_num_samples = -1;

        for i in 0..framebuffer_count as isize {
            let framebuffer_config = *framebuffer_configs.offset(i);
            let visual_info =
                glx::GetVisualFromFBConfig(display as *mut glx::types::Display, framebuffer_config);

            if visual_info.is_null() {
                continue;
            }

            let mut num_sample_buffers = 0;
            let mut num_samples = 0;

            glx::GetFBConfigAttrib(
                display as *mut glx::types::Display,
                framebuffer_config,
                glx::SAMPLE_BUFFERS as i32,
                &mut num_sample_buffers as *mut i32,
            );
            glx::GetFBConfigAttrib(
                display as *mut glx::types::Display,
                framebuffer_config,
                glx::SAMPLES as i32,
                &mut num_samples as *mut i32,
            );

            if num_sample_buffers > 0 && num_samples > max_num_samples {
                best_framebuffer_config_index = Some(i);
                max_num_samples = num_samples;
            }

            // Free visual info
            xlib::XFree(visual_info as *mut c_void);
        }

        if best_framebuffer_config_index.is_none() {
            // Error handling
            eprintln!("Could not find the best framebuffer config!");
            return;
        }

        // Get the best framebuffer config
        let best_framebuffer_config =
            *framebuffer_configs.offset(best_framebuffer_config_index.unwrap());

        // Free framebuffer configs
        xlib::XFree(framebuffer_configs as *mut c_void);

        // Get a visual info from the framebuffer config
        // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXGetVisualFromFBConfig.xml
        let visual_info = glx::GetVisualFromFBConfig(
            display as *mut glx::types::Display, // dpy
            best_framebuffer_config,             // config
        );

        if visual_info.is_null() {
            // TODO: Error handling
            eprintln!("Could not get a visual info from the framebuffer config!");
            return;
        }

        let mut context: glx::types::GLXContext = std::ptr::null_mut();

        if !is_extension_supported(
            "GLX_ARB_create_context",
            display as *mut glx::types::Display,
            screen_id,
        ) {
            // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXCreateNewContext.xml
            context = glx::CreateNewContext(
                display as *mut glx::types::Display, // dpy
                best_framebuffer_config,             // config
                glx::RGBA_TYPE as i32,               // render_type
                std::ptr::null::<c_void>(),          // share_list
                true as i32,                         // direct
            );
        } else {
            #[rustfmt::skip]
            let context_attributes = vec![
                glx::CONTEXT_MAJOR_VERSION_ARB as glx::types::GLint, 3,
                glx::CONTEXT_MINOR_VERSION_ARB as glx::types::GLint, 2,
                0, // This has to be the last item
            ];

            // Reference: https://www.khronos.org/registry/OpenGL/extensions/ARB/GLX_ARB_create_context.txt
            context = glx::CreateContextAttribsARB(
                display as *mut glx::types::Display, // dpy
                best_framebuffer_config,             // config
                0 as glx::types::GLXContext,         // share_context
                true as glx::types::Bool,            // direct
                context_attributes.as_ptr(),         // attrib_list
            );
        }

        if context.is_null() {
            // TODO: Error handling
            eprintln!("Could not create a context!");
            return;
        }

        // Flush the output buffer and wait until all request have been received and processed by the X server
        // Reference: https://tronche.com/gui/x/xlib/event-handling/XSync.html
        xlib::XSync(display, xlib::False);

        // Check if we obtained a direct context
        if glx::IsDirect(display as *mut glx::types::Display, context) == false as glx::types::Bool
        {
            // TODO: Error handling
            eprintln!("Created context is not a direct context!");
            return;
        }

        let mut attributes: xlib::XSetWindowAttributes =
            std::mem::MaybeUninit::uninit().assume_init();
        attributes.border_pixel = xlib::XBlackPixel(display, screen_id);
        attributes.background_pixel = xlib::XWhitePixel(display, screen_id);
        attributes.override_redirect = xlib::True;
        attributes.colormap = xlib::XCreateColormap(
            display,
            root,
            (*visual_info).visual as *mut xlib::Visual,
            xlib::AllocNone,
        );
        attributes.event_mask = xlib::ExposureMask;

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

        // Make context the current GLX rendering context of the calling thread and attach the context to the window
        // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXMakeCurrent.xml
        glx::MakeCurrent(display as *mut glx::types::Display, window, context);

        // Create window name
        let window_name = CString::new("Koala chess").unwrap();

        // Set window name
        xlib::XStoreName(display, window, window_name.as_ptr());

        let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
        let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();

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

        xlib::XSetWMProtocols(
            display,
            window,
            protocols.as_mut_ptr(),
            protocols.len() as c_int,
        );

        xlib::XMapWindow(display, window);

        let mut event: xlib::XEvent = std::mem::MaybeUninit::uninit().assume_init();

        loop {
            xlib::XNextEvent(display, &mut event);

            match event.get_type() {
                xlib::ClientMessage => {
                    let xclient = xlib::XClientMessageEvent::from(event);

                    if xclient.message_type == wm_protocols && xclient.format == 32 {
                        let protocol = xclient.data.get_long(0) as xlib::Atom;

                        if protocol == wm_delete_window {
                            break;
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

unsafe fn is_extension_supported(
    extension: &str,
    display: *mut glx::types::Display,
    screen: glx::types::GLint,
) -> bool {
    // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXQueryExtensionsString.xml
    let query_extension_string_raw = glx::QueryExtensionsString(
        display as *mut glx::types::Display, // dpy
        screen,                              // screen
    );

    let query_extension_string_cstring =
        std::ffi::CString::from_raw(query_extension_string_raw as *mut i8);
    let query_extension_string_str = query_extension_string_cstring.to_str().unwrap();

    query_extension_string_str.contains(extension)
}
