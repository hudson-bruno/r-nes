use r_nes::cpu::{Cpu, memory::stack::Stack};

#[test]
fn test_pha_implicit() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.memory[0] = 0x48;

    let result = cpu.clock();
    let a_register_from_pha = cpu.stack_pop();

    assert!(result.is_none());
    assert_eq!(a_register_from_pha, 0xFF);
}
