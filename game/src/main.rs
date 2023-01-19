#![feature(thread_local)]

extern crate little_bits;
use little_bits::*;

use std::borrow::BorrowMut;
use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::RefMut;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    init(Example::new());
}

struct Example {
    model: Option<Rc<Model>>,
    instance: Option<ModelInstance>,
    camera: Option<Rc<RefCell<Camera>>>
}

impl Game for Example {
    fn new() -> Box<Example> {
        Box::new(Example {
            model: None,
            instance: None,
            camera: None
        })
    }

    fn start(&mut self) {
        let app_icon = app().resources().get_image(String::from("assets/icon.png"));

        app().graphics().set_cursor_lock(true);
        app().graphics().set_title("Little Bits Example");
        app().graphics().set_icon(&app_icon);

        self.camera = Some(app().graphics().create_camera());
        app().graphics().set_render_camera(self.camera.clone());

        self.model = Some(app().resources().get_model(String::from("assets/test_models/DamagedHelmet/glTF/DamagedHelmet.gltf")));
        self.instance = Some(app().graphics().create_dynamic_model_instance(self.model.as_ref().unwrap().clone(), None));
    }
    
    fn update(&mut self, delta_time: f32) {
        let mut translation = Float3::default();
        if app().input().key(KeyCode::A) {
            translation += Float3::new(-1.0, 0.0, 0.0);
        }
        if app().input().key(KeyCode::D) {
            translation += Float3::new(1.0, 0.0, 0.0);
        }
        if app().input().key(KeyCode::W) {
            translation += Float3::new(0.0, 0.0, -1.0);
        }
        if app().input().key(KeyCode::S) {
            translation += Float3::new(0.0, 0.0, 1.0);
        }
        translation = translation.normalized() * delta_time;

        let mut camera = self.camera.as_ref().unwrap().as_ref().borrow_mut();
        camera.translate(translation);
    }
    
    fn stop(&mut self) {
    }
}