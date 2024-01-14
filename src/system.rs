use std::fs::File;
use std::io::{self, Read};

pub struct Emulator {
    rom: Vec<u8>,
}

impl Emulator {
    pub fn new() -> Self {
        Self { rom: Vec::new() }
    }

    pub fn load_rom(&mut self, romfile: &String) -> Result<(), io::Error> {
        let mut rom = File::open(romfile)?;

        rom.read_to_end(&mut self.rom)?;

        Ok(())
    }

    pub fn get_rom_size(&self) -> usize {
        self.rom.len()
    }
}
