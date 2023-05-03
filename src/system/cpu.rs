pub mod regfile;
pub mod instruction;
pub mod cpu_tests;

use crate::system::cpu::regfile::Regfile;
use crate::system::cpu::instruction::*;
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
                    Opcode::INC => { self.increment(memory, target) },
                    Opcode::DEC => { self.decrement(memory, target) },
                    Opcode::ADD => { self.add8(val) },
                    Opcode::SUB => { self.sub8(val, false) },
                    Opcode::ADC => { 
                                    if self.regfile.get_carry() {
                                        self.add8(1)
                                    }
                                    self.add8(val)
                                },
                    Opcode::SBC => { 
                                    if self.regfile.get_carry() {
                                        self.sub8(1, false)
                                    }
                                    self.sub8(val, false)
                                },
                    Opcode::AND => { self.and8(val) },
                    Opcode::XOR => { self.xor8(val) },
                    Opcode::OR => { self.or8(val) },
                    Opcode::CP => { self.sub8(val, true)},
                    _ => {}
                }
                self.pc_add(1);
            }
            Unary16(target) => {
                match instruction.op {
                    Opcode::INC => { self.increment16(target) },
                    Opcode::DEC => { self.decrement16(target) },
                    _ => {}
                }
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

    fn add8(&mut self, val: u8) {
        let (new_val, overflow) = self.regfile.r_a.overflowing_add(val);
        self.regfile.half_add(self.regfile.r_a, val);
        self.regfile.set_zero(new_val == 0);
        self.regfile.set_sub(false);
        self.regfile.set_carry(overflow);
        self.regfile.r_a = new_val;
    }

    fn sub8(&mut self, val: u8, compare: bool) {
        let (new_val, overflow) = self.regfile.r_a.overflowing_sub(val);
        self.regfile.half_sub(self.regfile.r_a, val);
        self.regfile.set_zero(new_val == 0);
        self.regfile.set_sub(true);
        self.regfile.set_carry(overflow);
        if !compare { self.regfile.r_a = new_val; }
    }

    fn and8(&mut self, val: u8) {
        self.regfile.r_a &= val;
        self.regfile.set_zero(self.regfile.r_a == 0);
        self.regfile.set_sub(false);
        self.regfile.set_half_carry(true);
        self.regfile.set_carry(false);
    }

    fn or8(&mut self, val: u8) {
        self.regfile.r_a |= val;
        self.regfile.set_zero(self.regfile.r_a == 0);
        self.regfile.set_sub(false);
        self.regfile.set_half_carry(false);
        self.regfile.set_carry(false);
    }

    fn xor8(&mut self, val: u8) {
        self.regfile.r_a ^= val;
        self.regfile.set_zero(self.regfile.r_a == 0);
        self.regfile.set_sub(false);
        self.regfile.set_half_carry(false);
        self.regfile.set_carry(false);
    }

    fn increment16(&mut self, target: Word16) {
        // no flags set
        match target {
            Word16::BC => {
                let val = self.regfile.get_bc();
                self.regfile.set_bc(val.wrapping_add(1));
            }
            Word16::DE => {
                let val = self.regfile.get_de();
                self.regfile.set_de(val.wrapping_add(1));
            }
            Word16::HL => {
                let val = self.regfile.get_hl();
                self.regfile.set_hl(val.wrapping_add(1));
            }
            Word16::SP => {
                let val = self.sp.wrapping_add(1);
                self.sp = val;
            }
        }
    }

    fn decrement16(&mut self, target: Word16) {
        // no flags set
        match target {
            Word16::BC => {
                let val = self.regfile.get_bc();
                self.regfile.set_bc(val.wrapping_sub(1));
            }
            Word16::DE => {
                let val = self.regfile.get_de();
                self.regfile.set_de(val.wrapping_sub(1));
            }
            Word16::HL => {
                let val = self.regfile.get_hl();
                self.regfile.set_hl(val.wrapping_sub(1));
            }
            Word16::SP => {
                let val = self.sp.wrapping_sub(1);
                self.sp = val;
            }
        }
    }

    fn increment(&mut self, memory: &mut Memory, reg: ArithmeticArg) {
        match reg {
            ArithmeticArg::A => { 
                self.regfile.half_add(self.regfile.r_a, 1);
                let val = self.regfile.r_a.wrapping_add(1);
                self.regfile.r_a = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::B => {
                self.regfile.half_add(self.regfile.r_b, 1); 
                let val = self.regfile.r_b.wrapping_add(1);
                self.regfile.r_b = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::C => { 
                self.regfile.half_add(self.regfile.r_c, 1);
                let val = self.regfile.r_c.wrapping_add(1);
                self.regfile.r_c = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::D => { 
                self.regfile.half_add(self.regfile.r_d, 1);
                let val = self.regfile.r_d.wrapping_add(1);
                self.regfile.r_d = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::E => { 
                self.regfile.half_add(self.regfile.r_e, 1);
                let val = self.regfile.r_e.wrapping_add(1);
                self.regfile.r_e = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::H => { 
                self.regfile.half_add(self.regfile.r_h, 1);
                let val = self.regfile.r_h.wrapping_add(1);
                self.regfile.r_h = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::L => { 
                self.regfile.half_add(self.regfile.r_l, 1);
                let val = self.regfile.r_l.wrapping_add(1);
                self.regfile.r_l = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::HL => {
                let addr = self.regfile.get_hl();
                let memory_val = memory.read_byte(addr);
                self.regfile.half_add(memory_val, 1);
                let inc_val = memory_val.wrapping_add(1);
                memory.write_byte(addr, inc_val);
                self.regfile.set_zero(inc_val == 0);
            }
            _ => {}
        }
        self.regfile.set_sub(false);
        // carry flag left unchanged
    }

    fn decrement(&mut self, memory: &mut Memory, reg: ArithmeticArg) {
        match reg {
            ArithmeticArg::A => { 
                self.regfile.half_sub(self.regfile.r_a, 1);
                let val = self.regfile.r_a.wrapping_sub(1);
                self.regfile.r_a = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::B => {
                self.regfile.half_sub(self.regfile.r_b, 1); 
                let val = self.regfile.r_b.wrapping_sub(1);
                self.regfile.r_b = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::C => { 
                self.regfile.half_sub(self.regfile.r_c, 1);
                let val = self.regfile.r_c.wrapping_sub(1);
                self.regfile.r_c = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::D => { 
                self.regfile.half_sub(self.regfile.r_d, 1);
                let val = self.regfile.r_d.wrapping_sub(1);
                self.regfile.r_d = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::E => { 
                self.regfile.half_sub(self.regfile.r_e, 1);
                let val = self.regfile.r_e.wrapping_sub(1);
                self.regfile.r_e = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::H => { 
                self.regfile.half_sub(self.regfile.r_h, 1);
                let val = self.regfile.r_h.wrapping_sub(1);
                self.regfile.r_h = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::L => { 
                self.regfile.half_sub(self.regfile.r_l, 1);
                let val = self.regfile.r_l.wrapping_sub(1);
                self.regfile.r_l = val;
                self.regfile.set_zero(val == 0);
            }
            ArithmeticArg::HL => {
                let addr = self.regfile.get_hl();
                let memory_val = memory.read_byte(addr);
                self.regfile.half_sub(memory_val, 1);
                let dec_val = memory_val.wrapping_sub(1);
                memory.write_byte(addr, dec_val);
                self.regfile.set_zero(dec_val == 0);
            }
            _ => {}
        }
        self.regfile.set_sub(true);
        // carry flag left unchanged
    }

}


