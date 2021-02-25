use std::{
    ffi::{c_void, CString},
    os::raw::{c_int, c_uint},
};
use x11::{glx::arb::GLX_CONTEXT_MINOR_VERSION_ARB, xlib};

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
        let screen = xlib::XDefaultScreen(display);

        #[rustfmt::skip]
        let attributes = vec![
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
            display as *mut glx::types::Display,
            screen,
            attributes.as_ptr(),
            &mut framebuffer_count,
        );

        if framebuffer_count == 0 {
            // TODO: Error handling
            eprintln!("Could not get a framebuffer config which matches the specified attributes!");
            return;
        }

        // Get the first framebuffer config
        let framebuffer_config = *framebuffer_configs;

        // Get a visual from the framebuffer config
        // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXGetVisualFromFBConfig.xml
        let visual =
            glx::GetVisualFromFBConfig(display as *mut glx::types::Display, framebuffer_config);

        if visual.is_null() {
            // TODO: Error handling
            eprintln!("Could not get a visual from the framebuffer config!");
            return;
        }

        let context: glx::types::GLXContext;

        if !is_extension_supported(
            "GLX_ARB_create_context",
            display as *mut glx::types::Display,
            screen,
        ) {
            // Reference: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glXCreateNewContext.xml
            context = glx::CreateNewContext(
                display as *mut glx::types::Display, // dpy
                framebuffer_config,                  // config
                glx::RGBA_TYPE as i32,               // render_type
                0 as *const c_void,                  // share_list
                true as i32,                         // direct
            );
        } else {
            #[rustfmt::skip]
            let context_attributes = vec![
                glx::CONTEXT_MAJOR_VERSION_ARB as glx::types::GLint, 3,
                glx::CONTEXT_MINOR_VERSION_ARB as glx::types::GLint, 2,
                glx::CONTEXT_FLAGS_ARB as glx::types::GLint,         glx::CONTEXT_FORWARD_COMPATIBLE_BIT_ARB as glx::types::GLint,
                glx::CONTEXT_PROFILE_MASK_ARB as glx::types::GLint,  glx::CONTEXT_CORE_PROFILE_BIT_ARB as glx::types::GLint,
                glx::NONE as glx::types::GLint, // This has to be the last item
            ];

            // Reference: https://www.khronos.org/registry/OpenGL/extensions/ARB/GLX_ARB_create_context.txt
            context = glx::CreateContextAttribsARB(
                display as *mut glx::types::Display, // dpy
                framebuffer_config,                  // config
                0 as glx::types::GLXContext,         // share_context
                true as glx::types::Bool,            // direct
                context_attributes.as_ptr(),         // attrib_list
            );
        }

        let root = xlib::XRootWindow(display, screen);

        let mut attributes: xlib::XSetWindowAttributes =
            std::mem::MaybeUninit::uninit().assume_init();
        attributes.background_pixel = xlib::XWhitePixel(display, screen);

        // Create window
        let window = xlib::XCreateWindow(
            display,                     // display
            root,                        // parent
            0,                           // x
            0,                           // y
            400,                         // width
            300,                         // height
            0,                           // border_width
            0,                           // depth
            xlib::InputOutput as c_uint, // class
            std::ptr::null_mut(),        // visual
            xlib::CWBackPixel,           // valuemask
            &mut attributes,             // attributes
        );

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

        xlib::XCloseDisplay(display);
    }
}

unsafe fn is_extension_supported(
    extension: &str,
    display: *mut glx::types::Display,
    screen: glx::types::GLint,
) -> bool {
    let query_extension_string_raw =
        glx::QueryExtensionsString(display as *mut glx::types::Display, screen);
    let query_extension_string_cstring =
        std::ffi::CString::from_raw(query_extension_string_raw as *mut i8);
    let query_extension_string_str = query_extension_string_cstring.to_str().unwrap();

    query_extension_string_str.contains(extension)
}
