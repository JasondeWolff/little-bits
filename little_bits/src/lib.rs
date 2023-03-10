#![crate_type = "lib"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![crate_name = "little_bits"]
#![feature(thread_local)]
#![feature(panic_info_message)]
#![allow(dead_code)]

extern crate gmaths;
pub use gmaths::*;

#[path = "system.rs"] pub mod system;
pub use system::*;

#[path = "application.rs"] pub mod application;
pub use application::*;

#[path = "common/shared.rs"] pub mod shared;
pub use shared::*;

#[thread_local]
static mut APP: Option<Box<Application>> = None;

pub fn init<G: Game + 'static>(game: Box<G>) {
    unsafe {
        APP = Some(Application::new(game));
        APP.as_mut().unwrap().start();
    }
}

pub fn app() -> &'static mut Application {
    unsafe {
        APP.as_mut().expect("Failed to get app.")
    }
}