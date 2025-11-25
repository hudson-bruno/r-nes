use r_nes::cpu::Cpu;

#[test]
fn test_sta_zero_page() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.memory[0..2].copy_from_slice(&[0x85, 0x03]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x03], 0xFF);
}

#[test]
fn test_sta_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.x_index_register = 0x01;
    cpu.memory[0..2].copy_from_slice(&[0x95, 0xFE]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0xFF], 0xFF);
}

#[test]
fn test_sta_zero_page_x_overflow() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.x_index_register = 0x04;
    cpu.memory[0..2].copy_from_slice(&[0x95, 0xFF]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x03], 0xFF);
}

#[test]
fn test_sta_absolute() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.memory[0..3].copy_from_slice(&[0x8D, 0xFF, 0x07]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x07FF], 0xFF);
}

#[test]
fn test_sta_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.x_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0x9D, 0xFE, 0x07]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x07FF], 0xFF);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_sta_absolute_x_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.a_register = 0xFF;
//     cpu.x_index_register = 0x05;
//     cpu.memory[0..5].copy_from_slice(&[0x9D, 0xFF, 0x07]);
//
//     let result = cpu.clock();
//
//     assert!(result.is_none());
//     assert_eq!(cpu.memory[0x04], 0xFF);
// }

#[test]
fn test_sta_absolute_y() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.y_index_register = 0x01;
    cpu.memory[0..4].copy_from_slice(&[0x99, 0xFE, 0x07, 0x00]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x07FF], 0xFF);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_sta_absolute_y_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.a_register = 0xFF;
//     cpu.y_index_register = 0x05;
//     cpu.memory[0..3].copy_from_slice(&[0x99, 0xFF, 0x07]);
//
//     let result = cpu.clock();
//
//     assert!(result.is_none());
//     assert_eq!(cpu.memory[0x04], 0xFF);
// }

#[test]
fn test_sta_indirect_x() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.x_index_register = 0x01;
    cpu.memory[0..5].copy_from_slice(&[0x81, 0x02, 0x00, 0xFF, 0x07]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x07FF], 0xFF);
}

#[test]
fn test_sta_indirect_x_overflow() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.x_index_register = 0x04;
    cpu.memory[0..5].copy_from_slice(&[0x81, 0xFF, 0x00, 0xFF, 0x07]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x07FF], 0xFF);
}

#[test]
fn test_sta_indirect_y() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.y_index_register = 0x01;
    cpu.memory[0..5].copy_from_slice(&[0x91, 0x03, 0x00, 0xFE, 0x07]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x07FF], 0xFF);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_sta_indirect_y_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.a_register = 0xFF;
//     cpu.y_index_register = 0x06;
//     cpu.memory[0..5].copy_from_slice(&[0x91, 0x03, 0x00, 0xFF, 0x07]);
//
//     let result = cpu.clock();
//
//     assert!(result.is_none());
//     assert_eq!(cpu.memory[0x05], 0xFF);
// }
