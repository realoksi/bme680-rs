use bitfield::bitfield;
use derive_more::{From, Into};

bitfield! {
    #[derive(Debug, From, Into)]
    pub struct Status(u8);

    pub get_spi_mem_page, set_spi_mem_page: 4, 4;
}

bitfield! {
    #[derive(Debug, From, Into)]
    pub struct Config(u8);

    pub get_filter, set_filter: 4, 2;
    pub get_spi_3w_en, set_spi_3w_en: 0, 0;
}

bitfield! {
    #[derive(Debug, From, Into)]
    pub struct CtrlMeas(u8);

    pub get_osrs_t, set_osrs_t: 7, 5;
    pub get_osrs_p, set_osrs_p: 4, 2;
    pub get_mode, set_mode: 1, 0;
}
