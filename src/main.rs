mod cpu;
mod dram;
mod bus;
mod arg_parse;

use crate::cpu::Cpu;

fn main() {
    let args = arg_parse::parse(&mut std::env::args());
    let mem = std::fs::read(args.rom.unwrap()).unwrap();
    let mut cpu = Cpu::new(bus::Bus{ dram: dram::Dram::new(&mem) });
    println!("{:X}", cpu.bus.load(0, 3).unwrap());
    loop {
        match cpu.step() {
            Some(_) => (),
            None => break
        }
        println!("{:X}", cpu.bus.load(0xfe, 3).unwrap());
        cpu.show_reg();
    }
}
