use std::ops::Range;

pub struct Bus {
    peripherals: Vec<(Range<usize>, Box<dyn Peripheral>)>,
}

pub trait Peripheral {
    fn read(&self, _addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, _addr: u16, _byte: u8) {
        todo!()
    }
}
