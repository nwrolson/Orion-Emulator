use super::{CPU, Memory};
#[cfg(test)]

#[test]
fn timer_inc() {
    // timer should increment after enough cpu cycles have passed

    let mut cpu = CPU::new();
    let mut memory = Memory::new();

    // enable timer and set it to update every 16 CPU cycles
    memory.write_byte(0xFF07, 0x5); // set TAC to 101
    assert_eq!(0x5, memory.timer.get_TAC());

    assert_eq!(memory.read_byte(0xFF05), 0x00);

    // run CPU 16 steps, each instruction being interrupted as NOP (0x00)
    for _ in 0..16 { 
        cpu.run(&mut memory);
    }

    assert_eq!(memory.read_byte(0xFF05), 0x01);
}

#[test]
fn timer_overflow() {
    // timer should overflow from 0xFF to 0

    let mut cpu = CPU::new();
    let mut memory = Memory::new();

    // enable timer and set it to update every 16 CPU cycles
    memory.write_byte(0xFF07, 0x5); // set TAC to 101
    memory.write_byte(0xFFFF, 0xFF); // enable interrupts
    assert_eq!(0x5, memory.timer.get_TAC());

    assert_eq!(memory.read_byte(0xFF05), 0x00);

    // run until timer should overflow
    for _ in 0..((16*0xFF)) { 
        cpu.run(&mut memory);
    }
    assert_eq!(memory.read_byte(0xFF05), 0xFF);

    for _ in 0..16 { 
        assert_eq!(memory.read_byte(0xFF05), 0xFF);
        cpu.run(&mut memory);
    }
    assert_eq!(memory.read_byte(0xFF05), 0x00);
    assert_eq!(cpu.get_pc(), 0x50); // Timer interrupt recognized by CPU
}