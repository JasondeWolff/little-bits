#![feature(thread_local)]

extern crate little_bits;
use little_bits::*;

use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    init(Example::new());
}

struct Example {
    model: Shared<Model>,
    instance: Shared<ModelInstance>,
    camera: Shared<Camera>
}

impl Game for Example {
    fn new() -> Box<Example> {
        Box::new(Example {
            model: Shared::empty(),
            instance: Shared::empty(),
            camera: Shared::empty()
        })
    }

    fn start(&mut self) {
        let app_icon = app().resources().get_image(String::from("assets/icon.png"));

        app().graphics().set_cursor_lock(true);
        app().graphics().set_title("Little Bits Example");
        app().graphics().set_icon(app_icon);

        self.camera = app().graphics().create_camera();
        app().graphics().set_render_camera(self.camera.clone());

        self.model = app().resources().get_model(String::from("assets/test_models/DamagedHelmet/glTF/DamagedHelmet.gltf"));
        self.instance = app().graphics().create_dynamic_model_instance(self.model.clone(), None);

        let rotation = Quaternion::from(Float3::new(-90.0, 0.0, 0.0));
        self.instance.as_mut().transform.set_rotation(rotation);
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

        self.camera.as_mut().translate(translation);

        let rotation = Quaternion::from(Float3::new(-90.0, app().time() * 10.0, 0.0));
        self.instance.as_mut().transform.set_rotation(rotation);
    }
    
    fn stop(&mut self) {
    }
}