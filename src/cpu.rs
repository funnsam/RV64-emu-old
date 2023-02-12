use crate::bus::*;

pub struct Cpu {
    pub rx: [u64; 32],
    pub pc: u64,
    pub bus: Bus
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        let mut rx = [0u64; 32];
        // rx[2] = mem.len() as u64;
        Self {
            rx,
            pc: DRAM_OFFSET,
            bus
        }
    }

    pub fn step(&mut self) -> Option<()> {
        if self.pc & 3 != 0 {
            return None
        }
        let inst = self.bus.load(self.pc, 3);
        let inst = match inst {
            Some(v) => v as u32,
            None => return None
        };
        self.pc += 4;
        let opcode = (inst & 0x7f) as u8;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;
        let funct3 = (inst >> 12) & 0x7;

        println!("{:X}", Self::dec_i(inst));

        match opcode {
            INST_ADDI   => self.rx[rd] = self.rx[rs1] + Self::dec_i(inst) as u64,
            INST_ADD    => self.rx[rd] = self.rx[rs1] + self.rx[rs2],
            INST_L      => {
                let a = self.bus.load(self.rx[rs1] + Self::dec_i(inst) as u64, (funct3+1) as u8 % 4)?;
                self.rx[rd] = match funct3 {
                    0 => a as i8  as i64 as u64,
                    1 => a as i16 as i64 as u64,
                    2 => a as i32 as i64 as u64,
                    3 => a as i64 as u64,
                    _ => a
                }
            },
            INST_S      => self.bus.store(Self::dec_i(inst) as u64+self.rx[rs1], self.rx[rs2], funct3 as u8 +1).unwrap(),
            _ => return None
        }
        Some(())
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
const INST_L    : u8 = 0x03;
const INST_S    : u8 = 0x23;
