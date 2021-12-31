use std::ops::Range;
use std::ops::ControlFlow;

pub trait Peripheral {
    fn read(&self, _addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, _addr: u16, _byte: u8) {
        todo!()
    }
}

pub struct Bus {
    peripherals: Vec<(Range<usize>, Box<dyn Peripheral>)>,
}

impl Bus {
    pub fn new() -> Self {
        Self { peripherals: vec![] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match self.get_peripheral_index(addr) {
            Some((addr, index)) => self.peripherals[index].1.read(addr),
            _ => 0
        }
    }

    pub fn write(&mut self, addr: u16, byte: u8) {
        if let Some((addr, index)) = self.get_peripheral_index(addr) {
            self.peripherals[index].1.write(addr, byte);
        }
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        match self.get_peripheral_index(addr) {
            Some((addr, index)) => (
                self.peripherals[index].1.read(addr + 1) as u16) << 8 |
                self.peripherals[index].1.read(addr) as u16,
            _ => 0
        }
    }

    pub fn write_u16(&mut self, addr: u16, data: u16) {
        if let Some((addr, index)) = self.get_peripheral_index(addr) {
            self.peripherals[index].1.write(addr + 0, (data & 0xff) as u8); // low byte
            self.peripherals[index].1.write(addr + 1, (data >>   8) as u8); // high byte
        }
    }

    pub fn get_peripheral_index(&self, addr: u16) -> Option<(u16, usize)> {
        self.peripherals.iter().enumerate().try_for_each(|(i, (range, peripheral))| {
            if range.contains(&(addr as usize)) {
                return ControlFlow::Break((addr - range.start as u16, i));
            }

            ControlFlow::Continue(())
        }).break_value()
    }

    pub fn attach<T>(&mut self, lo: u16, hi: u16, peripheral: T) -> Result<(), String>
    where
        T: Peripheral + 'static,
    {
        let lo = lo as usize;
        let hi = hi as usize;

        self.peripherals.iter().try_for_each(|(range, _)| {
            if lo < range.end && hi > range.start {
                return Err(format!(
                    "overlapping ranges: [{:x}:{:x}] and [{:x}:{:x}]",
                    lo, hi,
                    range.start, range.end - 1
                ));
            }

            Ok(())
        })?;

        Ok({ self.peripherals.push((lo..hi + 1, Box::new(peripheral))); })
    }
}