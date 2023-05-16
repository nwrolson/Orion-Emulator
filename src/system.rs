pub mod cpu;
pub mod memory;
pub mod timer;
pub mod timer_tests;

use crate::system::cpu::CPU;
use crate::system::memory::Memory;
use crate::system::timer::Timer;

pub struct System{
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
    pub fn new() -> System{
        let cpu = CPU::new();
        let memory = Memory::new();
        System {memory, cpu}
    }
}