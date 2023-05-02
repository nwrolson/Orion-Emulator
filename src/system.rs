pub mod cpu;
pub mod memory;

use crate::system::cpu::CPU;
use crate::system::memory::Memory;

pub struct System {
    /// Structure that encapsulates a system, including state
    /// of any flags, registers, and memory
    memory: Memory,
    cpu: CPU,
    // cartridge: Cartridge
}

/// struct that abstracts the ROM file as a cartridge connected to System
/// Max size is 32kB
pub struct Cartridge {

}

impl System {
    pub fn new() -> System {
        let memory = Memory::new();
        let cpu = CPU::new();
        System {memory, cpu}
    }
}