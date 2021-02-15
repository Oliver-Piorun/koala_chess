use std::{ffi::CString, os::raw::c_int, os::raw::c_uint};
use x11::{glx::GLX_X_RENDERABLE, xlib};

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
        let mut attributes = vec![
            /* 0x0005 */ x11::glx::GLX_DOUBLEBUFFER,  true as glx::types::GLint,
            /* 0x0008 */ x11::glx::GLX_RED_SIZE,      8,
            /* 0x0009 */ x11::glx::GLX_GREEN_SIZE,    8,
            /* 0x000a */ x11::glx::GLX_BLUE_SIZE,     8,
            /* 0x000b */ x11::glx::GLX_ALPHA_SIZE,    8,
            /* 0x000c */ x11::glx::GLX_DEPTH_SIZE,    24,
            /* 0x000d */ x11::glx::GLX_STENCIL_SIZE,  8,
            /* 0x0022 */ x11::glx::GLX_X_VISUAL_TYPE, x11::glx::GLX_TRUE_COLOR,
            /* 0x8010 */ x11::glx::GLX_DRAWABLE_TYPE, x11::glx::GLX_WINDOW_BIT,
            /* 0x8011 */ x11::glx::GLX_RENDER_TYPE,   x11::glx::GLX_RGBA_BIT,
            /* 0x8012 */ x11::glx::GLX_X_RENDERABLE,  true as glx::types::GLint,
            /* 0x8000 */ x11::glx::GLX_NONE, // This has to be the last item
        ];

        // Get a visual which matches the specified attributes
        let visual = glx::ChooseVisual(
            display as *mut glx::types::Display,
            screen,
            attributes.as_mut_ptr(),
        );

        if visual.is_null() {
            // TODO: Error handling
            eprintln!("Could not get a visual which matches the specified attributes!");
            return;
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
