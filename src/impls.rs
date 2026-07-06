use crate::Layer;

#[cfg(feature = "std")]
use std::fs::File;

#[cfg(feature = "i2c-linux")]
impl<I> Layer for i2c_linux::I2c<I> {
    fn read_byte(&mut self, addr: u8) -> crate::Result<u8> {
        todo!()
    }
    fn write_byte(&mut self, addr: u8, data: u8) -> crate::Result<()> {
        todo!()
    }
}
