#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![doc = include_str!("../README.md")]

mod macros;
mod memory_map;

pub type Result<T> = core::result::Result<T, ()>;

pub trait Layer {
    fn read_byte(&mut self, addr: u8) -> Result<u8>;
    fn read_block(&mut self, addr: u8, data: &mut [u8]) -> Result<()>;
    fn write_byte(&mut self, addr: u8, data: u8) -> Result<()>;
}

#[cfg(feature = "embedded-hal")]
impl<T> Layer for T
where
    T: embedded_hal::i2c::I2c,
{
    fn read_byte(&mut self, addr: u8) -> Result<u8> {
        let mut data: [u8; 1] = [0];
        self.write_read(0x76, &[addr], &mut data).map_err(|_| ())?;
        Ok(data[0])
    }
    fn read_block(&mut self, addr: u8, data: &mut [u8]) -> Result<()> {
        self.write_read(0x76, &[addr], data).map_err(|_| ())
    }
    fn write_byte(&mut self, addr: u8, data: u8) -> Result<()> {
        self.write(0x76, &[addr, data]).map_err(|_| ())
    }
}

/// iir filter coefficient. useful for suppressing environmental disturbances
///
/// see [BME680::set_filter]
pub enum Filter {
    /// no filtering
    X0 = 0b000,
    /// min filtering
    X1 = 0b001,
    X3 = 0b010,
    X7 = 0b011,
    X15 = 0b100,
    X31 = 0b101,
    X63 = 0b110,
    /// max filtering
    X127 = 0b111,
}

impl TryFrom<u8> for Filter {
    type Error = ();

    fn try_from(value: u8) -> Result<Filter> {
        Ok(match value {
            0b000 => Filter::X0,
            0b001 => Filter::X1,
            0b010 => Filter::X3,
            0b011 => Filter::X7,
            0b100 => Filter::X15,
            0b101 => Filter::X31,
            0b110 => Filter::X63,
            0b111 => Filter::X127,
            _ => return Err(()),
        })
    }
}

/// controls the bit resolution of a measurement. higher oversampling takes longer
///
/// see [BME680::set_osrs_t], [BME680::set_osrs_p], and [BME680::set_osrs_h]
pub enum Oversampling {
    /// skips the measurement
    X0 = 0b000,
    /// min oversampling
    X1 = 0b001,
    X2 = 0b010,
    X4 = 0b011,
    X8 = 0b100,
    /// max oversampling
    X16 = 0b101,
}

impl TryFrom<u8> for Oversampling {
    type Error = ();

    fn try_from(value: u8) -> Result<Oversampling> {
        Ok(match value {
            0b000 => Oversampling::X0,
            0b001 => Oversampling::X1,
            0b010 => Oversampling::X2,
            0b011 => Oversampling::X4,
            0b100 => Oversampling::X8,
            0b101 => Oversampling::X16,
            _ => return Err(()),
        })
    }
}

/// sensor power mode
pub enum Mode {
    /// idle state
    Sleep = 0b00,
    /// single measurements. goes back to sleep
    Forced = 0b01,
    /// multiple measurements. stays awake
    Parallel = 0b10,
}

impl TryFrom<u8> for Mode {
    type Error = ();

    fn try_from(value: u8) -> Result<Mode> {
        Ok(match value {
            0b00 => Mode::Sleep,
            0b01 => Mode::Forced,
            0b10 => Mode::Parallel,
            _ => return Err(()),
        })
    }
}

pub struct BME680<L>
where
    L: Layer,
{
    transport_layer: L,
    chip_id: Option<u8>,
    variant_id: Option<u8>,
    par_t1: Option<u16>,
    par_t2: Option<i16>,
    par_t3: Option<i8>,
    par_p1: Option<u16>,
    par_p2: Option<i16>,
    par_p3: Option<i8>,
    par_p4: Option<i16>,
    par_p5: Option<i16>,
    par_p6: Option<i8>,
    par_p7: Option<i8>,
    par_p8: Option<i16>,
    par_p9: Option<i16>,
    par_p10: Option<u8>,
    par_h1: Option<u16>,
    par_h2: Option<u16>,
    par_h3: Option<i8>,
    par_h4: Option<i8>,
    par_h5: Option<i8>,
    par_h6: Option<u8>,
    par_h7: Option<u8>,
    par_g1: Option<i8>,
    par_g2: Option<i16>,
    par_g3: Option<i8>,
}

