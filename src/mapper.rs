pub mod mapper_000;

pub trait Mapper {
    fn map_cpu_read(&self, addr: u16) -> u16;
    fn map_cpu_write(&self, addr: u16) -> u16;
    fn map_ppu_read(&self, addr: u16) -> u16;
    fn map_ppu_write(&self, addr: u16) -> u16;
}
