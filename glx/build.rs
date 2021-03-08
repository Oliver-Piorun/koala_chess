use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::{env, fs::File, path::Path};

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(
        Api::Glx,
        (1, 4),
        Profile::Core,
        Fallbacks::All,
        ["GLX_ARB_create_context"],
    )
    .write_bindings(GlobalGenerator, &mut file)
    .unwrap();
}
