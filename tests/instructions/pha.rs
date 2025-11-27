use r_nes::{
    cpu::memory::stack::Stack,
    nes::Nes,
};

#[test]
fn test_pha_implicit() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.bus.cpu_memory[0] = 0x48;

    let result = nes.clock();
    let a_register_from_pha = nes.cpu.stack_pop(&mut nes.bus);

    assert!(result.is_none());
    assert_eq!(a_register_from_pha, 0xFF);
}
