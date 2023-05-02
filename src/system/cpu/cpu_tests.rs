#[cfg(test)]
mod cpu_tests {
    use crate::system::CPU;
    use crate::system::Memory;
    use crate::system::cpu::regfile;
    use crate::system::cpu::regfile::Regfile;

    #[test]
    fn NOP0x0() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let regfile = Regfile::new();
        memory.write_byte(0, 0x0);

        cpu.run(&mut memory);
        assert_eq!(cpu.regfile, regfile);
        assert_eq!(1, cpu.pc);
    }
}