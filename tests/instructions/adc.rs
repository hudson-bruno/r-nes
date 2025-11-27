use r_nes::{cartridge::Cartridge, cpu::Status, nes::Nes};

#[test]
fn test_adc_immediate() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x69, 0x05]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x04;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_immediate_status_carry() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x69, 0x02]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xFF;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::CARRY));
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_adc_immediate_status_carry_with_carry_set() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x69, 0x02]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xFF;
    nes.cpu.status_register.insert(Status::CARRY);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::CARRY));
    assert_eq!(nes.cpu.a_register, 0x02);
}

#[test]
fn test_adc_immediate_status_zero() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x69, 0x00]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x00;

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
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x69, 0x40]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x40;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::OVERFLOW));
    assert_eq!(nes.cpu.a_register, 0x80);

    // Case 2: Negative + Negative = Positive (Overflow)
    // A = 0x80 (-128), M = 0x80 (-128), Carry = 0
    // Result = 0x00 (0) (with carry set)
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x69, 0x80]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x80;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::OVERFLOW));
    assert!(nes.cpu.status_register.contains(Status::CARRY)); // Also expect carry in this case
    assert_eq!(nes.cpu.a_register, 0x00);

    // Case 3: Positive + Negative = Positive (No Overflow)
    // A = 0x40 (64), M = 0xC0 (-64), Carry = 0
    // Result = 0x00 (0)
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x69, 0xC0]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x40;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(!nes.cpu.status_register.contains(Status::OVERFLOW));
    assert!(nes.cpu.status_register.contains(Status::CARRY));
    assert_eq!(nes.cpu.a_register, 0x00);
}

#[test]
fn test_adc_immediate_status_negative() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x69, 0x01]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x7F;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
    assert_eq!(nes.cpu.a_register, 0x80);
}

#[test]
fn test_adc_zero_page() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0003].copy_from_slice(&[0x65, 0x03, 0x00, 0x05]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x04;
    nes.bus.cpu_memory[0x0003] = 0x05;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_zero_page_x() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x75, 0xFE]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x04;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0x00FF] = 0x05;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_zero_page_x_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x75, 0xFF]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x04;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0x0003] = 0x05;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_absolute() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x6D, 0xFF, 0x07]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x04;
    nes.bus.cpu_memory[0x07FF] = 0x05;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_absolute_x() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x7D, 0xFE, 0x07]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x04;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0x07FF] = 0x05;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_absolute_y() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x79, 0xFE, 0x07]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x04;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0x07FF] = 0x05;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_indirect_x() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x61, 0x02, 0x00]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x04;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0x0003..=0x0004].copy_from_slice(&[0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x05;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_indirect_x_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x61, 0xFF, 0x00]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x04;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0x0003..=0x0004].copy_from_slice(&[0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x05;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_indirect_y() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x71, 0x03, 0x00]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x04;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0x0003..=0x0004].copy_from_slice(&[0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x05;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}

#[test]
fn test_adc_indirect_y_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x71, 0x03, 0x00]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x04;
    nes.cpu.y_index_register = 0x06;
    nes.bus.cpu_memory[0x0003..=0x0005].copy_from_slice(&[0xFF, 0x07, 0x05]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x09);
}
