use r_nes::nes::Nes;

#[test]
fn test_asl_accumulator() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x01;
    nes.bus.cpu_memory[0] = 0x0A;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x02);
}

#[test]
fn test_asl_zero_page() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x06, 0x03, 0x00, 0x01]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x03], 0x02);
}

#[test]
fn test_asl_zero_page_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x16, 0xFE]);
    nes.bus.cpu_memory[0xFF] = 0x01;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0xFF], 0x02);
}

#[test]
fn test_asl_zero_page_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x16, 0xFF, 0x00, 0x01]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x03], 0x02);
}

#[test]
fn test_asl_absolute() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x0E, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x80;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0x00);
}

#[test]
fn test_asl_absolute_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x1E, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x01;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0x02);
}

// At the moment the nes does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_asl_absolute_x_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.x_index_register = 0x05;
//     nes.bus.cpu_memory[0..5].copy_from_slice(&[0x1E, 0xFF, 0x07, 0x00, 0x01]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.bus.cpu_memory[0x04], 0x02);
// }
