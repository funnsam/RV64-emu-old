mod cpu;
use crate::cpu::Cpu;

fn main() {
    let mem = std::fs::read("test.bin").unwrap();
    println!("{:?}", mem);
    let mut cpu = Cpu::new(mem);
    while cpu.pc < cpu.mem.len() as u64 {
        cpu.step().unwrap();
        cpu.show_reg()
    }
}
