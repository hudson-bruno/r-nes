use r_nes::{cpu::Status, nes::Nes};

#[test]
fn test_cpy_immediate_status_carry() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0x02;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0xC0, 0x01]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cpy_immediate_status_zero() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0xC0, 0x01]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cpy_immediate_status_negative() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0xFF;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0xC0, 0x01]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_cpy_zero_page_carry() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0x02;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xC4, 0x03, 0x00, 0x01]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cpy_zero_page_zero() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xC4, 0x03, 0x00, 0x01]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cpy_zero_page_negative() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0xFF;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xC4, 0x03, 0x00, 0x01]);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_cpy_absolute_carry() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0x02;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0xCC, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x01;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cpy_absolute_zero() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0xCC, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x01;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cpy_absolute_negative() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0xFF;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0xCC, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x01;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
}
