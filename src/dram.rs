pub struct Dram {
    pub ram: Vec<u8>
}

const DRAM_SIZE: u64 = 8 << 20; // 8MiB

impl Dram {
    pub fn new(code: &Vec<u8>) -> Self {
        let mut ram = vec![0; DRAM_SIZE as usize];
        ram.splice(..code.len(), code.iter().cloned());

        Self { ram }
    }

    pub fn load(&self, addr: u64, size: u8) -> u64 {
        match size {
            1 => self.load_8 (addr) as u64,
            2 => self.load_16(addr) as u64,
            3 => self.load_32(addr) as u64,
            4 => self.load_64(addr),
            _  => 0
        }
    }

    pub fn store(&mut self, addr: u64, data: u64, size: u8) {
        match size {
            1 => self.store_8 (addr, data as u8 ),
            2 => self.store_16(addr, data as u16),
            3 => self.store_32(addr, data as u32),
            4 => self.store_64(addr, data),
            _  => ()
        }
    }

    fn load_8(&self, addr: u64) -> u8 {
        self.ram[addr as usize]
    }

    fn store_8(&mut self, addr: u64, data: u8) {
        self.ram[addr as usize] = data
    }

    fn load_16(&self, addr: u64) -> u16 {
        let off = addr as usize;
           (self.ram[off  ] as u16)
        | ((self.ram[off+1] as u16) << 8)
    }

    fn store_16(&mut self, addr: u64, data: u16) {
        self.ram[addr as usize    ] =  data       as u8;
        self.ram[addr as usize + 1] = (data >> 8) as u8;
    }

    fn load_32(&self, addr: u64) -> u32 {
           (self.load_16(addr  ) as u32)
        | ((self.load_16(addr+2) as u32) << 16)
    }

    fn store_32(&mut self, addr: u64, data: u32) {
        self.store_16(addr  ,  data        as u16);
        self.store_16(addr+2, (data >> 16) as u16);
    }

    fn load_64(&self, addr: u64) -> u64 {
           (self.load_32(addr  ) as u64)
        | ((self.load_32(addr+4) as u64) << 32)
    }

    fn store_64(&mut self, addr: u64, data: u64) {
        self.store_32(addr,  data        as u32);
        self.store_32(addr, (data >> 16) as u32);
    }
}
