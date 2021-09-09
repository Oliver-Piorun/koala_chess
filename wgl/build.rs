use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::{env, fs::File, path::Path};

fn main() {
    let destination = env::var("OUT_DIR")
        .unwrap_or_else(|e| panic!("Could not get \"OUT_DIR\" environment variable! ({})", e));
    let mut file = File::create(&Path::new(&destination).join("bindings.rs"))
        .unwrap_or_else(|e| panic!("Could not create bindings.rs file! ({})", e));

    Registry::new(
        Api::Wgl,
        (1, 0),
        Profile::Core,
        Fallbacks::All,
        [
            "WGL_ARB_create_context",    // For wglCreateContextAttribsARB(...)
            "WGL_ARB_extensions_string", // For wglGetExtensionsStringARB(...)
        ],
    )
    .write_bindings(GlobalGenerator, &mut file)
    .unwrap_or_else(|e| panic!("Could not create bindings! ({})", e));
}
