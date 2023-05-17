use crate::system::*;

pub struct Memory {
    memory: [u8; 0x10000],
    // devices mapped to memory addresses
    pub timer: Timer
}

pub struct Interrupts {
    pub vblank: bool,
    pub lcd: bool,
    pub timer: bool,
    pub serial: bool,
    pub joypad: bool,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0 as u8; 0x10000],
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

    pub fn get_interrupts(&self) -> Interrupts {
        // 0xFFFF - Interrupt Enable
        // 0xFF0F - Interrupt Flags
        // Flags in 0xFF0F:
        // Bit 0 - VBlank
        // Bit 1 - LCD STAT
        // Bit 2 - Timer
        // Bit 3 - Serial
        // Bit 4 - Joypad
        let enable = self.memory[0xFFFF];
        let flags = self.memory[0xFF0F];
        let vblank = ((enable & 0x1) & (flags & 0x1)) > 0;
        let lcd = ((enable & 0x2) & (flags & 0x2)) > 0;
        let timer = ((enable & 0x4) & (flags & 0x4)) > 0;
        let serial = ((enable & 0x8) & (flags & 0x8)) > 0;
        let joypad = ((enable & 0x10) & (flags & 0x10)) > 0;
        Interrupts {
            vblank, lcd, timer, serial, joypad
        }

    }

    pub fn clear_interrupts(&mut self) {
        self.memory[0xFF0F] = 0;
    }

    pub fn update_cycle(&mut self, cycles: u8) {
        let timer = self.timer.update_timestep(cycles);
        if timer { self.memory[0xFF0F] |= 0x04 }
    }
}