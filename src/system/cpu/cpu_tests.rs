use super::{CPU, regfile::Regfile, memory::Memory};

#[cfg(test)]
mod cpu_tests {
    use crate::system::CPU;
    use crate::system::Memory;
    use crate::system::cpu::regfile::Regfile;

    // INS TARGET
    // Instruction Length in Bytes, Cycle Amount
    // Z N H C flag registers

    #[test]
    fn NOP_0x0() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let regfile = Regfile::new();
        memory.write_byte(0, 0x0);

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(1, cpu.pc);
    }

    // Unary 16-bit word Opcodes
    #[test]
    fn INC_0x3() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0x3);

        regfile.set_bc(1); 
        cpu.run(&mut memory);

        assert_eq!(cpu.regfile, regfile);
    }

    #[test]
    fn DEC_0xB() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0xB);

        regfile.set_bc(0xFFFF); 
        cpu.run(&mut memory);

        assert_eq!(cpu.regfile, regfile);
    }

    // Arithmetic Opcodes
    #[test]
    fn INC_0x4() {
        // INC B
        // 1 4
        // Z 0 H -
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0x4);

        cpu.regfile.set_sub(true); // verify it gets overwritten
        regfile.set_sub(false);
        regfile.r_b = 1;

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(1, cpu.pc);

        // test half carry and zero
        cpu.regfile.r_b = 0xFF;
        regfile.r_b = 0;
        regfile.set_half_carry(true);
        regfile.set_zero(true);
        memory.write_byte(1, 0x4);

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
    }

    #[test]
    fn DEC_0x5() {
        // DEC B
        // 1 4
        // Z 1 H -
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0x5);

        cpu.regfile.set_sub(false); // verify it gets overwritten
        cpu.regfile.r_b = 1;
        regfile.set_sub(true);
        regfile.set_zero(true);
        regfile.r_b = 0;

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(1, cpu.pc);

        // test half carry
        cpu.regfile.r_b = 0;
        regfile.r_b = 0xFF;
        regfile.set_half_carry(true);
        regfile.set_zero(false);
        memory.write_byte(1, 0x5);

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
    }

    #[test]
    fn INC_0x34() {
        // INC (HL)
        // 1 12
        // Z 0 H -
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0x34);

        cpu.regfile.set_sub(true); // verify it gets overwritten
        cpu.regfile.set_hl(0x7);
        regfile.set_sub(false);
        regfile.set_hl(0x7);

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(1, memory.read_byte(0x7));

        // test half carry and zero
        memory.write_byte(0x7, 0xFF);
        regfile.set_half_carry(true);
        regfile.set_zero(true);
        memory.write_byte(1, 0x34);

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(0, memory.read_byte(0x7))
    }

    #[test]
    fn DEC_0x35() {
        // DEC (HL)
        // 1 12
        // Z 1 H -
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0x35);

        cpu.regfile.set_sub(false); // verify it gets overwritten
        cpu.regfile.set_hl(0x7);
        regfile.set_sub(true);
        regfile.set_hl(0x7);
        memory.write_byte(0x7, 2);

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(1, memory.read_byte(0x7));

        // test half carry and zero
        memory.write_byte(0x7, 0x0);
        regfile.set_half_carry(true);
        regfile.set_zero(false);
        memory.write_byte(1, 0x35);

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(0xFF, memory.read_byte(0x7))
    }

    #[test]
    fn ADD_0x80() {
        // ADD A,B
        // 1 4
        // Z 0 H C
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0x80);

        cpu.regfile.r_b = 7;
        cpu.regfile.set_sub(true);
        regfile.r_a = 7;
        regfile.r_b = 7;
        cpu.run(&mut memory);

        assert_eq!(cpu.regfile, regfile);

        cpu.regfile.r_a = 0xFF;
        cpu.regfile.r_b = 1;
        memory.write_byte(1, 0x80);

        regfile.r_a = 0;
        regfile.r_b = 1;
        regfile.set_zero(true);
        regfile.set_carry(true);
        regfile.set_half_carry(true);
        cpu.run(&mut memory);

        assert_eq!(cpu.regfile, regfile);
    }

    #[test]
    fn SUB_0x90() {
        // ADD A,B
        // 1 4
        // Z 0 H C
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0x90);

        cpu.regfile.r_b = 0x0F;
        regfile.r_a = 0xF1;
        regfile.r_b = 0xF;
        regfile.set_sub(true);
        regfile.set_half_carry(true);
        regfile.set_carry(true);
        cpu.run(&mut memory);

        assert_eq!(cpu.regfile, regfile);

        cpu.regfile.r_a = 0xFF;
        cpu.regfile.r_b = 1;
        memory.write_byte(1, 0x90);

        regfile.r_a = 0xFE;
        regfile.r_b = 1;
        regfile.set_carry(false);
        regfile.set_half_carry(false);
        cpu.run(&mut memory);

        assert_eq!(cpu.regfile, regfile);
    }

    #[test]
    fn ADC_0x88() {
        // ADC A,B
        // 1 4
        // Z 0 H C
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0x88);

        cpu.regfile.r_b = 7;
        cpu.regfile.set_carry(false);
        cpu.regfile.set_sub(true);
        regfile.r_a = 7;
        regfile.r_b = 7;
        cpu.run(&mut memory);

        assert_eq!(cpu.regfile, regfile);

        cpu.regfile.r_a = 0xFE;
        cpu.regfile.r_b = 1;
        cpu.regfile.set_carry(true);
        memory.write_byte(1, 0x88);

        regfile.r_a = 0;
        regfile.r_b = 1;
        regfile.set_zero(true);
        regfile.set_carry(true);
        regfile.set_half_carry(true);
        cpu.run(&mut memory);

        assert_eq!(cpu.regfile, regfile);
    }

    // Load Opcodes
    #[test]
    fn LOAD_0x02() {
        // LOAD (BC),A
        // 1 8
        // - - - -
        // Memory address at (BC) is target, A is source
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0x02);

        cpu.regfile.set_bc(0xF);
        regfile.set_bc(0xF);
        cpu.regfile.r_a = 7;
        regfile.r_a = 7;

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(7, memory.read_byte(0xF));
    }

    #[test]
    fn LOAD_0x22() {
        // LOAD (HL+),A
        // 1 8
        // - - - -
        // Memory address at (HL) is target, A is source
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0x22);

        cpu.regfile.set_hl(0x0F);
        regfile.set_hl(0x10);
        cpu.regfile.r_a = 7;
        regfile.r_a = 7;

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(7, memory.read_byte(0xF));
    }

     // Rotate Opcodes
     #[test]
     fn RLCA_0x07() {
        // RLCA
        // 1 4
        // 0 0 0 C
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0x07);

        cpu.regfile.r_a = 0x85;
        regfile.r_a = 0x0B;
        regfile.set_carry(true);
        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
     }

     #[test]
     fn CB_RLC_0x00() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0xCB);
        memory.write_byte(1, 0x00);

        cpu.regfile.r_b = 0x85;
        regfile.r_b = 0x0B;
        regfile.set_carry(true);
        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(cpu.pc, 2);
     }

     #[test]
     fn RRCA_0x0F() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();

        memory.write_byte(0, 0x0F);

        cpu.regfile.r_a = 0x3B;
        regfile.r_a = 0x9D;
        regfile.set_carry(true);

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);

     }

     #[test]
     fn CB_RR_0x1E() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0xCB);
        memory.write_byte(1, 0x1E);

        memory.write_byte(0x07, 0x81);
        cpu.regfile.set_hl(0x07);
        regfile.set_hl(0x07);
        regfile.set_carry(true);

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(0x40, memory.read_byte(0x07));
        assert_eq!(cpu.pc, 2);
     }

    // Stack Opcodes
    #[test]
    fn PUSH_POP_BC_0xC5_0xD1() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut regfile = Regfile::new();
        memory.write_byte(0, 0xC5);
        memory.write_byte(1, 0xD1);

        cpu.regfile.set_bc(0xABCD);
        regfile.set_bc(0xABCD);

        cpu.run(&mut memory); // PUSH BC
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(cpu.sp, 0xFFFC); // should be decremented by 2
        assert_eq!(0xAB, memory.read_byte(0xFFFD));
        assert_eq!(0xCD, memory.read_byte(0xFFFC));

        regfile.set_de(0xABCD);
        cpu.run(&mut memory); // POP DE
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(cpu.sp, 0xFFFE);
    }
}

