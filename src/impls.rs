use crate::Layer;

#[cfg(feature = "i2c-linux")]
impl Layer for i2c_linux::I2c<std::fs::File> {
    fn read_byte(&mut self, addr: u8) -> crate::Result<u8> {
        self.smbus_read_byte_data(addr).map_err(|_| ())
    }
    fn write_byte(&mut self, addr: u8, data: u8) -> crate::Result<()> {
        self.smbus_write_byte_data(addr, data).map_err(|_| ())
    }
}
