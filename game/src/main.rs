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
    chair_model: Option<Rc<Model>>
}

impl Game for Example {
    fn new() -> Box<Example> {
        Box::new(Example {
            chair_model: None
        })
    }

    fn start(&mut self) {
        self.chair_model = Some(app().resources().get_model(String::from("assets/wooden_chair/wooden_chair.fbx")));
    }
    
    fn update(&mut self, _: f32) {
        if !app().input().mouse_button(MouseButton::Left) {
            app().graphics().draw_model(self.chair_model.as_ref().unwrap().clone());
        }
    }
    
    fn stop(&mut self) {
    }
}