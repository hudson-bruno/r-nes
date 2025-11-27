use r_nes::{
    cpu::memory::stack::Stack,
    nes::Nes,
};

#[test]
fn test_jsr() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x20, 0xFF, 0x07]);

    let result = nes.clock();
    let pc_before_jsr = nes.cpu.stack_pop(&mut nes.bus);

    assert!(result.is_none());
    assert_eq!(pc_before_jsr, 0x03);
}
