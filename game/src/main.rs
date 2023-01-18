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
        let app_icon = app().resources().get_image(String::from("assets/icon.png"));

        app().graphics().set_title("Little Bits Example");
        app().graphics().set_icon(&app_icon);

        self.chair_model = Some(app().resources().get_model(String::from("assets/monkey.gltf")));
    }
    
    fn update(&mut self, _: f32) {
        if !app().input().mouse_button(MouseButton::Left) {
            app().graphics().draw_model(self.chair_model.as_ref().unwrap().clone());
        }
    }
    
    fn stop(&mut self) {
    }
}