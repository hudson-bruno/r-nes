use r_nes::cpu::{Cpu, ExitStatus};

#[test]
fn test_asl_accumulator() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x01;
    cpu.memory[0] = 0x0A;

    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0x02);
}

#[test]
fn test_asl_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory[0..4].copy_from_slice(&[0x06, 0x03, 0x00, 0x01]);
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0x03], 0x02);
}

#[test]
fn test_asl_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x01;
    cpu.memory[0..2].copy_from_slice(&[0x16, 0xFE]);
    cpu.memory[0xFF] = 0x01;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0xFF], 0x02);
}

#[test]
fn test_asl_zero_page_x_overflow() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x04;
    cpu.memory[0..4].copy_from_slice(&[0x16, 0xFF, 0x00, 0x01]);
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0x03], 0x02);
}

#[test]
fn test_asl_absolute() {
    let mut cpu = Cpu::new();
    cpu.memory[0..3].copy_from_slice(&[0x0E, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0x80;

    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0x07FF], 0x00);
}

#[test]
fn test_asl_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0x1E, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0x01;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0x07FF], 0x02);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_asl_absolute_x_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.x_index_register = 0x05;
//     cpu.memory[0..5].copy_from_slice(&[0x1E, 0xFF, 0x07, 0x00, 0x01]);
//     let result = cpu.run();
//
//     assert_eq!(result, ExitStatus::Brk);
//     assert_eq!(cpu.memory[0x04], 0x02);
// }
