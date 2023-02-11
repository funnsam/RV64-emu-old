mod cpu;
mod arg_parse;

use crate::cpu::Cpu;

fn main() {
    let args = arg_parse::parse(&mut std::env::args());
    let mem = std::fs::read(args.rom.unwrap()).unwrap();
    println!("{:?}", mem);
    let mut cpu = Cpu::new(mem);
    while cpu.pc < cpu.mem.len() as u64 {
        cpu.step().unwrap();
        cpu.show_reg()
    }
}
