use r_nes::nes::Nes;

#[test]
fn test_rol_accumulator() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.bus.cpu_memory[0] = 0x2A;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xFE);
}

#[test]
fn test_rol_zero_page() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x26, 0x03, 0x00, 0xFF]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x03], 0xFE);
}

#[test]
fn test_rol_zero_page_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x36, 0xFE]);
    nes.bus.cpu_memory[0xFF] = 0xFF;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0xFF], 0xFE);
}

#[test]
fn test_rol_zero_page_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x36, 0xFF, 0x00, 0xFF]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x03], 0xFE);
}

#[test]
fn test_rol_absolute() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x2E, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xFF;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0xFE);
}

#[test]
fn test_rol_absolute_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x3E, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xFF;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0xFE);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_rol_absolute_x_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.x_index_register = 0x05;
//     nes.bus.cpu_memory[0..5].copy_from_slice(&[0x3E, 0xFF, 0x07, 0x00, 0xFF]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.bus.cpu_memory[0x04], 0xFE);
// }
