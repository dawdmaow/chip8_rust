mod audio;
mod core;
mod cpu;
mod display;
mod keyboard;
mod memory;
mod rom;
mod ui;

use raylib::prelude::*;
use std::env;
use std::fs;

fn list_available_roms() {
    let rom_directories = [
        "chip8-roms/games",
        "chip8-roms/demos",
        "chip8-roms/programs",
        // "chip8-roms/hires",
        "chip8-roms/tests",
    ];

    eprintln!("\nAvailable ROMs:");

    for dir in &rom_directories {
        if let Ok(entries) = fs::read_dir(dir) {
            let category = dir.split('/').last().unwrap_or(dir);
            eprintln!("\n{}:", category.to_uppercase());

            let mut roms: Vec<String> = entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext == "ch8")
                        .unwrap_or(false)
                })
                .map(|entry| format!("  {}/{}", dir, entry.file_name().to_string_lossy()))
                .collect();

            roms.sort();
            for rom in roms {
                eprintln!("{}", rom);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        let exe_name = std::path::Path::new(&args[0])
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&args[0]);
        eprintln!("Usage: {} <rom_path>", exe_name);
        eprintln!(
            "Example: {} \"chip8-roms/games/Tetris [Fran Dachille, 1991].ch8\"",
            exe_name
        );
        list_available_roms();
        std::process::exit(1);
    }

    let rom_path = &args[1];

    let (mut rl, thread) = raylib::init()
        .size(ui::WINDOW_WIDTH, ui::WINDOW_HEIGHT)
        .title("CHIP-8 Emulator")
        .build();

    rl.set_target_fps(60);

    let mut cpu = cpu::Cpu::new();

    let mut ui = ui::UI::new();

    let core = core::Core::new().expect("Failed to initialize core systems");

    let beep_sound = audio::create_beep_sound(&core);

    if let Err(e) = rom::load_rom(&mut cpu, rom_path) {
        eprintln!("Failed to load ROM '{}': {}", rom_path, e);
        std::process::exit(1);
    }

    const CYCLES_PER_FRAME: u32 = 15;

    while !rl.window_should_close() {
        cpu.keyboard.update(&rl);

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            ui.paused = !ui.paused;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            ui.step_mode = !ui.step_mode;
        }

        if !ui.paused {
            if ui.step_mode {
                if rl.is_key_pressed(KeyboardKey::KEY_S) {
                    cpu.cycle();
                }
            } else {
                for _ in 0..CYCLES_PER_FRAME {
                    cpu.cycle();
                }
                cpu.tick_timers();
            }
        }

        if cpu.sound_timer > 0 {
            if let Some(ref sound) = beep_sound {
                if !sound.is_playing() {
                    sound.play();
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        display::draw(&mut d, &cpu.display);

        ui.render_status_bar(&mut d, &cpu, rom_path);
    }
}
