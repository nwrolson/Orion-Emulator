pub struct Memory {
    memory: [u8; 0xFFFF]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0 as u8; 0xFFFF],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn read_next_word(&self, addr: u16) -> u16 {
        let least_significant_byte = self.memory[addr as usize] as u16;
        let most_significant_byte = self.memory[addr as usize] as u16;
        (most_significant_byte << 8) | least_significant_byte
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        // TODO: Add special mem addresses
        self.memory[addr as usize] = byte;
    }
}