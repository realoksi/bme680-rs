use bitfield::bitfield;

bitfield! {
    pub struct Status(u8);

    pub get_spi_mem_page, set_spi_mem_page: 4, 4;
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        Status(value)
    }
}

impl Into<u8> for Status {
    fn into(self) -> u8 {
        self.0
    }
}

bitfield! {
    pub struct Config(u8);

    pub get_filter, set_filter: 4, 2;
    pub get_spi_3w_en, set_spi_3w_en: 0, 0;
}

impl From<u8> for Config {
    fn from(value: u8) -> Self {
        Config(value)
    }
}

impl Into<u8> for Config {
    fn into(self) -> u8 {
        self.0
    }
}

