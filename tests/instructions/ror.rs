use r_nes::{
    cpu::Status,
    nes::Nes,
};

#[test]
fn test_ror_accumulator() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.bus.cpu_memory[0] = 0x6A;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x7F);
}

#[test]
fn test_ror_accumulator_status_carry() {
    let mut nes = Nes::new();
    nes.cpu.status_register.insert(Status::CARRY);
    nes.cpu.a_register = 0xFF;
    nes.bus.cpu_memory[0] = 0x6A;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xFF);
}

#[test]
fn test_ror_zero_page() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x66, 0x03, 0x00, 0xFF]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x03], 0x7F);
}

#[test]
fn test_ror_zero_page_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x76, 0xFE]);
    nes.bus.cpu_memory[0xFF] = 0xFF;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0xFF], 0x7F);
}

#[test]
fn test_ror_zero_page_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x76, 0xFF, 0x00, 0xFF]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x03], 0x7F);
}

#[test]
fn test_ror_absolute() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x6E, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xFF;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0x7F);
}

#[test]
fn test_ror_absolute_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x7E, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xFF;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0x7F);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_ror_absolute_x_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.x_index_register = 0x05;
//     nes.bus.cpu_memory[0..5].copy_from_slice(&[0x7E, 0xFF, 0x07, 0x00, 0xFF]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.bus.cpu_memory[0x04], 0x7F);
// }
