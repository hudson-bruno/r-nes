use r_nes::{
    cpu::memory::stack::Stack,
    nes::Nes,
};

#[test]
fn test_rts() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0] = 0x60;
    nes.cpu.stack_push_address(&mut nes.bus, 0x07FF);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.program_counter, 0x07FF + 1);
}
