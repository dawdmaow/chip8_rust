use raylib::prelude::*;

pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.keys[0x1] = rl.is_key_down(KeyboardKey::KEY_ONE);
        self.keys[0x2] = rl.is_key_down(KeyboardKey::KEY_TWO);
        self.keys[0x3] = rl.is_key_down(KeyboardKey::KEY_THREE);
        self.keys[0xC] = rl.is_key_down(KeyboardKey::KEY_FOUR);

        self.keys[0x4] = rl.is_key_down(KeyboardKey::KEY_Q);
        self.keys[0x5] = rl.is_key_down(KeyboardKey::KEY_W);
        self.keys[0x6] = rl.is_key_down(KeyboardKey::KEY_E);
        self.keys[0xD] = rl.is_key_down(KeyboardKey::KEY_R);

        self.keys[0x7] = rl.is_key_down(KeyboardKey::KEY_A);
        self.keys[0x8] = rl.is_key_down(KeyboardKey::KEY_S);
        self.keys[0x9] = rl.is_key_down(KeyboardKey::KEY_D);
        self.keys[0xE] = rl.is_key_down(KeyboardKey::KEY_F);

        self.keys[0xA] = rl.is_key_down(KeyboardKey::KEY_Z);
        self.keys[0x0] = rl.is_key_down(KeyboardKey::KEY_X);
        self.keys[0xB] = rl.is_key_down(KeyboardKey::KEY_C);
        self.keys[0xF] = rl.is_key_down(KeyboardKey::KEY_V);
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        if key < 16 {
            self.keys[key as usize]
        } else {
            false
        }
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        for (i, &pressed) in self.keys.iter().enumerate() {
            if pressed {
                return Some(i as u8);
            }
        }
        None
    }

    pub fn clear(&mut self) {
        self.keys = [false; 16];
    }

    pub fn pressed_keys(&self) -> [bool; 16] {
        self.keys
    }
}
