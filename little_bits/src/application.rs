#[path = "graphics/graphics.rs"] pub mod graphics;
pub use graphics::*;

#[path = "resources/resources.rs"] pub mod resources;
pub use resources::*;

#[path = "input/input.rs"] pub mod input;
pub use input::*;

#[path = "common/timer.rs"] pub mod timer;
pub use timer::Timer;

use crate::System;

pub struct Application {
    graphics: Option<Box<Graphics>>,
    resources: Option<Box<Resources>>,
    input: Option<Box<Input>>,

    timer: Timer,
    stopped: bool,
    game: Box<dyn Game>,
}

pub trait Game {
    fn new() -> Box<Self> where Self: Sized;

    fn start(&mut self);
    fn update(&mut self, delta_time: f32);
    fn stop(&mut self);
}

impl Application {
    pub fn new<G: Game + 'static>(game: Box<G>) -> Box<Self> {
        Box::new(Application {
            graphics: None,
            resources: None,
            input: None,

            timer: Timer::new(),
            stopped: false,
            game: game
        })
    }

    fn init_systems(&mut self) {
        self.resources = Some(Resources::init());
        self.graphics = Some(Graphics::init());
        self.input = Some(Input::init());
    }

    pub fn start(&mut self) {
        self.init_systems();

        self.game.start();

        let mut timer = Timer::new();
        let mut frames: u32 = 0;
        let mut delta_timer = Timer::new();

        while !self.graphics().should_close() {
            let delta_time = delta_timer.elapsed() as f32;
            delta_timer.reset();

            self.game.update(delta_time);

            self.input().update();
            self.graphics().update();
            self.resources().update();
    
            frames += 1;
            if timer.elapsed() >= 1.0 {
                println!("FPS {}", frames);
                frames = 0;
                timer.reset();
            }
        }

        self.game.stop();
        self.stopped = true;
    }

    pub fn graphics(&mut self) -> &mut Graphics {
        self.graphics.as_mut().expect("Failed to get graphics.").as_mut()
    }

    pub fn resources(&mut self) -> &mut Resources {
        self.resources.as_mut().expect("Failed to get resources.").as_mut()
    }

    pub fn input(&mut self) -> &mut Input {
        self.input.as_mut().expect("Failed to get input.").as_mut()
    }

    pub fn time(&self) -> f32 {
        self.timer.elapsed() as f32
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        if !self.stopped {
            self.game.stop();
        }
    }
}