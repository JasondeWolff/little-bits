#![feature(thread_local)]

extern crate little_bits;
use little_bits::*;

use std::env;
use std::rc::Rc;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    init(Example::new());
}

struct Example {
    cube_model: Option<Rc<Model>>
}

impl Game for Example {
    fn new() -> Box<Example> {
        Box::new(Example {
            cube_model: None
        })
    }

    fn start(&mut self) {
        //self.cube_model = Some(app().resources().get_model(String::from("assets/wooden_chair/wooden_chair.fbx")));
        self.cube_model = Some(app().resources().get_model(String::from("assets/monkey.fbx")));
    }
    
    fn update(&mut self, delta_time: f32) {
        if (!app().input().mouse_button(MouseButton::Left)) {
            app().graphics().draw_model(self.cube_model.as_ref().unwrap().clone());
        }
    }
    
    fn stop(&mut self) {
    }
}