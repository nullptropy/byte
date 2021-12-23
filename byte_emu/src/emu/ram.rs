use byte_core::bus::Peripheral;

pub struct Ram {
    data: [u8; 0x10000],
}

impl Default for Ram {
    fn default() -> Self {
        Self { data: [0; 0x10000] }
    }
}

impl Peripheral for Ram {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, byte: u8) {
        self.data[addr as usize] = byte;
    }
}
