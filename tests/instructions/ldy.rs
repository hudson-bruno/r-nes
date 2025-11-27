use r_nes::nes::Nes;

#[test]
fn test_ldy_immediate() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0xA0, 0x01, 0x00]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_zero_page() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xA4, 0x03, 0x00, 0x01]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_zero_page_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0xB4, 0xFE, 0x00]);
    nes.bus.cpu_memory[0xFF] = 0x01;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_zero_page_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xB4, 0xFF, 0x00, 0x01]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_absolute() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xAC, 0xFF, 0x07, 0x00]);
    nes.bus.cpu_memory[0x07FF] = 0x01;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_absolute_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xBC, 0xFE, 0x07, 0x00]);
    nes.bus.cpu_memory[0x07FF] = 0x01;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_ldy_absolute_x_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.x_index_register = 0x05;
//     nes.bus.cpu_memory[0..5].copy_from_slice(&[0xBC, 0xFF, 0x07, 0x00, 0x01]);
//
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.cpu.a_register, 0x01);
// }
