use minifb::{Key, Window};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Chip8Key {
    D0 = 0,
    D1 = 1,
    D2 = 2,
    D3 = 3,
    D4 = 4,
    D5 = 5,
    D6 = 6,
    D7 = 7,
    D8 = 8,
    D9 = 9,
    A = 10,
    B = 11,
    C = 12,
    D = 13,
    E = 14,
    F = 15,
}

pub struct Chip8Keypad {
    current_key: Option<Chip8Key>,
}

impl Chip8Keypad {
    pub fn new() -> Self {
        return Self { current_key: None };
    }

    pub fn process_input(&mut self, window: &Window) {
        let keys = window.get_keys();
        self.current_key = keys.last().and_then(|key| match key {
            Key::Key1 => Some(Chip8Key::D1),
            Key::Key2 => Some(Chip8Key::D2),
            Key::Key3 => Some(Chip8Key::D3),
            Key::Key4 => Some(Chip8Key::C),

            Key::Q => Some(Chip8Key::D4),
            Key::W => Some(Chip8Key::D5),
            Key::E => Some(Chip8Key::D6),
            Key::R => Some(Chip8Key::D),

            Key::A => Some(Chip8Key::D7),
            Key::S => Some(Chip8Key::D8),
            Key::D => Some(Chip8Key::D9),
            Key::F => Some(Chip8Key::E),

            Key::Z => Some(Chip8Key::A),
            Key::X => Some(Chip8Key::D0),
            Key::C => Some(Chip8Key::B),
            Key::V => Some(Chip8Key::F),

            _ => None,
        });
    }

    pub fn get_key(&self) -> Option<Chip8Key> {
        self.current_key
    }
}
