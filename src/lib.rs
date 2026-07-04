#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod memory_map;

pub type Result<T> = core::result::Result<T, ()>;

pub trait Layer {
    fn read_byte(&mut self, addr: u8) -> Result<u8>;
    fn read_block(&mut self, addr: u8, data: &mut [u8]) -> Result<()> {
        for (i, k) in data.iter_mut().enumerate() {
            *k = self.read_byte(addr + (i as u8))?;
        }
        Ok(())
    }
    fn write_byte(&mut self, addr: u8, data: u8) -> Result<()>;
}

pub struct BME680<L>
where
    L: Layer,
{
    transport_layer: L,
}

/// A helper macro for accessing simple registers from `memory_map`.
macro_rules! reg {
    ($name:ident, $result:ident) => {
        paste::paste! {
            pub(crate) fn[<get_ $name>](&mut self) -> Result<$result> {
                memory_map::$name::read(&mut self.transport_layer)
            }
        }
    };
}

impl<L> BME680<L>
where
    L: Layer,
{
    pub fn new(transport_layer: L) -> Self {
        Self { transport_layer }
    }

    reg!(chip_id, u8);
}
