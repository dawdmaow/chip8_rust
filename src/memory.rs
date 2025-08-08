use std::fs::File;
use std::io::Read;

pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Self { ram: [0; 4096] }
    }

    pub fn read(&self, address: u16) -> u8 {
        if address < 4096 {
            self.ram[address as usize]
        } else {
            0
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address < 4096 {
            self.ram[address as usize] = value;
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        let start_address = 0x200;
        for (i, &byte) in program.iter().enumerate() {
            if start_address + i < 4096 {
                self.ram[start_address + i] = byte;
            }
        }
    }

    pub fn load_rom(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        self.load_program(&buffer);
        Ok(())
    }

    pub fn load_fontset(&mut self) {
        let fontset = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];

        for (i, &byte) in fontset.iter().enumerate() {
            self.ram[i] = byte;
        }
    }

    pub fn clear(&mut self) {
        self.ram = [0; 4096];
    }
}
