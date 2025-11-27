use r_nes::{cpu::Status, nes::Nes};

#[test]
fn test_bit_zero_page_status_zero() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0b0011_1111;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x24, 0x03, 0x00, 0b0000_0000]);
    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_bit_zero_page_status_overflow() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0b0011_1111;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x24, 0x03, 0x00, 0b0100_0001]);
    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::OVERFLOW));
}

#[test]
fn test_bit_zero_page_status_negative() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0b0011_1111;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x24, 0x03, 0x00, 0b1000_0001]);
    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_bit_absolute_status_zero() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0b0011_1111;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x2C, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0b0000_0000;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_bit_absolute_status_overflow() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0b0011_1111;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x2C, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0b0100_0001;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::OVERFLOW));
}

#[test]
fn test_bit_absolute_status_negative() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0b0011_1111;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x2C, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0b1000_0001;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
}