impl<L> BME680<L>
where
    L: Layer,
{
    pub fn new(transport_layer: L) -> Self {
        Self {
            transport_layer,
            chip_id: None,
            variant_id: None,
            par_t1: None,
            par_t2: None,
            par_t3: None,
            par_p1: None,
            par_p2: None,
            par_p3: None,
            par_p4: None,
            par_p5: None,
            par_p6: None,
            par_p7: None,
            par_p8: None,
            par_p9: None,
            par_p10: None,
            par_h1: None,
            par_h2: None,
            par_h3: None,
            par_h4: None,
            par_h5: None,
            par_h6: None,
            par_h7: None,
            par_g1: None,
            par_g2: None,
            par_g3: None,
        }
    }

    /// Consumes `self` and returns the underlying transport layer.
    pub fn into_inner(self) -> L {
        self.transport_layer
    }

    once_ro!(
        /// returns the chip id
        chip_id
    );
    once_ro!(
        /// returns the variant id
        variant_id
    );
    once_ro!(
        /// returns first temp calibration parameter
        par_t1,
        u16,
        [par_t1_msb, par_t1_lsb]
    );
    once_ro!(
        /// returns second temp calibration parameter
        par_t2,
        i16,
        [par_t2_msb, par_t2_lsb]
    );
    once_ro!(
        /// returns third temp calibration parameter
        par_t3,
        i8
    );
    once_ro!(
        /// returns first pressure calibration parameter
        par_p1,
        u16,
        [par_p1_msb, par_p1_lsb]
    );
    once_ro!(
        /// returns second pressure calibration parameter
        par_p2,
        i16,
        [par_p2_msb, par_p2_lsb]
    );
    once_ro!(
        /// returns third pressure calibration parameter
        par_p3,
        i8
    );
    once_ro!(
        /// returns fourth pressure calibration parameter
        par_p4,
        i16,
        [par_p4_msb, par_p4_lsb]
    );
    once_ro!(
        /// returns fifth pressure calibration parameter
        par_p5,
        i16,
        [par_p5_msb, par_p5_lsb]
    );
    once_ro!(
        /// returns sixth pressure calibration parameter
        par_p6,
        i8
    );
    once_ro!(
        /// returns seventh pressure calibration parameter
        par_p7,
        i8
    );
    once_ro!(
        /// returns eighth pressure calibration parameter
        par_p8,
        i16,
        [par_p8_msb, par_p8_lsb]
    );
    once_ro!(
        /// returns ninth pressure calibration parameter
        par_p9,
        i16,
        [par_p9_msb, par_p9_lsb]
    );
    once_ro!(
        /// returns tenth pressure calibration parameter
        par_p10
    );
    once_ro!(
        /// returns first humidity calibration parameter
        par_h1,
        u16,
        [par_h1_msb, par_h1_lsb],
        |msb, lsb| (msb as u16) << 4 | (lsb as u16) & 0b0000_1111);
    once_ro!(
        /// returns second humidity calibration parameter
        par_h2,
        u16,
        [par_h2_msb, par_h2_lsb],
        |msb, lsb| (msb as u16) << 4 | (lsb as u16) >> 4
    );
    once_ro!(
        /// returns third humidity calibration parameter
        par_h3,
        i8
    );
    once_ro!(
        /// returns fourth humidity calibration parameter
        par_h4,
        i8
    );
    once_ro!(
        /// returns fifth humidity calibration parameter
        par_h5,
        i8
    );
    once_ro!(
        /// returns sixth humidity calibration parameter
        par_h6
    );
    once_ro!(
        /// returns seventh humidity calibration parameter
        par_h7
    );
    once_ro!(
        /// returns first gas calibration parameter
        par_g1,
        i8
    );
    once_ro!(
        /// returns second gas calibration parameter
        par_g2,
        i16,
        [par_g2_msb, par_g2_lsb]
    );
    once_ro!(
        /// returns third gas calibration parameter
        par_g3,
        i8
    );

    live_ro!(gas_r_2, [gas_r_msb_2, gas_r_lsb_2], |msb, lsb| (msb as u16)
        << 2
        | (lsb as u16) >> 6);

    live_ro!(hum_adc_0, [hum_msb_0, hum_lsb_0]);

    live_ro!(
        temp_adc_2,
        [temp_msb_2, temp_lsb_2, temp_xlsb_2],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(
        press_adc_2,
        [press_msb_2, press_lsb_2, press_xlsb_2],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(gas_r_1, [gas_r_msb_1, gas_r_lsb_1], |msb, lsb| (msb as u16)
        << 2
        | (lsb as u16) >> 6);

    live_ro!(hum_adc_1, [hum_msb_1, hum_lsb_1]);

    live_ro!(
        temp_adc_1,
        [temp_msb_1, temp_lsb_1, temp_xlsb_1],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(hum_adc_2, [hum_msb_2, hum_lsb_2]);

    live_ro!(
        press_adc_1,
        [press_msb_1, press_lsb_1, press_xlsb_1],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(gas_r_0, [gas_r_msb_0, gas_r_lsb_0], |msb, lsb| (msb as u16)
        << 2
        | (lsb as u16) >> 6);

    live_ro!(
        temp_adc_0,
        [temp_msb_0, temp_lsb_0, temp_xlsb_0],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(
        press_adc_0,
        [press_msb_0, press_lsb_0, press_xlsb_0],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    reg_rw!(status, u8);

    pub fn get_spi_mem_page(&mut self) -> Result<bool> {
        Ok(((self.read_status()? & 0b0001_0000) >> 4) != 0)
    }

    pub fn set_spi_mem_page(&mut self, value: bool) -> Result<()> {
        let status = (self.read_status()? & !0b0001_0000) | ((value as u8) << 4);

        self.write_status(status)
    }

    reg_rw!(reset, u8);

    pub fn soft_reset(&mut self) -> Result<()> {
        self.write_reset(0xB6)
    }

    reg_rw!(config, u8);

    pub fn get_filter(&mut self) -> Result<Filter> {
        Filter::try_from((self.read_config()? & 0b0001_1100) >> 2)
    }

    pub fn set_filter(&mut self, value: Filter) -> Result<()> {
        let config = (self.read_config()? & !0b0001_1100) | ((value as u8) << 2);

        self.write_config(config)
    }

    pub fn get_spi_3w_en(&mut self) -> Result<bool> {
        Ok((self.read_config()? & 0b1) != 0)
    }

    pub fn set_spi_3w_en(&mut self, value: bool) -> Result<()> {
        let config = (self.read_config()? & !0b1) | (value as u8);

        self.write_config(config)
    }

    reg_rw!(ctrl_meas, u8);

    pub fn get_ctrl_meas(&mut self) -> Result<()> {
        todo!()
    }

    pub fn get_osrs_t(&mut self) -> Result<Oversampling> {
        Oversampling::try_from((self.read_ctrl_meas()? & 0b1110_0000) >> 5)
    }

    pub fn set_osrs_t(&mut self, value: Oversampling) -> Result<()> {
        let ctrl_meas = (self.read_ctrl_meas()? & !0b1110_0000) | ((value as u8) << 5);

        self.write_ctrl_meas(ctrl_meas)
    }

    pub fn get_osrs_p(&mut self) -> Result<Oversampling> {
        Oversampling::try_from((self.read_ctrl_meas()? & 0b0001_1100) >> 2)
    }

    pub fn set_osrs_p(&mut self, value: Oversampling) -> Result<()> {
        let ctrl_meas = (self.read_ctrl_meas()? & !0b0001_1100) | ((value as u8) << 2);

        self.write_ctrl_meas(ctrl_meas)
    }

    pub fn get_mode(&mut self) -> Result<Mode> {
        Mode::try_from(self.read_ctrl_meas()? & 0b11)
    }

    pub fn set_mode(&mut self, value: Mode) -> Result<()> {
        let ctrl_meas = (self.read_ctrl_meas()? & !0b11) | (value as u8);

        self.write_ctrl_meas(ctrl_meas)
    }

    reg_rw!(ctrl_hum, u8);

    pub fn get_osrs_h(&mut self) -> Result<Oversampling> {
        Oversampling::try_from(self.read_ctrl_hum()? & 0b0000_0111)
    }

    pub fn set_osrs_h(&mut self, value: Oversampling) -> Result<()> {
        let ctrl_hum = (self.read_ctrl_hum()? & !0b0000_0111) | (value as u8);

        self.write_ctrl_hum(ctrl_hum)
    }

    reg_rw!(ctrl_gas_1, u8);
    reg_rw!(ctrl_gas_0, u8);
    reg_rw!(gas_wait_shared, u8);
    reg_ro!(sub_meas_index_0, u8);
    reg_ro!(meas_status_0, u8);
    reg_ro!(sub_meas_index_2, u8);
    reg_ro!(meas_status_2, u8);
    reg_ro!(sub_meas_index_1, u8);
    reg_ro!(meas_status_1, u8);
    reg_ro!(res_heat_range, u8);
    reg_ro!(res_heat_val, u8);
    reg_ro!(gas_range_0, u8);
    reg_ro!(gas_range_1, u8);
    reg_ro!(gas_range_2, u8);

    fn get_tfine(&mut self, temp_adc: i32) -> Result<i32> {
        let par_t1 = self.get_par_t1()? as i32;
        let par_t2 = self.get_par_t2()? as i32;
        let par_t3 = self.get_par_t3()? as i32;

        let var1 = (temp_adc >> 3) - (par_t1 << 1);
        let var2 = (var1 * par_t2) >> 11;
        let var3 = ((((var1 >> 1) * (var1 >> 1)) >> 12) * (par_t3 << 4)) >> 14;

        Ok(var2 + var3)
    }

    fn get_t_fine_0(&mut self) -> Result<i32> {
        let temp_adc_0 = self.get_temp_adc_0()? as i32;

        Ok(self.get_tfine(temp_adc_0)?)
    }

    pub fn get_temp_comp_0(&mut self) -> Result<i32> {
        Ok(((self.get_t_fine_0()? * 5) + 128) >> 8)
    }

    fn get_t_fine_1(&mut self) -> Result<i32> {
        let temp_adc_1 = self.get_temp_adc_1()? as i32;

        Ok(self.get_tfine(temp_adc_1)?)
    }

    pub fn get_temp_comp_1(&mut self) -> Result<i32> {
        Ok(((self.get_t_fine_1()? * 5) + 128) >> 8)
    }

    fn get_t_fine_2(&mut self) -> Result<i32> {
        let temp_adc_2 = self.get_temp_adc_2()? as i32;

        Ok(self.get_tfine(temp_adc_2)?)
    }

    pub fn get_temp_comp_2(&mut self) -> Result<i32> {
        Ok(((self.get_t_fine_2()? * 5) + 128) >> 8)
    }

    pub fn get_press_comp_0(&mut self) -> Result<i32> {
        let mut var1 = (self.get_t_fine_0()? >> 1) - 64000;
        let mut var2 = ((((var1 >> 2) * (var1 >> 2)) >> 11) * (self.get_par_p6()? as i32)) >> 2;

        var2 = var2 + ((var1 * (self.get_par_p5()? as i32)) << 1);
        var2 = (var2 >> 2) + ((self.get_par_p4()? as i32) << 16);
        var1 = (((((var1 >> 2) * (var1 >> 2)) >> 13) * ((self.get_par_p3()? as i32) << 5)) >> 3)
            + (((self.get_par_p2()? as i32) * var1) >> 1);
        var1 = var1 >> 18;
        var1 = ((32768 + var1) * (self.get_par_p1()? as i32)) >> 15;

        let press_comp = 1048576 - self.get_press_adc_0()? as i32;
        let mut press_comp = ((press_comp - (var2 >> 12)) * 3125) as u32;

        if press_comp >= (1 << 30) {
            press_comp = (press_comp / (var1 as u32)) << 1;
        } else {
            press_comp = (press_comp << 1) / var1 as u32;
        }

        var1 = ((self.get_par_p9()? as i32)
            * ((((press_comp >> 3) * (press_comp >> 3)) >> 13) as i32))
            >> 12;
        var2 = (((press_comp >> 2) as i32) * (self.get_par_p8()? as i32)) >> 13;

        let var3 = (((press_comp >> 8) as i32)
            * ((press_comp >> 8) as i32)
            * ((press_comp >> 8) as i32)
            * (self.get_par_p10()? as i32))
            >> 17;
        let press_comp =
            (press_comp as i32) + ((var1 + var2 + var3 + ((self.get_par_p7()? as i32) << 7)) >> 4);

        Ok(press_comp)
    }

    /// A diagnostic operation that performs a single 256-byte burst read over the whole range of
    /// addresses.
    ///
    /// Useful for taking a snapshot of the chips current state.
    pub fn diag_dump(&mut self) -> Result<[u8; 256]> {
        let mut res = [0; 256];
        self.transport_layer.read_block(0, &mut res)?;
        Ok(res)
    }
}
