pub trait Peripheral {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, byte: u8);
}

struct PeripheralItem {
    range: (u16, u16),
    peripheral: Box<dyn Peripheral>,
}

pub struct Bus {
    mirror: [u8; 1 << 16],
    peripherals: Vec<PeripheralItem>,
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            mirror: [0; 1 << 16],
            peripherals: Vec::new(),
        }
    }
}

impl PeripheralItem {
    fn new(range: (u16, u16), peripheral: Box<dyn Peripheral>) -> Self {
        Self { range, peripheral }
    }

    fn handles(&self, addr: u16) -> bool {
        self.range.0 <= addr && self.range.1 >= addr
    }

    fn overlaps(&self, range: (u16, u16)) -> bool {
        range.0 < self.range.1 && range.1 > self.range.0
    }
}

impl Bus {
    pub fn read(&self, addr: u16) -> u8 {
        if let Some((i, addr)) = self.get_peripheral_index(addr) {
            self.peripherals[i].peripheral.read(addr)
        } else {
            0
        }
    }

    pub fn write(&mut self, addr: u16, byte: u8) {
        // mirror everything written into memory
        self.mirror[addr as usize] = byte;

        if let Some((i, addr)) = self.get_peripheral_index(addr) {
            self.peripherals[i].peripheral.write(addr, byte);
        }
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        if let Some((i, addr)) = self.get_peripheral_index(addr) {
            let hi = self.peripherals[i].peripheral.read(addr + 1);
            let lo = self.peripherals[i].peripheral.read(addr);

            (hi as u16) << 8 | (lo as u16)
        } else {
            0
        }
    }

    pub fn write_u16(&mut self, addr: u16, data: u16) {
        if let Some((i, addr)) = self.get_peripheral_index(addr) {
            self.peripherals[i]
                .peripheral
                .write(addr, (data & 0xff) as u8); // low byte
            self.peripherals[i]
                .peripheral
                .write(addr + 1, (data >> 8) as u8); // high byte
        }
    }

    pub fn get_peripheral_index(&self, addr: u16) -> Option<(usize, u16)> {
        for (i, peripheral) in self.peripherals.iter().enumerate() {
            if peripheral.handles(addr) {
                return Some((i, addr - peripheral.range.0));
            }
        }

        None
    }

    pub fn attach<P>(&mut self, lo: u16, hi: u16, peripheral: P) -> Result<(), String>
    where
        P: Peripheral + 'static,
    {
        for item in self.peripherals.iter() {
            if item.overlaps((lo, hi)) {
                return Err(format!(
                    "overlapping ranges: [{:x}:{:x}] and [{:x}:{:x}]",
                    lo, hi, item.range.0, item.range.1,
                ));
            }
        }

        self.peripherals
            .push(PeripheralItem::new((lo, hi), Box::new(peripheral)));
        Ok(())
    }

    pub fn get_memory_region(&self, range: (u16, u16)) -> &[u8] {
        let lower = (range.0 as usize).clamp(0, u16::MAX as usize);
        let upper = (lower + range.1 as usize).clamp(0, u16::MAX as usize);

        &self.mirror[lower..=upper]
    }
}
