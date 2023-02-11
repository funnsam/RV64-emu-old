#[derive(Debug)]
pub struct Cpu {
    pub rx: [u64; 32],
    pub pc: u64,
    pub mem: Vec<u8>
}

impl Cpu {
    pub fn new(mem: Vec<u8>) -> Self {
        let mut rx = [0u64; 32];
        rx[2] = mem.len() as u64;
        Self {
            rx,
            pc: 0,
            mem
        }
    }

    pub fn step(&mut self) -> Option<()> {
        if self.pc & 3 != 0 {
            // return None
        }
        let inst = self.fetch();
        println!("{inst:08X} {}", Self::dec_i(inst));
        self.pc += 4;
        let opcode = (inst & 0x7f) as u8;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;

        match opcode {
            INST_ADDI   => self.rx[rd] = self.rx[rs1] + Self::dec_i(inst) as u64,
            INST_ADD    => self.rx[rd] = self.rx[rs1] + self.rx[rs2],
            _ => todo!("{opcode:02X}")
        }
        Some(())
    }

    fn fetch(&mut self) -> u32 {
        let index = self.pc as usize;
           (self.mem[index    ] as u32)
        | ((self.mem[index + 1] as u32) <<  8)
        | ((self.mem[index + 2] as u32) << 16)
        | ((self.mem[index + 3] as u32) << 24)
    }

    fn dec_i(inst: u32) -> u16 {
        ((inst & 0xfff00000) as i32 as i64 >> 20) as u16
    }

    pub fn show_reg(&self) {
        for (i, j) in self.rx.iter().enumerate() {
            println!("R{i}: 0x{j:X}");
        }
        println!("")
    }
}

const INST_ADD  : u8 = 0x33;
const INST_ADDI : u8 = 0x13;