#[test]
fn JP_0xCA() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    let mut regfile = Regfile::new();
    memory.write_byte(0, 0xCA); // JP Z
    memory.write_byte(1, 0xAB); // msb of address
    memory.write_byte(2, 0xCD); // lsb of address
    // jump to 0xABCD if Zero flag is true
    // otherwise advance to line 3 of memory



    cpu.run(&mut memory);
    assert_eq!(cpu.pc, 3);

    regfile.set_zero(true);
    cpu.regfile.set_zero(true);
    cpu.pc = 0;
    cpu.run(&mut memory);

    assert_eq!(cpu.regfile, regfile);
    assert_eq!(cpu.pc, 0xABCD);
}

#[test]
fn JR_0x30() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    let mut regfile = Regfile::new();
    memory.write_byte(0, 0x30); // JR NC
    memory.write_byte(1, 0x05); // two's complement byte to advance PC by

    cpu.run(&mut memory);
    assert_eq!(cpu.pc, 0x07); // 0x05 + 0x02
    assert_eq!(cpu.regfile, regfile);

    regfile.set_carry(true);
    cpu.regfile.set_carry(true);
    cpu.pc = 0;

    cpu.run(&mut memory);
    assert_eq!(cpu.regfile, regfile);
    assert_eq!(cpu.pc, 0x02);
}

