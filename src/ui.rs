use crate::cpu::Cpu;
use raylib::prelude::*;

pub const WINDOW_WIDTH: i32 = 1200;
pub const WINDOW_HEIGHT: i32 = 800;
pub const STATUS_BAR_HEIGHT: i32 = 25;

const STATUS_BAR_BOUNDS: Rectangle =
    Rectangle::new(0.0, 0.0, WINDOW_WIDTH as f32, STATUS_BAR_HEIGHT as f32);

pub struct UI {
    pub paused: bool,
    pub step_mode: bool,
}

impl UI {
    pub fn new() -> Self {
        Self {
            paused: false,
            step_mode: false,
        }
    }

    pub fn render_status_bar(&self, d: &mut RaylibDrawHandle, cpu: &Cpu, rom_path: &str) {
        let pressed_keys = cpu.keyboard.pressed_keys();
        let mut pressed_text = String::new();
        for (i, &pressed) in pressed_keys.iter().enumerate() {
            if pressed {
                let key_name = match i {
                    0x0 => "0",
                    0x1 => "1",
                    0x2 => "2",
                    0x3 => "3",
                    0x4 => "4",
                    0x5 => "5",
                    0x6 => "6",
                    0x7 => "7",
                    0x8 => "8",
                    0x9 => "9",
                    0xA => "A",
                    0xB => "B",
                    0xC => "C",
                    0xD => "D",
                    0xE => "E",
                    0xF => "F",
                    _ => "?",
                };
                pressed_text.push_str(key_name);
                pressed_text.push(' ');
            }
        }
        if pressed_text.is_empty() {
            pressed_text = "None".to_string();
        }

        let rom_name = std::path::Path::new(rom_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(rom_path);

        let status_text = format!(
            "ROM: {} | Space: Pause | TAB: Step | I: 0x{:04X} | DT: {:3} | ST: {:3} | FPS: {} | Keys: {} | PC: 0x{:04X}",
            rom_name, cpu.index, cpu.delay_timer, cpu.sound_timer, d.get_fps(), pressed_text.trim(), cpu.program_counter
        );

        d.gui_status_bar(STATUS_BAR_BOUNDS, &status_text);
    }
}
