#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod memory_map;

pub type Result<T> = core::result::Result<T, ()>;

pub trait Layer {
    fn read_byte(&mut self, addr: u8) -> Result<u8>;
    fn write_byte(&mut self, addr: u8, data: u8) -> Result<()>;
}

pub struct BME680<L>
where
    L: Layer,
{
    transport_layer: L,
}

impl<L> BME680<L>
where
    L: Layer,
{
    pub fn new(transport_layer: L) -> Self {
        Self {
            transport_layer
        }
    }

    pub fn get_chip_id(&mut self) -> u8 {
        self.transport_layer.read_byte(memory_map::chip_id::get_addr()).unwrap()
    }
}