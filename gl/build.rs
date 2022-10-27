use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::{env, fs::File, path::Path};

fn main() {
    let destination = env::var("OUT_DIR")
        .unwrap_or_else(|e| panic!("Could not get \"OUT_DIR\" environment variable! ({e})"));
    let mut file = File::create(Path::new(&destination).join("bindings.rs"))
        .unwrap_or_else(|e| panic!("Could not create bindings.rs file! ({e})"));

    Registry::new(
        Api::Gles2,
        (3, 2),
        Profile::Core,
        Fallbacks::All,
        [
            "GL_EXT_texture_format_BGRA8888", // For GL_BGRA_EXT
        ],
    )
    .write_bindings(GlobalGenerator, &mut file)
    .unwrap_or_else(|e| panic!("Could not create bindings! ({e})"));
}
