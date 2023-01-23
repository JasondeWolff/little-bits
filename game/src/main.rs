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
        let app_icon = app().resources().get_image(String::from("assets/icon.png"), None);

        //app().graphics().set_cursor_lock(true);
        app().graphics().set_title("Little Bits Example");
        app().graphics().set_icon(app_icon);

        self.camera = app().graphics().create_camera();
        app().graphics().set_render_camera(self.camera.clone());

        self.model = app().resources().get_model(String::from("assets/test_models/DamagedHelmet/glTF/DamagedHelmet.gltf"));
        self.instance = app().graphics().create_dynamic_model_instance(self.model.clone(), None);

        let rotation = Quaternion::from(Float3::new(-90.0, 0.0, 0.0));
        let transform = &mut self.instance.as_mut().transform;
        transform.set_rotation(rotation);
        transform.set_translation(Float3::new(0.0, 0.0, -5.0));
    }
    
    fn update(&mut self, delta_time: f32) {
        let rotation = Quaternion::from(Float3::new(-90.0, app().time() * 30.0, 0.0));
        self.instance.as_mut().transform.set_rotation(rotation);

        // Camera Controller
        {
            let mut translation = Float3::default();
            if app().input().key(KeyCode::A) {
                translation -= Float3::right();
            }
            if app().input().key(KeyCode::D) {
                translation += Float3::right();
            }
            if app().input().key(KeyCode::W) {
                translation -= Float3::forward();
            }
            if app().input().key(KeyCode::S) {
                translation += Float3::forward();
            }
            if app().input().key(KeyCode::E) {
                translation += Float3::up();
            }
            if app().input().key(KeyCode::Q) {
                translation -= Float3::up();
            }
            translation = translation.normalized() * delta_time;

            self.camera.as_mut().translate(translation);
        }
    }

    fn debug_ui(&mut self, ui: &mut DebugUI) {
        ui.window("Hello world")
        .size([300.0, 100.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Hello world!");
            ui.text("こんにちは世界！");
            ui.text("This...is...imgui-rs!");
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        });
    }
    
    fn stop(&mut self) {
    }
}