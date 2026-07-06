use bme680_rs::Layer;

// MEMORY_MAP is a dump from a real BME688 device after a fresh startup.
const MEMORY_MAP: [u8; 256] = [
    39, 170, 22, 204, 243, 7, 77, 38, 0, 0, 1, 0, 14, 0, 2, 4, 16, 0, 64, 0, 128, 0, 30, 0, 31,
    127, 31, 16, 0, 0, 0, 128, 0, 0, 128, 0, 0, 128, 0, 128, 0, 0, 0, 4, 0, 4, 0, 0, 128, 0, 0,
    128, 0, 0, 128, 0, 128, 0, 0, 0, 4, 0, 4, 0, 0, 128, 0, 0, 128, 0, 0, 128, 0, 128, 0, 0, 0, 4,
    0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 4, 254, 22, 155, 8, 0, 255, 135, 78, 137, 69, 139, 84, 19, 6,
    177, 64, 55, 101, 3, 16, 164, 141, 41, 215, 88, 0, 1, 30, 205, 255, 36, 30, 0, 0, 72, 248, 201,
    244, 30, 131, 137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 0, 192, 0, 84, 0, 0, 0, 0, 96, 2, 0, 1, 1,
    129, 31, 96, 3, 0, 0, 0, 0, 255, 15, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 97, 1, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 4, 16, 64, 0, 0, 65, 180, 34, 0, 45, 20, 120, 156, 14, 103, 63, 228, 40, 18, 129,
    1, 0, 0, 0, 0, 0, 0, 128, 0, 0, 128, 0, 0, 128, 0, 0,
];

// Not an emulator. Allows reading and writing to an instance of the above snapshot.
pub struct I2c {
    memory_map: [u8; 256],
}

impl I2c {
    pub fn new() -> Self {
        Self {
            memory_map: MEMORY_MAP,
        }
    }
}

impl Layer for I2c {
    fn read_byte(&mut self, addr: u8) -> bme680_rs::Result<u8> {
        Ok(self.memory_map[addr as usize])
    }
    fn write_byte(&mut self, addr: u8, byte: u8) -> bme680_rs::Result<()> {
        self.memory_map[addr as usize] = byte;

        Ok(())
    }
}
