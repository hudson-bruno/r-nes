use r_nes::cpu::{Cpu, memory::stack::Stack};

#[test]
fn test_rts() {
    let mut cpu = Cpu::new();
    cpu.memory[0] = 0x60;
    cpu.stack_push_address(0x07FF);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.program_counter, 0x07FF + 1);
}
