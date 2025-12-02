use r_nes::{cartridge::Cartridge, cpu::Status, nes::Nes};

#[test]
fn test_sbc_immediate() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xE9, 0x04]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x06;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_sbc_immediate_status_carry() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xE9, 0x01]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x00;

    let result = nes.step();

    assert!(result.is_none());
    assert!(!nes.cpu.status_register.contains(Status::CARRY));
    assert_eq!(nes.cpu.a_register, 0xFE);
}

#[test]
fn test_sbc_immediate_status_carry_with_carry_set() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xE9, 0x04]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x06;
    nes.cpu.status_register.insert(Status::CARRY);
    let result = nes.step();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::CARRY));
    assert_eq!(nes.cpu.a_register, 0x02);
}

#[test]
fn test_sbc_immediate_status_zero() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xE9, 0x01]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x02;
    let result = nes.step();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::ZERO));
    assert_eq!(nes.cpu.a_register, 0x00);
}

#[test]
fn test_sbc_immediate_status_overflow() {
    // Case 1: Positive - Negative = Positive (Overflow)
    // A = 0x70 (112), M = 0xF0 (-16), Carry = 1 (no borrow)
    // SBC: 0x70 - (-0x10) - 0 = 0x80 (128). Overflow.
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xE9, 0xF0]); // SBC immediate 0xF0
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x70;
    nes.cpu.status_register.insert(Status::CARRY); // C=1 for no borrow
    let result = nes.step();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::OVERFLOW));
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
    assert!(!nes.cpu.status_register.contains(Status::CARRY)); // A borrow IS needed (0x70 < 0xF0 unsigned)
    assert_eq!(nes.cpu.a_register, 0x80);

    // Case 2: Negative - Positive = Positive (Overflow)
    // A = 0x80 (-128), M = 0x10 (16), Carry = 1 (no borrow)
    // SBC: 0x80 - 0x10 - 0 = 0x70 (112). Result -144 (underflow).
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xE9, 0x10]); // SBC immediate 0x10
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x80;
    nes.cpu.status_register.insert(Status::CARRY); // C=1 for no borrow
    let result = nes.step();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::OVERFLOW));
    assert!(!nes.cpu.status_register.contains(Status::NEGATIVE)); // Result 0x70 is positive
    assert!(nes.cpu.status_register.contains(Status::CARRY)); // No borrow needed (0x80 >= 0x10 unsigned)
    assert_eq!(nes.cpu.a_register, 0x70);

    // Case 3: Positive - Positive = Positive (No Overflow)
    // A = 0x40 (64), M = 0x10 (16), Carry = 1 (no borrow)
    // SBC: 0x40 - 0x10 - 0 = 0x30 (48). No overflow.
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xE9, 0x10]); // SBC immediate 0x10
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x40;
    nes.cpu.status_register.insert(Status::CARRY); // C=1 for no borrow
    let result = nes.step();

    assert!(result.is_none());
    assert!(!nes.cpu.status_register.contains(Status::OVERFLOW));
    assert!(!nes.cpu.status_register.contains(Status::NEGATIVE));
    assert!(nes.cpu.status_register.contains(Status::CARRY));
    assert_eq!(nes.cpu.a_register, 0x30);

    // Case 4: Positive - Positive = Negative (No Overflow, but borrow occurs)
    // A = 0x10 (16), M = 0x40 (64), Carry = 0 (borrow intended)
    // SBC: 0x10 - 0x40 - 1 = 0xCF (-49). No overflow.
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xE9, 0x40]); // SBC immediate 0x40
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x10;
    nes.cpu.status_register.remove(Status::CARRY); // C=0 for borrow
    let result = nes.step();

    assert!(result.is_none());
    assert!(!nes.cpu.status_register.contains(Status::OVERFLOW));
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
    assert!(!nes.cpu.status_register.contains(Status::CARRY));
    assert_eq!(nes.cpu.a_register, 0xCF);
}

#[test]
fn test_sbc_immediate_status_negative() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xE9, 0x01]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x00;
    let result = nes.step();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
    assert_eq!(nes.cpu.a_register, 0xFE);
}

#[test]
fn test_sbc_zero_page() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xE5, 0x03]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x06;
    nes.bus.cpu_memory[0x0003] = 0x04;
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_sbc_zero_page_x() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xF5, 0xFE]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x06;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0x00FF] = 0x04;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_sbc_zero_page_x_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xF5, 0xFF]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x06;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0x0003] = 0x04;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_sbc_absolute() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0xED, 0xFF, 0x07]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x06;
    nes.bus.cpu_memory[0x07FF] = 0x04;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_sbc_absolute_x() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0xFD, 0xFE, 0x07]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x06;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0x07FF] = 0x04;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_sbc_absolute_y() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0xF9, 0xFE, 0x07]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x06;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0x07FF] = 0x04;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_sbc_indirect_x() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xE1, 0x02]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x06;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0x0003..=0x0004].copy_from_slice(&[0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x04;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_sbc_indirect_x_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xE1, 0xFF]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x06;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0x0003..=0x0004].copy_from_slice(&[0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x04;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_sbc_indirect_y() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xF1, 0x03]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0x06;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0x0003..=0x0004].copy_from_slice(&[0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x04;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}
