use r_nes::nes::Nes;

#[test]
fn test_lda_immediate() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0xA9, 0x01, 0x00]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_lda_zero_page() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xA5, 0x03, 0x00, 0x01]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_lda_zero_page_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0xB5, 0xFE, 0x00]);
    nes.bus.cpu_memory[0xFF] = 0x01;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_lda_zero_page_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xB5, 0xFF, 0x00, 0x01]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_lda_absolute() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xAD, 0xFF, 0x07, 0x00]);
    nes.bus.cpu_memory[0x07FF] = 0x01;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_lda_absolute_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xBD, 0xFE, 0x07, 0x00]);
    nes.bus.cpu_memory[0x07FF] = 0x01;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_lda_absolute_x_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.x_index_register = 0x05;
//     nes.bus.cpu_memory[0..5].copy_from_slice(&[0xBD, 0xFF, 0x07, 0x00, 0x01]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.cpu.a_register, 0x01);
// }

#[test]
fn test_lda_absolute_y() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xB9, 0xFE, 0x07, 0x00]);
    nes.bus.cpu_memory[0x07FF] = 0x01;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_lda_absolute_y_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.y_index_register = 0x05;
//     nes.bus.cpu_memory[0..5].copy_from_slice(&[0xB9, 0xFF, 0x07, 0x00, 0x01]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.cpu.a_register, 0x01);
// }

#[test]
fn test_lda_indirect_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0xA1, 0x02, 0x00, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x01;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_lda_indirect_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0xA1, 0xFF, 0x00, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x01;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

#[test]
fn test_lda_indirect_y() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0xB1, 0x03, 0x00, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x01;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0x01);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_lda_indirect_y_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.y_index_register = 0x06;
//     nes.bus.cpu_memory[0..6].copy_from_slice(&[0xB1, 0x03, 0x00, 0xFF, 0x07, 0x01]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.cpu.a_register, 0x01);
// }
