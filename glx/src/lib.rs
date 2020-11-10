#![allow(clippy::all)]
#[link(name = "GL")]
extern "C" {}
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
