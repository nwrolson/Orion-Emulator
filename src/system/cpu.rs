pub mod regfile;
pub mod instruction;
pub mod cpu_tests;

use crate::system::cpu::regfile::Regfile;
use crate::system::cpu::instruction::*;
use crate::system::memory::{Memory, self};
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
        let (opcode_byte, prefix) = self.fetch(memory);
        // decode
        let instruction = if !prefix {
            Instruction::from_byte(opcode_byte)
        } else { Instruction::from_byte_prefix(opcode_byte) };
        self.execute(instruction, memory);
    }

    fn fetch(&mut self, memory: &Memory) -> (u8,bool) {
        let mut byte = memory.read_byte(self.pc);
        let mut prefix = false;
        if byte == 0xCB { //prefix instruction
            self.pc += 1;
            byte = memory.read_byte(self.pc);
            prefix = true; 
        }
        (byte, prefix)
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
                    Opcode::DAA => { self.daa() },
                    Opcode::CPL => {
                        self.regfile.r_a ^= 0xFF;
                        self.regfile.set_sub(true);
                        self.regfile.set_half_carry(true);
                    },
                    Opcode::CCF => {
                        self.regfile.toggle_carry();
                        self.regfile.set_sub(false);
                        self.regfile.set_half_carry(false);
                    },
                    Opcode::SCF => {
                        self.regfile.set_carry(true);
                        self.regfile.set_sub(false);
                        self.regfile.set_half_carry(false);
                    },
                    _ => {}
                }
                self.pc_add(1);
            }
            Rotate(target) => {
                match instruction.op {
                    Opcode::RLCA => {
                        self.rotate_left(memory, Word8::A, Opcode::RLCA);
                        self.regfile.set_zero(false);
                    }
                    Opcode::RLC => { self.rotate_left(memory, target, Opcode::RLC) }
                    Opcode::RLA => { 
                        self.rotate_left(memory, target, instruction.op);
                        self.regfile.set_zero(false);
                    }
                    Opcode::RL => {
                        self.rotate_left(memory, target, instruction.op)
                    }
                    Opcode::RRCA => {
                        self.rotate_right(memory, Word8::A, instruction.op);
                        self.regfile.set_zero(false);
                    }
                    Opcode::RRC => {
                        self.rotate_right(memory, target, instruction.op)
                    }
                    Opcode::RRA => {
                        self.rotate_right(memory, target, instruction.op);
                        self.regfile.set_zero(false);
                    }
                    Opcode::RR => {
                        self.rotate_right(memory, target, instruction.op)
                    }
                    Opcode::SWAP => {
                        self.swap_bits(memory, target);
                    }
                    _ => {}
                }
                self.pc_add(1);
            }
            Bit(target, bit) => {
                match instruction.op {
                    Opcode::BIT => self.bit(memory, target, bit),
                    Opcode::RES => self.set_bit(memory, target, bit, false),
                    Opcode::SET => self.set_bit(memory, target, bit, true),
                    _ => {}
                }
            }
            Load(target, source) => {
                let source_val = {
                    match source {
                        LoadSource::A => self.regfile.r_a,
                        LoadSource::B => self.regfile.r_b,
                        LoadSource::C => self.regfile.r_c,
                        LoadSource::D => self.regfile.r_d,
                        LoadSource::E => self.regfile.r_e,
                        LoadSource::H => self.regfile.r_h,
                        LoadSource::L => self.regfile.r_l,
                        LoadSource::HL => {
                            let addr = self.regfile.get_hl();
                            memory.read_byte(addr)
                        }
                        LoadSource::HLI => {
                            // increment HL after fetching
                            let addr = self.regfile.get_hl();
                            let val = memory.read_byte(addr);
                            let new_hl = addr.wrapping_add(1);
                            self.regfile.set_hl(new_hl);
                            val
                        }
                        LoadSource::HLD => {
                            // decrement HL after fetching
                            let addr = self.regfile.get_hl();
                            let val = memory.read_byte(addr);
                            let new_hl = addr.wrapping_sub(1);
                            self.regfile.set_hl(new_hl);
                            val
                        }
                        LoadSource::BC => {
                            let addr = self.regfile.get_bc();
                            memory.read_byte(addr)
                        }
                        LoadSource::DE => {
                            let addr = self.regfile.get_de();
                            memory.read_byte(addr)
                        }
                        LoadSource::D8 => {
                            self.pc_add(1);
                            memory.read_byte(self.pc)
                        }
                        LoadSource::A8 => {
                            self.pc_add(1);
                            let addr = (memory.read_byte(self.pc) as u16)
                                + 0xFF00;
                            memory.read_byte(addr)
                        }
                        LoadSource::A16 => {
                            let addr = memory.read_next_word(self.pc);
                            memory.read_byte(addr)
                        }
                        LoadSource::CA => {
                            let addr = (self.regfile.r_c as u16) + 0xFF00;
                            memory.read_byte(addr)
                        }
                    }
                };
                match target {
                    LoadTarget::A => self.regfile.r_a = source_val,
                    LoadTarget::B => self.regfile.r_b = source_val,
                    LoadTarget::C => self.regfile.r_c = source_val,
                    LoadTarget::D => self.regfile.r_d = source_val,
                    LoadTarget::E => self.regfile.r_e = source_val,
                    LoadTarget::H => self.regfile.r_h = source_val,
                    LoadTarget::L => self.regfile.r_l = source_val,
                    LoadTarget::HL => {
                        let addr = self.regfile.get_hl();
                        memory.write_byte(addr, source_val);
                    }
                    LoadTarget::HLI => {
                        let addr = self.regfile.get_hl();
                        memory.write_byte(addr, source_val);
                        let new_hl = addr.wrapping_add(1);
                        self.regfile.set_hl(new_hl);
                    }
                    LoadTarget::HLD => {
                        let addr = self.regfile.get_hl();
                        memory.write_byte(addr, source_val);
                        let new_hl = addr.wrapping_sub(1);
                        self.regfile.set_hl(new_hl);
                    }
                    LoadTarget::BC => {
                        let addr = self.regfile.get_bc();
                        memory.write_byte(addr, source_val);
                    }
                    LoadTarget::DE => {
                        let addr = self.regfile.get_de();
                        memory.write_byte(addr, source_val);
                    }
                    LoadTarget::A8 => {
                        self.pc_add(1);
                        let addr = (memory.read_byte(self.pc) as u16)
                            + 0xFF00;
                        memory.write_byte(addr, source_val);
                    }
                    LoadTarget::CA => {
                        let addr = (self.regfile.r_c as u16) + 0xFF00;
                        memory.write_byte(addr, source_val);
                    }
                    LoadTarget::A16 => {
                        let addr = memory.read_next_word(self.pc);
                        self.pc_add(2);
                        memory.write_byte(addr, source_val);
                    }
                }
                self.pc_add(1);
            }
            Push(target) => {
                match target {
                    RegisterPair::BC => self.stack_push(memory, self.regfile.get_bc()),
                    RegisterPair::DE => self.stack_push(memory, self.regfile.get_de()),
                    RegisterPair::HL => self.stack_push(memory, self.regfile.get_hl()),
                    RegisterPair::AF => self.stack_push(memory, self.regfile.get_af()),
                }
                self.pc_add(1);
                },
             Pop(target) => {
                let val = self.stack_pop(memory);
                match target {
                    RegisterPair::BC => self.regfile.set_bc(val),
                    RegisterPair::DE => self.regfile.set_de(val),
                    RegisterPair::HL => self.regfile.set_hl(val),
                    RegisterPair::AF => self.regfile.set_af(val),
                }
                self.pc_add(1);
             },
             Jump(cond) => {
                let should_jump = {
                    match cond {
                        JumpCond::Zero => self.regfile.get_zero(),
                        JumpCond::Carry => self.regfile.get_carry(),
                        JumpCond::NotZero => !self.regfile.get_zero(),
                        JumpCond::NotCarry => !self.regfile.get_carry(),
                        JumpCond::Always => true,
                    }
                };
                self.pc = {
                    match instruction.op {
                        Opcode::JR => self.jump_relative(memory, should_jump),
                        Opcode::JP => self.jump(memory, should_jump),
                        _ => 0
                    }
                };
            }
            JumpHL => {
                self.pc = self.regfile.get_hl();
            }
            RST(addr) => {
                self.pc = addr as u16;
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

    fn sp_inc(&mut self) {
        self.sp = self.sp.wrapping_add(1);
    }

    fn sp_dec(&mut self) {
        self.sp = self.sp.wrapping_sub(1);
    }

    fn stack_push(&mut self, memory: &mut Memory, val: u16) {
        let most_significant_byte = ((val & 0xFF00) >> 8) as u8;
        let least_significant_byte = (val & 0x00FF) as u8;
        // write MSB first
        // println!("pushing - msb: {:02X?} lsb: {:02X?}", most_significant_byte,
        //  least_significant_byte);
        self.sp_dec();
        memory.write_byte(self.sp, most_significant_byte);
        self.sp_dec();
        memory.write_byte(self.sp, least_significant_byte);
    }

    fn stack_pop(&mut self, memory: &mut Memory) -> u16 {
        let least_significant_byte = memory.read_byte(self.sp) as u16;
        self.sp_inc();
        let most_significant_byte = memory.read_byte(self.sp) as u16;
        self.sp_inc();
        (most_significant_byte << 8) | least_significant_byte
    }

    fn jump_relative(&self, memory: &mut Memory, should_jump: bool) -> u16 {
        if should_jump {
            // converting from two's complement to decimal
            let val: i16 = memory.read_byte(self.pc.wrapping_add(1)) as i16;
            let offset : i16 = (-1*(val & 0x80) + (val & 0x7F)) as i16;
            ((self.pc as i16) + 2 + offset) as u16 // + 2 is to account for instruction length
        }
        else {
            self.pc.wrapping_add(2)
        }
    }

    fn jump(&self, memory: &mut Memory, should_jump: bool) -> u16 {
        if should_jump {
            memory.read_next_word(self.pc)
        }
        else {
            self.pc.wrapping_add(3)
        }
    }

    fn rotate_left(&mut self, memory: &mut Memory, target: Word8, opcode: Opcode) {
        // this code is atrociously ugly, but I blame the gameboy cpu!
        // there isn't an elegant way to implement this I can find
        fn rl(opcode: Opcode, regfile: &mut Regfile, path: impl Fn(&mut Regfile) -> &mut u8) {
            let current_val = *(path(regfile));
            let old_carry = regfile.get_carry();
            regfile.set_carry((current_val & 0x80) > 0);
            if opcode == Opcode::SLA {
                // shift
                let val = current_val.rotate_left(1);
                let val = val & 0xFE; // bit 0 is always zeroed
                *(path(regfile)) = val;
                regfile.set_zero(val == 0);
            }
            else {
                let val = if opcode == Opcode::RLC || opcode == Opcode::RLCA {
                    current_val.rotate_left(1)
                }
                else {
                    let val = current_val.rotate_left(1);
                    if old_carry { val | 0x01 } else { val & 0xFE }
                };
                *(path(regfile)) = val;
                regfile.set_zero(val == 0);
            }
        }
        match target {
            Word8::A => {
                rl(opcode, &mut self.regfile, |regfile| &mut regfile.r_a);
            }
            Word8::B => {
                rl(opcode, &mut self.regfile, |regfile| &mut regfile.r_b);
            }
            Word8::C => {
                rl(opcode, &mut self.regfile, |regfile| &mut regfile.r_c);
            }
            Word8::D => {
                rl(opcode, &mut self.regfile, |regfile| &mut regfile.r_d);
            }
            Word8::E => {
                rl(opcode, &mut self.regfile, |regfile| &mut regfile.r_e);
            }
            Word8::H => {
                rl(opcode, &mut self.regfile, |regfile| &mut regfile.r_h);
            }
            Word8::L => {
                rl(opcode, &mut self.regfile, |regfile| &mut regfile.r_l);
            }
            Word8::HL => {
                let addr = self.regfile.get_hl();
                let byte = memory.read_byte(addr);
                let old_carry = self.regfile.get_carry();
                self.regfile.set_carry((byte & 0x80) > 0);
                
                if opcode == Opcode::SLA {
                    // shift
                    let val = byte.rotate_left(1);
                    let val = val & 0xFE; // bit 0 is always zeroed
                    memory.write_byte(addr, val);
                    self.regfile.set_zero(val == 0);
                }
                else {
                    let val = if opcode == Opcode::RLC || opcode == Opcode::RLCA {
                        byte.rotate_left(1)
                    }
                    else {
                        let val = byte.rotate_left(1);
                        if old_carry { val | 0x01 } else { val & 0xFE }
                    };
                    memory.write_byte(addr, val);
                    self.regfile.set_zero(val == 0);
                }
            }
        };
        self.regfile.set_half_carry(false);
        self.regfile.set_sub(false);
    }

    fn rotate_right(&mut self, memory: &mut Memory, target: Word8, opcode: Opcode) {

        fn rr(opcode: Opcode, regfile: &mut Regfile, path: impl Fn(&mut Regfile) -> &mut u8) {
            let current_val = *(path(regfile));
            let old_carry = regfile.get_carry();
            regfile.set_carry((current_val & 0x01) > 0);

            if opcode == Opcode::SRA || opcode == Opcode::SRL  {
                let val = current_val.rotate_right(1);
                let val = if opcode == Opcode::SRL {
                    val & 0x7F
                }
                else { val & (current_val | 0x7F) };
                *(path(regfile)) = val;
                regfile.set_zero(val == 0);
            }
            else {
                let val = if opcode == Opcode::RRC || opcode == Opcode::RRCA {
                    current_val.rotate_right(1)
                }
                else {
                    let val = current_val.rotate_right(1);
                    if old_carry { val | 0x80 } else { val & 0x7F }
                };
                *(path(regfile)) = val;
                regfile.set_zero(val == 0);
            }
        }
        match target {
            Word8::A => {
                rr(opcode, &mut self.regfile, |regfile| &mut regfile.r_a);
            }
            Word8::B => {
                rr(opcode, &mut self.regfile, |regfile| &mut regfile.r_b);
            }
            Word8::C => {
                rr(opcode, &mut self.regfile, |regfile| &mut regfile.r_c);
            }
            Word8::D => {
                rr(opcode, &mut self.regfile, |regfile| &mut regfile.r_d);
            }
            Word8::E => {
                rr(opcode, &mut self.regfile, |regfile| &mut regfile.r_e);
            }
            Word8::H => {
                rr(opcode, &mut self.regfile, |regfile| &mut regfile.r_h);
            }
            Word8::L => {
                rr(opcode, &mut self.regfile, |regfile| &mut regfile.r_l);
            }
            Word8::HL => {
                let addr = self.regfile.get_hl();
                let byte = memory.read_byte(addr);
                let old_carry = self.regfile.get_carry();
                self.regfile.set_carry((byte & 0x80) > 0);

                if opcode == Opcode::SRA || opcode == Opcode::SRL {
                    let val = byte.rotate_right(1);
                    let val = if opcode == Opcode::SRL {
                        val & 0x7F
                    }
                    else { val & (byte | 0x7F) };
                    memory.write_byte(addr, val);
                    self.regfile.set_zero(val == 0);
                }
                else {
                    let val = if opcode == Opcode::RRC || opcode == Opcode::RRCA {
                        byte.rotate_right(1)
                    }
                    else {
                        let val = byte.rotate_right(1);
                        if old_carry { val | 0x80 } else { val & 0x7F }
                    };
                    memory.write_byte(addr, val);
                    self.regfile.set_zero(val == 0);
                }
            }
        };
        self.regfile.set_half_carry(false);
        self.regfile.set_sub(false);
    }

    /*
    // note: assumes a is a uint8_t and wraps from 0xff to 0
    if (!n_flag) {  // after an addition, adjust if (half-)carry occurred or if result is out of bounds
    if (c_flag || a > 0x99) { a += 0x60; c_flag = 1; }
    if (h_flag || (a & 0x0f) > 0x09) { a += 0x6; }
    } else {  // after a subtraction, only adjust if (half-)carry occurred
    if (c_flag) { a -= 0x60; }
    if (h_flag) { a -= 0x6; }
    }
    // these flags are always updated
    z_flag = (a == 0); // the usual z flag
    h_flag = 0; // h flag is always cleared
     */
    fn daa(&mut self) {
        if (self.regfile.get_sub()) { // subtraction, only adjust if (half-)carry occurred
            if (self.regfile.get_carry()) {
                self.regfile.r_a = self.regfile.r_a.wrapping_sub(0x60)
            }
            if (self.regfile.get_half_carry()) {
                self.regfile.r_a = self.regfile.r_a.wrapping_sub(0x06)
            }
        }
        else { // after an addition, adjust if (half-)carry occurred or if result is out of bounds
            if (self.regfile.get_carry() || self.regfile.r_a > 0x99) {
                self.regfile.r_a = self.regfile.r_a.wrapping_add(0x60);
                self.regfile.set_carry(true);
            }
            if (self.regfile.get_half_carry() || (self.regfile.r_a & 0x0F) > 0x09) {
                self.regfile.r_a = self.regfile.r_a.wrapping_add(0x06);
            }
        }
        self.regfile.set_zero(self.regfile.r_a == 0);
        self.regfile.set_half_carry(false);
    }

    fn swap_bits(&mut self, memory: &mut Memory, target: Word8) {
        // swaps upper and lower 4 bits in an 8-bit word
        match target {
            Word8::A => {
                let val = self.regfile.r_a.rotate_left(4);
                self.regfile.r_a = val;
                self.regfile.set_zero(val == 0);
            },
            Word8::B => {
                let val = self.regfile.r_b.rotate_left(4);
                self.regfile.r_b = val;
                self.regfile.set_zero(val == 0);
            },
            Word8::C => {
                let val = self.regfile.r_c.rotate_left(4);
                self.regfile.r_c = val;
                self.regfile.set_zero(val == 0);
            },
            Word8::D => {
                let val = self.regfile.r_d.rotate_left(4);
                self.regfile.r_d = val;
                self.regfile.set_zero(val == 0);
            },
            Word8::E => {
                let val = self.regfile.r_e.rotate_left(4);
                self.regfile.r_e = val;
                self.regfile.set_zero(val == 0);
            },
            Word8::H => {
                let val = self.regfile.r_h.rotate_left(4);
                self.regfile.r_h = val;
                self.regfile.set_zero(val == 0);
            },
            Word8::L => {
                let val = self.regfile.r_l.rotate_left(4);
                self.regfile.r_l = val;
                self.regfile.set_zero(val == 0);
            },
            Word8::HL => {
                let addr = self.regfile.get_hl();
                let byte = memory.read_byte(addr);
                let val = byte.rotate_left(4);
                memory.write_byte(addr, val);
                self.regfile.set_zero(val == 0);
            },
        }
        self.regfile.set_sub(false);
        self.regfile.set_carry(false);
        self.regfile.set_half_carry(false);
    }

    fn bit(&mut self, memory: &mut Memory, target: Word8, bit: u8) {
        // copies the complement of the specified bit in target to zero flag
        let mask = 0x1 << bit; 
        let val = {
            match target {
                Word8::A => self.regfile.r_a,
                Word8::B => self.regfile.r_b,
                Word8::C => self.regfile.r_c,
                Word8::D => self.regfile.r_d,
                Word8::E => self.regfile.r_e,
                Word8::H => self.regfile.r_h,
                Word8::L => self.regfile.r_l,
                Word8::HL => memory.read_byte(self.regfile.get_hl()),
            }
        };
        let bit = val & mask;
        self.regfile.set_zero(bit == 0); // when bit is 1, flag gets set to 0
        self.regfile.set_sub(false);
        self.regfile.set_half_carry(true);
    }

    fn set_bit(&mut self, memory: &mut Memory, target: Word8, bit: u8, state: bool) {
        // sets the specified bit in the target word to the val arg
        let mask = 0x1 << bit; 
        match target {
            Word8::A => {
                let val = self.regfile.r_a;
                let val = { 
                    if state { val | mask }
                    else { val & !mask }
                };
                self.regfile.r_a = val;
            },
            Word8::B => {
                let val = self.regfile.r_b;
                let val = { 
                    if state { val | mask }
                    else { val & !mask }
                };
                self.regfile.r_b = val;
            },
            Word8::C => {
                let val = self.regfile.r_c;
                let val = { 
                    if state { val | mask }
                    else { val & !mask }
                };
                self.regfile.r_c = val;
            },
            Word8::D => {
                let val = self.regfile.r_d;
                let val = { 
                    if state { val | mask }
                    else { val & !mask }
                };
                self.regfile.r_d = val;
            },
            Word8::E => {
                let val = self.regfile.r_e;
                let val = { 
                    if state { val | mask }
                    else { val & !mask }
                };
                self.regfile.r_e = val;
            },
            Word8::H => {
                let val = self.regfile.r_h;
                let val = { 
                    if state { val | mask }
                    else { val & !mask }
                };
                self.regfile.r_h = val;
            },
            Word8::L => {
                let val = self.regfile.r_l;
                let val = { 
                    if state { val | mask }
                    else { val & !mask }
                };
                self.regfile.r_l = val;
            },
            Word8::HL => {
                let addr = self.regfile.get_hl();
                let val = memory.read_byte(addr);
                let val = { 
                    if state { val | mask }
                    else { val & !mask }
                };
                memory.write_byte(addr, val);
            },
        }
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


