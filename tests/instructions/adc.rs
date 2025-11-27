use r_nes::{cpu::Status, nes::Nes};

#[test]
fn test_adc_immediate() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x04;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x69, 0x05]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_immediate_status_carry() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x69, 0x02]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::CARRY));
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_adc_immediate_status_carry_with_carry_set() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.cpu.status_register.insert(Status::CARRY);
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x69, 0x02]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::CARRY));
    assert_eq!(nes.cpu.a_register, 0x02);
}

#[test]
fn test_adc_immediate_status_zero() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x00;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x69, 0x00]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(!nes.cpu.status_register.contains(Status::CARRY));
    assert_eq!(nes.cpu.a_register, 0x00);
}

#[test]
fn test_adc_immediate_status_overflow() {
    // Case 1: Positive + Positive = Negative (Overflow)
    // A = 0x40 (64), M = 0x40 (64), Carry = 0
    // Result = 0x80 (-128)
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x40;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x69, 0x40]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::OVERFLOW));
    assert_eq!(nes.cpu.a_register, 0x80);

    // Case 2: Negative + Negative = Positive (Overflow)
    // A = 0x80 (-128), M = 0x80 (-128), Carry = 0
    // Result = 0x00 (0) (with carry set)
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x80;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x69, 0x80]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::OVERFLOW));
    assert!(nes.cpu.status_register.contains(Status::CARRY)); // Also expect carry in this case
    assert_eq!(nes.cpu.a_register, 0x00);

    // Case 3: Positive + Negative = Positive (No Overflow)
    // A = 0x40 (64), M = 0xC0 (-64), Carry = 0
    // Result = 0x00 (0)
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x40;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x69, 0xC0]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(!nes.cpu.status_register.contains(Status::OVERFLOW));
    assert!(nes.cpu.status_register.contains(Status::CARRY));
    assert_eq!(nes.cpu.a_register, 0x00);
}

#[test]
fn test_adc_immediate_status_negative() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x7F;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x69, 0x01]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
    assert_eq!(nes.cpu.a_register, 0x80);
}

#[test]
fn test_adc_zero_page() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x04;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x65, 0x03, 0x00, 0x05]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_zero_page_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x04;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x75, 0xFE]);
    nes.bus.cpu_memory[0xFF] = 0x05;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_zero_page_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x04;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x75, 0xFF, 0x00, 0x05]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_absolute() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x04;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x6D, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x05;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_absolute_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x04;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x7D, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x05;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_absolute_y() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x04;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x79, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x05;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_indirect_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x04;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x61, 0x02, 0x00, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x05;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_indirect_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x04;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x61, 0xFF, 0x00, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x05;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_indirect_y() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x04;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x71, 0x03, 0x00, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x05;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

// At the moment the nes does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_adc_indirect_y_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.a_register = 0x04;
//     nes.cpu.y_index_register = 0x06;
//     nes.bus.cpu_memory[0..6].copy_from_slice(&[0x71, 0x03, 0x00, 0xFF, 0x07, 0x05]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.cpu.a_register, 0x09);
// }
