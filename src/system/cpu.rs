pub mod regfile;
pub mod instruction;
pub mod cpu_tests;

use crate::system::cpu::regfile::Regfile;
use crate::system::cpu::instruction::{Instruction, ArithmeticArg};
use crate::system::memory::Memory;
pub struct CPU {
    regfile: Regfile,
    pc: u16,
    sp: u16,
}

impl CPU {
    pub fn new() -> CPU {
        let pc: u16 = 0;
        let sp: u16 = 0xFFFE;
        let regfile: Regfile = Regfile::new();
        CPU {regfile, pc, sp}
    }

    pub fn run(&mut self, memory: &mut Memory) {
        // TODO: IME and interrupt checking
        let opcode_byte = self.fetch(memory);
        // decode
        let instruction = Instruction::from_byte(opcode_byte);
        self.execute(instruction, memory);
    }

    fn fetch(&mut self, memory: &Memory) -> u8 {
        let mut byte = memory.read_byte(self.pc);
        if byte == 0xCB { //prefix instruction
            self.pc += 1;
            byte = memory.read_byte(self.pc); 
        }
        byte
    }

    fn execute(&mut self, instruction: Instruction, memory: &mut Memory) {
        use crate::system::cpu::instruction::InstructionType::*;
        match instruction.instr_type {
            Arithmetic(target) => {
                // get val to use for operation
                let val = { 
                    match target {
                        ArithmeticArg::A => self.regfile.r_a,
                        ArithmeticArg::B => self.regfile.r_b,
                        ArithmeticArg::C => self.regfile.r_c,
                        ArithmeticArg::D => self.regfile.r_d,
                        ArithmeticArg::E => self.regfile.r_e,
                        ArithmeticArg::H => self.regfile.r_h,
                        ArithmeticArg::L => self.regfile.r_l,
                        ArithmeticArg::HL => {
                            let addr = self.regfile.get_hl();
                            memory.read_byte(addr)   
                        }
                        ArithmeticArg::D8 => {
                            self.pc_add(1);
                            memory.read_byte(self.pc)
                        }
                        _ => 0,
                    }
                };
            match instruction.op {
                _ => {}
                }
            self.pc_add(1);
            }
            Misc => {
                match instruction.op {
                    _ => {}
                }
                self.pc_add(1);
            }
            _ => {}
        }
    }

    // helper functions for instructions
    fn pc_add(&mut self, val: u16) {
        let new_val = self.pc.wrapping_add(val);
        self.pc = new_val;
    }


}


