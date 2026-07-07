use crate::Layer;

// TODO: Slave address 0x76 shouldn't be hardcoded.
#[cfg(feature = "embedded-hal")]
impl<T> Layer for T where T: embedded_hal::i2c::I2c {
    fn read_byte(&mut self, addr: u8) -> crate::Result<u8> {
        let mut data: [u8; 1] = [0];
        self.write_read(0x76, &[addr], &mut data).map_err(|_| ())?;
        Ok(data[0])
    }
    fn read_block(&mut self, addr: u8, data: &mut [u8]) -> crate::Result<()> {
        self.write_read(0x76, &[addr], data).map_err(|_| ())
    }
    fn write_byte(&mut self, addr: u8, data: u8) -> crate::Result<()> {
        self.write(0x76, &[addr, data]).map_err(|_| ())
    }
}
