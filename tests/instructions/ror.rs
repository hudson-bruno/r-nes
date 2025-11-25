use r_nes::cpu::{Cpu, ExitStatus, Status};

#[test]
fn test_ror_accumulator() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.memory[0] = 0x6A;

    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0x7F);
}

#[test]
fn test_ror_accumulator_status_carry() {
    let mut cpu = Cpu::new();
    cpu.status_register.insert(Status::CARRY);
    cpu.a_register = 0xFF;
    cpu.memory[0] = 0x6A;

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.a_register, 0xFF);
}

#[test]
fn test_ror_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory[0..4].copy_from_slice(&[0x66, 0x03, 0x00, 0xFF]);
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0x03], 0x7F);
}

#[test]
fn test_ror_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x01;
    cpu.memory[0..2].copy_from_slice(&[0x76, 0xFE]);
    cpu.memory[0xFF] = 0xFF;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0xFF], 0x7F);
}

#[test]
fn test_ror_zero_page_x_overflow() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x04;
    cpu.memory[0..4].copy_from_slice(&[0x76, 0xFF, 0x00, 0xFF]);
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0x03], 0x7F);
}

#[test]
fn test_ror_absolute() {
    let mut cpu = Cpu::new();
    cpu.memory[0..3].copy_from_slice(&[0x6E, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0xFF;

    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0x07FF], 0x7F);
}

#[test]
fn test_ror_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0x7E, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0xFF;
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0x07FF], 0x7F);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_ror_absolute_x_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.x_index_register = 0x05;
//     cpu.memory[0..5].copy_from_slice(&[0x7E, 0xFF, 0x07, 0x00, 0xFF]);
//     let result = cpu.run();
//
//     assert_eq!(result, ExitStatus::Brk);
//     assert_eq!(cpu.memory[0x04], 0x7F);
// }
