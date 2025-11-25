use r_nes::cpu::{Cpu, memory::stack::Stack};

#[test]
fn test_jsr() {
    let mut cpu = Cpu::new();
    cpu.memory[0..3].copy_from_slice(&[0x20, 0xFF, 0x07]);

    let result = cpu.clock();
    let pc_before_jsr = cpu.stack_pop();

    assert!(result.is_none());
    assert_eq!(pc_before_jsr, 0x03);
}
