use r_nes::cpu::{Cpu, ExitStatus};

#[test]
fn test_ora_immediate() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.memory[0..3].copy_from_slice(&[0x09, 0xBA, 0x00]);

    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_zero_page() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.memory[0..4].copy_from_slice(&[0x05, 0x03, 0x00, 0xBA]);

    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_absolute() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.memory[0..3].copy_from_slice(&[0x0D, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0xBA;

    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_indirect_x() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.x_index_register = 0x01;
    cpu.memory[0..5].copy_from_slice(&[0x01, 0x02, 0x00, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0xBA;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_indirect_x_overflow() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.x_index_register = 0x04;
    cpu.memory[0..5].copy_from_slice(&[0x01, 0xFF, 0x00, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0xBA;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB | 0xBA);
}
