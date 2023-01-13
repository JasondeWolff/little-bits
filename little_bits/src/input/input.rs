use crate::system::*;
use crate::maths::*;

#[path = "input_codes.rs"] pub mod input_codes;
pub use input_codes::*;

pub(crate) const MAX_KEYS: usize = 512;
pub(crate) const MAX_BUTTONS: usize = 32;

#[derive(Clone)]
pub(crate) struct KeyboardValues {
    pub keys: [bool; MAX_KEYS]
}

impl KeyboardValues {
    pub fn new() -> Self {
        KeyboardValues {
            keys: [false; MAX_KEYS]
        }
    }
}

#[derive(Clone)]
pub(crate) struct MouseValues {
    pub buttons: [bool; MAX_BUTTONS],
    pub position: Float2
}

impl MouseValues {
    pub fn new() -> Self {
        MouseValues {
            buttons: [false; MAX_BUTTONS],
            position: Float2::default()
        }
    }
}

pub struct Input {
    keyboard: KeyboardValues,
    old_keyboard: KeyboardValues,
    mouse: MouseValues,
    old_mouse: MouseValues
}

impl System for Input {
    fn init() -> Box<Input> {
        Box::new(Input {
            keyboard: KeyboardValues::new(),
            old_keyboard: KeyboardValues::new(),
            mouse: MouseValues::new(),
            old_mouse: MouseValues::new()
        })
    }

    fn update(&mut self) {
        self.old_keyboard = self.keyboard.clone();
        self.old_mouse = self.mouse.clone();
    }
}

impl Input {
    pub fn key(&self, key_code: KeyCode) -> bool {
        self.keyboard.keys[key_code as usize]
    }

    pub fn key_down(&self, key_code: KeyCode) -> bool {
        self.keyboard.keys[key_code as usize] && !self.old_keyboard.keys[key_code as usize]
    }

    pub fn mouse_button(&self, button: MouseButton) -> bool {
        self.mouse.buttons[button as usize]
    }

    pub fn mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse.buttons[button as usize] && !self.old_mouse.buttons[button as usize]
    }

    pub fn mouse_position(&self) -> Float2 {
        self.mouse.position
    }

    pub fn mouse_delta(&self) -> Float2 {
        self.old_mouse.position - self.mouse.position
    }

    pub(crate) fn set_key(&mut self, key_code: KeyCode, value: bool) {
        self.keyboard.keys[key_code as usize] = value;
    }

    pub(crate) fn set_button(&mut self, button: MouseButton, value: bool) {
        self.mouse.buttons[button as usize] = value;
    }

    pub(crate) fn set_mouse_position(&mut self, position: Float2) {
        self.mouse.position = position;
    }
}