#[test]
fn CALL_RETURN_0xCC_0xC8() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    memory.write_byte(0, 0xCC); // CALL Z
    memory.write_byte(1, 0xAB);
    memory.write_byte(2, 0xCD);
    memory.write_byte(0xABCD, 0xC8);

    cpu.regfile.set_zero(false);

    cpu.run(&mut memory); // CALL Z, zero is false
    assert_eq!(cpu.pc, 0x3);
    assert_eq!(cpu.sp, 0xFFFE);
    assert_eq!(memory.read_byte(0xFFFE), 0x00);
    assert_eq!(memory.read_byte(0xFFFD), 0x00);

    cpu.regfile.set_zero(true);
    cpu.pc = 0;
    
    cpu.run(&mut memory); // CALL Z, zero is true
    assert_eq!(cpu.pc, 0xABCD);
    assert_eq!(cpu.sp, 0xFFFC);
    assert_eq!(memory.read_byte(0xFFFD), 0x00);
    assert_eq!(memory.read_byte(0xFFFC), 0x03);

    cpu.run(&mut memory); // RET Z, zero is true
    assert_eq!(cpu.pc, 0x03);
    assert_eq!(cpu.sp, 0xFFFE);
}

#[test]
fn EI_DI_0xFB_0xF4() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    memory.write_byte(0, 0xF3); // DI
    memory.write_byte(1, 0xFB); // EI
    memory.write_byte(2, 0x00); // just showing there's a NOP here
    memory.write_byte(3, 0xF3);
    memory.write_byte(4, 0xFB);
    memory.write_byte(5, 0xF3);

    cpu.run(&mut memory); // DI
    assert_eq!(cpu.ime, false);

    // EI doesn't take effect until after next instruction
    cpu.run(&mut memory); // EI
    assert_eq!(cpu.ime, false);
    cpu.run(&mut memory); // NOP
    assert_eq!(cpu.ime, true);

    // if EI is executed, then immediately followed with a DI,
    // then ime is never set to true
    cpu.run(&mut memory); // DI
    assert_eq!(cpu.ime, false);
    cpu.run(&mut memory); // EI
    assert_eq!(cpu.ime, false);
    cpu.run(&mut memory); // DI
    assert_eq!(cpu.ime, false);

    
}


