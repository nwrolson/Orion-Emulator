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
}