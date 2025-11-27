use r_nes::nes::Nes;

#[test]
fn test_sta_zero_page() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x85, 0x03]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x03], 0xFF);
}

#[test]
fn test_sta_zero_page_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x95, 0xFE]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0xFF], 0xFF);
}

#[test]
fn test_sta_zero_page_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x95, 0xFF]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x03], 0xFF);
}

#[test]
fn test_sta_absolute() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x8D, 0xFF, 0x07]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0xFF);
}

#[test]
fn test_sta_absolute_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x9D, 0xFE, 0x07]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0xFF);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_sta_absolute_x_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.a_register = 0xFF;
//     nes.cpu.x_index_register = 0x05;
//     nes.bus.cpu_memory[0..5].copy_from_slice(&[0x9D, 0xFF, 0x07]);
//
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.bus.cpu_memory[0x04], 0xFF);
// }

#[test]
fn test_sta_absolute_y() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x99, 0xFE, 0x07, 0x00]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0xFF);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_sta_absolute_y_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.a_register = 0xFF;
//     nes.cpu.y_index_register = 0x05;
//     nes.bus.cpu_memory[0..3].copy_from_slice(&[0x99, 0xFF, 0x07]);
//
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.bus.cpu_memory[0x04], 0xFF);
// }

#[test]
fn test_sta_indirect_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x81, 0x02, 0x00, 0xFF, 0x07]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0xFF);
}

#[test]
fn test_sta_indirect_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x81, 0xFF, 0x00, 0xFF, 0x07]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0xFF);
}

#[test]
fn test_sta_indirect_y() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x91, 0x03, 0x00, 0xFE, 0x07]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0xFF);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_sta_indirect_y_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.a_register = 0xFF;
//     nes.cpu.y_index_register = 0x06;
//     nes.bus.cpu_memory[0..5].copy_from_slice(&[0x91, 0x03, 0x00, 0xFF, 0x07]);
//
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.bus.cpu_memory[0x05], 0xFF);
// }
