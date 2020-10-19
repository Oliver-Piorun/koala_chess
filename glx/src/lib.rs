#![allow(clippy::all)]
#[link(name = "GL")] extern {}
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
