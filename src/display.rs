use raylib::prelude::*;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    pixels: [[bool; WIDTH]; HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [[false; WIDTH]; HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.pixels {
            for pixel in row {
                *pixel = false;
            }
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        if x < WIDTH && y < HEIGHT {
            self.pixels[y][x]
        } else {
            false
        }
    }

    pub fn toggle_pixel(&mut self, x: usize, y: usize) {
        if x < WIDTH && y < HEIGHT {
            self.pixels[y][x] = !self.pixels[y][x];
        }
    }
}

pub const PIXEL_SIZE: i32 = 10;

pub fn draw(d: &mut RaylibDrawHandle, display: &Display) {
    let game_width = (WIDTH as i32) * PIXEL_SIZE;
    let game_height = (HEIGHT as i32) * PIXEL_SIZE;
    let rom_panel_height = 80;
    let window_width = 1200;
    let window_height = 800;

    let offset_x = (window_width - game_width) / 2;
    let offset_y = rom_panel_height + (window_height - rom_panel_height - game_height) / 2;

    d.draw_rectangle(
        offset_x,
        offset_y,
        game_width,
        game_height,
        Color::new(20, 20, 20, 255),
    );

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if display.get_pixel(x, y) {
                d.draw_rectangle(
                    offset_x + (x as i32 * PIXEL_SIZE),
                    offset_y + (y as i32 * PIXEL_SIZE),
                    PIXEL_SIZE,
                    PIXEL_SIZE,
                    Color::WHITE,
                );
            }
        }
    }
}
