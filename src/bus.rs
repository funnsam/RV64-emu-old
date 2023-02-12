use crate::dram::*;

pub const DRAM_OFFSET: u64 = 0x000_0000;

pub struct Bus {
    pub dram: Dram
}

impl Bus {
    pub fn load(&self, addr: u64, size: u8) -> Option<u64> {
        if DRAM_OFFSET <= addr {
            Some(self.dram.load(addr - DRAM_OFFSET, size))
        } else {
            None
        }
    }

    pub fn store(&mut self, addr: u64, data: u64, size: u8) -> Option<()> {
        if DRAM_OFFSET <= addr {
            self.dram.store(addr - DRAM_OFFSET, data, size);
            Some(())
        } else {
            None
        }
    }
}
