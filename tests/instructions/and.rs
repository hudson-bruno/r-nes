use r_nes::cpu::{Cpu, ExitStatus};

#[test]
fn test_and_immediate() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.memory[0..2].copy_from_slice(&[0x29, 0xBA]);
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_zero_page() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.memory[0..4].copy_from_slice(&[0x25, 0x03, 0x00, 0xBA]);
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.x_index_register = 0x01;
    cpu.memory[0..2].copy_from_slice(&[0x35, 0xFE]);
    cpu.memory[0xFF] = 0xBA;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_zero_page_x_overflow() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.x_index_register = 0x04;
    cpu.memory[0..4].copy_from_slice(&[0x35, 0xFF, 0x00, 0xBA]);
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_absolute() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.memory[0..3].copy_from_slice(&[0x2D, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0xBA;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.x_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0x3D, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0xBA;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB & 0xBA);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_and_absolute_x_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.a_register = 0xAB;
//     cpu.x_index_register = 0x05;
//     cpu.memory[0..5].copy_from_slice(&[0x3D, 0xFF, 0x07, 0x00, 0xBA]);
//     let result = cpu.run();
//
//     assert_eq!(result, ExitStatus::Brk);
//     assert_eq!(cpu.a_register, 0xAB & 0xBA);
// }

#[test]
fn test_and_absolute_y() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.y_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0x39, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0xBA;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB & 0xBA);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_and_absolute_y_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.a_register = 0xAB;
//     cpu.y_index_register = 0x05;
//     cpu.memory[0..5].copy_from_slice(&[0x39, 0xFF, 0x07, 0x00, 0xBA]);
//     let result = cpu.run();
//
//     assert_eq!(result, ExitStatus::Brk);
//     assert_eq!(cpu.a_register, 0xAB & 0xBA);
// }

#[test]
fn test_and_indirect_x() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.x_index_register = 0x01;
    cpu.memory[0..5].copy_from_slice(&[0x21, 0x02, 0x00, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0xBA;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_indirect_x_overflow() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.x_index_register = 0x04;
    cpu.memory[0..5].copy_from_slice(&[0x21, 0xFF, 0x00, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0xBA;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_indirect_y() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xAB;
    cpu.y_index_register = 0x01;
    cpu.memory[0..5].copy_from_slice(&[0x31, 0x03, 0x00, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0xBA;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0xAB & 0xBA);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_and_indirect_y_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.a_register = 0xAB;
//     cpu.y_index_register = 0x06;
//     cpu.memory[0..6].copy_from_slice(&[0x31, 0x03, 0x00, 0xFF, 0x07, 0xBA]);
//     let result = cpu.run();
//
//     assert_eq!(result, ExitStatus::Brk);
//     assert_eq!(cpu.a_register, 0xAB & 0xBA);
// }
