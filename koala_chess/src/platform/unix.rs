use std::{ffi::CString, os::raw::c_int, os::raw::c_uint};
use x11::xlib;

pub fn create_window() {
    let display = unsafe { xlib::XOpenDisplay(std::ptr::null()) };

    if display.is_null() {
        // TODO: Error handling
        eprintln!("Could not open display!");
        return;
    }

    unsafe {
        let screen = xlib::XDefaultScreen(display);
        let root = xlib::XRootWindow(display, screen);

        let mut attributes: xlib::XSetWindowAttributes =
            std::mem::MaybeUninit::uninit().assume_init();
        attributes.background_pixel = xlib::XWhitePixel(display, screen);

        let window = xlib::XCreateWindow(
            display,
            root,
            0,
            0,
            400,
            300,
            0,
            0,
            xlib::InputOutput as c_uint,
            std::ptr::null_mut(),
            xlib::CWBackPixel,
            &mut attributes,
        );

        let title_str = CString::new("Koala chess").unwrap();
        xlib::XStoreName(display, window, title_str.as_ptr());

        let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
        let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();

        let wm_protocols = xlib::XInternAtom(display, wm_protocols_str.as_ptr(), xlib::False);
        let wm_delete_window =
            xlib::XInternAtom(display, wm_delete_window_str.as_ptr(), xlib::False);

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
