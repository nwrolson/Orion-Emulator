use crate::system::*;

pub struct Memory {
    memory: [u8; 0xFFFF],
    // devices mapped to memory addresses
    pub timer: Timer
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0 as u8; 0xFFFF],
            timer: Timer::new()
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            // Timer Registers
            0xFF04 => self.timer.get_DIV(),
            0xFF05 => self.timer.get_TIMA(),
            0xFF06 => self.timer.get_TMA(),
            0xFF07 => self.timer.get_TAC(),

            // normal unmapped address
            _ => self.memory[addr as usize]
        }
    }

    pub fn read_next_word(&self, addr: u16) -> u16 {
        let least_significant_byte = self.memory[(addr+2) as usize] as u16;
        let most_significant_byte = self.memory[(addr+1) as usize] as u16;
        (most_significant_byte << 8) | least_significant_byte
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        match addr {
            // Timer Registers
            0xFF04 => self.timer.reset_DIV(),
            0xFF05 => self.timer.set_TIMA(byte),
            0xFF06 => self.timer.set_TMA(byte),
            0xFF07 => self.timer.set_TAC(byte),

            // normal unmapped address
            _ => self.memory[addr as usize] = byte
        }
    }

    pub fn update_cycle(&mut self, cycles: u8) {
        self.timer.update_timestep(cycles);
    }
}