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
    model: Option<Rc<Model>>,
    instance: Option<ModelInstance>
}

impl Game for Example {
    fn new() -> Box<Example> {
        Box::new(Example {
            model: None,
            instance: None
        })
    }

    fn start(&mut self) {
        let app_icon = app().resources().get_image(String::from("assets/icon.png"));

        app().graphics().set_title("Little Bits Example");
        app().graphics().set_icon(&app_icon);

        self.model = Some(app().resources().get_model(String::from("assets/test_models/DamagedHelmet/glTF/DamagedHelmet.gltf")));
        self.instance = Some(app().graphics().create_dynamic_model_instance(self.model.as_ref().unwrap().clone(), None));
    }
    
    fn update(&mut self, _: f32) {
        if app().input().mouse_button_down(MouseButton::Left) {
            let transform = app().graphics().get_dynamic_model_transform(self.instance.as_ref().unwrap().clone());
            transform.set_translation(transform.get_translation() + Float3::new(0.0, 0.0, -1.0));
        }
        if app().input().key_down(KeyCode::Space) {
            app().graphics().destroy_dynamic_models();
        }
    }
    
    fn stop(&mut self) {
    }
}