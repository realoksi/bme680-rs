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

bitfield! {
    #[derive(Debug, From, Into)]
    pub struct CtrlHum(u8);

    pub get_spi_3w_int_en, set_spi_3w_int_en: 6, 6;
    pub get_osrs_h, set_osrs_h: 2, 0;
}

bitfield! {
    #[derive(Debug, From, Into)]
    pub struct CtrlGas1(u8);

    pub get_run_gas, set_run_gas: 5, 5;
    pub get_nb_conv, set_nb_conv: 3, 0;
}

bitfield! {
    #[derive(Debug, From, Into)]
    pub struct CtrlGas0(u8);

    pub get_heat_off, set_heat_off: 3, 3;
}

bitfield! {
    #[derive(Debug, From, Into)]
    pub struct MeasStatus2(u8);

    pub get_new_data, _: 7, 7;
    pub get_gas_measuring, _: 6, 6;
    pub get_measuring, _: 5, 5;
    pub get_gas_meas_index_2, _: 3, 0;
}

bitfield! {
    #[derive(Debug, From, Into)]
    pub struct MeasStatus1(u8);

    pub get_new_data, _: 7, 7;
    pub get_gas_measuring, _: 6, 6;
    pub get_measuring, _: 5, 5;
    pub get_gas_meas_index_1, _: 3, 0;
}

bitfield! {
    #[derive(Debug, From, Into)]
    pub struct MeasStatus0(u8);

    pub get_new_data, _: 7, 7;
    pub get_gas_measuring, _: 6, 6;
    pub get_measuring, _: 5, 5;
    pub get_gas_meas_index_0, _: 3 , 0 ;
}

bitfield! {
    #[derive(Debug, From, Into)]
    pub struct GasRLsb0(u8);

    pub get_gas_r, _: 7, 6;
    pub get_gas_valid_r, _: 5, 5;
    pub get_heat_stab_r, _: 4, 4;
    pub get_gas_range_r, _: 3, 0;
}
