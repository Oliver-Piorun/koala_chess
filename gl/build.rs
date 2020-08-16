use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::{env, fs::File, path::Path};

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(
        Api::Gles2,
        (3, 2),
        Profile::Core,
        Fallbacks::All,
        ["GL_EXT_texture_format_BGRA8888"],
    )
    .write_bindings(GlobalGenerator, &mut file)
    .unwrap();
}
