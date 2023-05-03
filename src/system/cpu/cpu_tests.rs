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
}