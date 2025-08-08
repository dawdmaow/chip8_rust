use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::memory::Memory;

pub struct Cpu {
    pub memory: Memory,
    pub display: Display,
    pub keyboard: Keyboard,
    pub registers: [u8; 16],
    pub index: u16,
    pub program_counter: u16,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Cpu {
    fn invalid(&self, opcode: u16) -> ! {
        let pc = self.program_counter.saturating_sub(2);
        panic!("Invalid opcode 0x{:04X} at PC 0x{:04X}", opcode, pc)
    }

    fn invalid_ctx(&self, opcode: u16, ctx: &str) -> ! {
        let pc = self.program_counter.saturating_sub(2);
        panic!(
            "Invalid opcode 0x{:04X} ({}) at PC 0x{:04X}",
            opcode, ctx, pc
        )
    }
    pub fn new() -> Self {
        let mut memory = Memory::new();
        memory.load_fontset();

        Self {
            memory,
            display: Display::new(),
            keyboard: Keyboard::new(),
            registers: [0; 16],
            index: 0,
            program_counter: 0x200,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn reset(&mut self) {
        self.display.clear();

        self.registers = [0; 16];

        self.index = 0;

        self.program_counter = 0x200;

        self.stack.clear();

        self.delay_timer = 0;
        self.sound_timer = 0;

        self.keyboard.clear();

        self.memory.clear();
        self.memory.load_fontset();
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
        self.execute_opcode(opcode);
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn fetch_opcode(&mut self) -> u16 {
        let high_byte = self.memory.read(self.program_counter) as u16;
        let low_byte = self.memory.read(self.program_counter + 1) as u16;
        self.program_counter += 2;
        (high_byte << 8) | low_byte
    }

    fn execute_opcode(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => self.cls(),
                0x00EE => self.ret(),
                _ => self.invalid(opcode),
            },
            0x1000 => self.jp(nnn),
            0x2000 => self.call(nnn),
            0x3000 => self.se(x, nn),
            0x4000 => self.sne(x, nn),
            0x5000 => self.se_reg(x, y),
            0x6000 => self.ld(x, nn),
            0x7000 => self.add(x, nn),
            0x8000 => match opcode & 0x000F {
                0x0000 => self.ld_reg(x, y),
                0x0001 => self.or(x, y),
                0x0002 => self.and(x, y),
                0x0003 => self.xor(x, y),
                0x0004 => self.add_reg(x, y),
                0x0005 => self.sub(x, y),
                0x0006 => self.shr(x, y),
                0x0007 => self.subn(x, y),
                0x000E => self.shl(x, y),
                _ => self.invalid_ctx(opcode, "8xy?"),
            },
            0x9000 => self.sne_reg(x, y),
            0xA000 => self.ld_i(nnn),
            0xB000 => self.jp_v0(nnn),
            0xC000 => self.rnd(x, nn),
            0xD000 => self.drw(x, y, n),
            0xE000 => match opcode & 0x00FF {
                0x009E => self.skp(x),
                0x00A1 => self.sknp(x),
                _ => self.invalid_ctx(opcode, "Ex??"),
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => self.ld_dt(x),
                0x000A => self.ld_k(x),
                0x0015 => self.ld_delay(x),
                0x0018 => self.ld_sound(x),
                0x001E => self.add_i(x),
                0x0029 => self.ld_f(x),
                0x0033 => self.ld_b(x),
                0x0055 => self.ld_mem(x),
                0x0065 => self.ld_reg_mem(x),
                _ => self.invalid_ctx(opcode, "Fx??"),
            },
            _ => self.invalid(opcode),
        }
    }

    fn cls(&mut self) {
        self.display.clear();
    }

    fn ret(&mut self) {
        if let Some(addr) = self.stack.pop() {
            self.program_counter = addr;
        }
    }

    fn jp(&mut self, addr: u16) {
        self.program_counter = addr;
    }

    fn call(&mut self, addr: u16) {
        self.stack.push(self.program_counter);
        self.program_counter = addr;
    }

    fn se(&mut self, x: usize, nn: u8) {
        if self.registers[x] == nn {
            self.program_counter += 2;
        }
    }

    fn sne(&mut self, x: usize, nn: u8) {
        if self.registers[x] != nn {
            self.program_counter += 2;
        }
    }

    fn se_reg(&mut self, x: usize, y: usize) {
        if self.registers[x] == self.registers[y] {
            self.program_counter += 2;
        }
    }

    fn ld(&mut self, x: usize, nn: u8) {
        self.registers[x] = nn;
    }

    fn add(&mut self, x: usize, nn: u8) {
        self.registers[x] = self.registers[x].wrapping_add(nn);
    }

    fn ld_reg(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[y];
    }

    fn or(&mut self, x: usize, y: usize) {
        self.registers[x] |= self.registers[y];
        self.registers[0xF] = 0;
    }

    fn and(&mut self, x: usize, y: usize) {
        self.registers[x] &= self.registers[y];
        self.registers[0xF] = 0;
    }

    fn xor(&mut self, x: usize, y: usize) {
        self.registers[x] ^= self.registers[y];
        self.registers[0xF] = 0;
    }

    fn add_reg(&mut self, x: usize, y: usize) {
        let vx = self.registers[x];
        let vy = self.registers[y];
        let sum = vx as u16 + vy as u16;
        self.registers[x] = sum as u8;
        self.registers[0xF] = if sum > 255 { 1 } else { 0 };
    }

    fn sub(&mut self, x: usize, y: usize) {
        let vx = self.registers[x];
        let vy = self.registers[y];
        let result = vx.wrapping_sub(vy);
        self.registers[x] = result;
        self.registers[0xF] = if vx >= vy { 1 } else { 0 };
    }

    fn shr(&mut self, x: usize, _y: usize) {
        let vx = self.registers[x];
        let result = vx >> 1;
        self.registers[x] = result;
        self.registers[0xF] = vx & 1;
    }

    fn subn(&mut self, x: usize, y: usize) {
        let vx = self.registers[x];
        let vy = self.registers[y];
        let result = vy.wrapping_sub(vx);
        self.registers[x] = result;
        self.registers[0xF] = if vy >= vx { 1 } else { 0 };
    }

    fn shl(&mut self, x: usize, _y: usize) {
        let vx = self.registers[x];
        let result = vx << 1;
        self.registers[x] = result;
        self.registers[0xF] = (vx & 0x80) >> 7;
    }

    fn sne_reg(&mut self, x: usize, y: usize) {
        if self.registers[x] != self.registers[y] {
            self.program_counter += 2;
        }
    }

    fn ld_i(&mut self, addr: u16) {
        self.index = addr;
    }

    fn jp_v0(&mut self, addr: u16) {
        self.program_counter = addr + self.registers[0] as u16;
    }

    fn rnd(&mut self, x: usize, nn: u8) {
        let random = rand::random::<u8>();
        self.registers[x] = random & nn;
    }

    fn drw(&mut self, x: usize, y: usize, n: u8) {
        let x_pos = self.registers[x] as usize;
        let y_pos = self.registers[y] as usize;

        self.registers[0xF] = 0;

        for row in 0..n {
            let sprite_byte = self.memory.read(self.index + row as u16);
            for col in 0..8 {
                let sprite_pixel = (sprite_byte >> (7 - col)) & 1;
                if sprite_pixel == 1 {
                    let display_x = x_pos + col as usize;
                    let display_y = y_pos + row as usize;
                    if display_x < crate::display::WIDTH && display_y < crate::display::HEIGHT {
                        if self.display.get_pixel(display_x, display_y) {
                            self.registers[0xF] = 1;
                        }
                        self.display.toggle_pixel(display_x, display_y);
                    }
                }
            }
        }
    }

    fn skp(&mut self, x: usize) {
        if self.keyboard.is_key_pressed(self.registers[x]) {
            self.program_counter += 2;
        }
    }

    fn sknp(&mut self, x: usize) {
        if !self.keyboard.is_key_pressed(self.registers[x]) {
            self.program_counter += 2;
        }
    }

    fn ld_dt(&mut self, x: usize) {
        self.registers[x] = self.delay_timer;
    }

    fn ld_k(&mut self, x: usize) {
        if let Some(key) = self.keyboard.get_pressed_key() {
            self.registers[x] = key;
        } else {
            self.program_counter -= 2;
        }
    }

    fn ld_delay(&mut self, x: usize) {
        self.delay_timer = self.registers[x];
    }

    fn ld_sound(&mut self, x: usize) {
        self.sound_timer = self.registers[x];
    }

    fn add_i(&mut self, x: usize) {
        self.index += self.registers[x] as u16;
    }

    fn ld_f(&mut self, x: usize) {
        self.index = (self.registers[x] as u16) * 5;
    }

    fn ld_b(&mut self, x: usize) {
        let value = self.registers[x];
        self.memory.write(self.index, value / 100);
        self.memory.write(self.index + 1, (value / 10) % 10);
        self.memory.write(self.index + 2, value % 10);
    }

    fn ld_mem(&mut self, x: usize) {
        for i in 0..=x {
            self.memory.write(self.index + i as u16, self.registers[i]);
        }
        self.index += (x as u16) + 1;
    }

    fn ld_reg_mem(&mut self, x: usize) {
        for i in 0..=x {
            self.registers[i] = self.memory.read(self.index + i as u16);
        }
        self.index += (x as u16) + 1;
    }
}
