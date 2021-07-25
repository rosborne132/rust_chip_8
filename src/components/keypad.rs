use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::option::Option;

pub struct Keypad {
    pub key: [u8; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Self { key: [0; 16] }
    }

    pub fn press(&mut self, key: Option<Keycode>, state: u8) {
        match key {
            Some(Keycode::Num1) => self.key[1] = state,
            Some(Keycode::Num2) => self.key[2] = state,
            Some(Keycode::Num3) => self.key[3] = state,
            Some(Keycode::Num4) => self.key[0xC] = state,
            Some(Keycode::Q) => self.key[4] = state,
            Some(Keycode::W) => self.key[5] = state,
            Some(Keycode::E) => self.key[6] = state,
            Some(Keycode::R) => self.key[0xD] = state,
            Some(Keycode::A) => self.key[7] = state,
            Some(Keycode::S) => self.key[8] = state,
            Some(Keycode::D) => self.key[9] = state,
            Some(Keycode::F) => self.key[0xE] = state,
            Some(Keycode::Z) => self.key[0xA] = state,
            Some(Keycode::X) => self.key[0] = state,
            Some(Keycode::C) => self.key[0xB] = state,
            Some(Keycode::V) => self.key[0xF] = state,
            _ => {}
        }
    }
}
