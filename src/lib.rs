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

pub enum Filter {
    X0 = 0b000,
    X1 = 0b001,
    X3 = 0b010,
    X7 = 0b011,
    X15 = 0b100,
    X31 = 0b101,
    X63 = 0b110,
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
            0b111 => Filter::X7,
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
    par_t2: Option<u16>,
    par_t3: Option<u8>,
    par_p1: Option<u16>,
    par_p2: Option<u16>,
    par_p3: Option<u8>,
    par_p4: Option<u16>,
    par_p5: Option<u16>,
    par_p6: Option<u8>,
    par_p7: Option<u8>,
    par_p8: Option<u16>,
    par_p9: Option<u16>,
    par_p10: Option<u8>,
    par_h1: Option<u16>,
    par_h2: Option<u16>,
    par_h3: Option<u8>,
    par_h4: Option<u8>,
    par_h5: Option<u8>,
    par_h6: Option<u8>,
    par_h7: Option<u8>,
    par_g1: Option<u8>,
    par_g2: Option<u16>,
    par_g3: Option<u8>,
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
        /// Returns the chip ID. For BME68x devices, this value will always be `0x61`.
        chip_id
    );
    once_ro!(
        /// Returns the variant ID. For the BME688, this value will be `0x01`.
        variant_id
    );
    once_ro!(
        /// Returns temperature calibration parameter 1.
        par_t1,
        [par_t1_msb, par_t1_lsb]
    );
    once_ro!(
        /// Returns temperature calibration parameter 2.
        par_t2,
        [par_t2_msb, par_t2_lsb]
    );
    once_ro!(
        /// Returns temperature calibration parameter 3.
        par_t3
    );
    once_ro!(
        /// Returns pressure calibration parameter 1.
        par_p1,
        [par_p1_msb, par_p1_lsb]
    );
    once_ro!(
        /// Returns pressure calibration parameter 2.
        par_p2,
        [par_p2_msb, par_p2_lsb]
    );
    once_ro!(
        /// Returns pressure calibration parameter 3.
        par_p3
    );
    once_ro!(
        /// Returns pressure calibration parameter 4.
        par_p4,
        [par_p4_msb, par_p4_lsb]
    );
    once_ro!(
        /// Returns pressure calibration parameter 5.
        par_p5, [par_p5_msb, par_p5_lsb]);
    once_ro!(
        /// Returns pressure calibration parameter 6.
        par_p6
    );
    once_ro!(
        /// Returns pressure calibration parameter 7.
        par_p7
    );
    once_ro!(
        /// Returns pressure calibration parameter 8.
        par_p8,
        [par_p8_msb, par_p8_lsb]
    );
    once_ro!(
        /// Returns pressure calibration parameter 9.
        par_p9,
        [par_p9_msb, par_p9_lsb]
    );
    once_ro!(
        /// Returns pressure calibration parameter 10.
        par_p10
    );
    once_ro!(
        /// Returns humidity calibration parameter 1.
        par_h1,
        [par_h1_msb, par_h1_lsb], |msb, lsb| (msb as u16)
        << 4
        | (lsb as u16) & 0b0000_1111);
    once_ro!(
        /// Returns humidity calibration parameter 2.
        par_h2,
        [par_h2_msb, par_h2_lsb], |msb, lsb| (msb as u16)
        << 4
        | (lsb as u16) >> 4);
    once_ro!(
        /// Returns humidity calibration parameter 3.
        par_h3
    );
    once_ro!(
        /// Returns humidity calibration parameter 4.
        par_h4
    );
    once_ro!(
        /// Returns humidity calibration parameter 5.
        par_h5
    );
    once_ro!(
        /// Returns humidity calibration parameter 6.
        par_h6
    );
    once_ro!(
        /// Returns humidity calibration parameter 7.
        par_h7
    );
    once_ro!(
        /// Returns gas calibration parameter 1.
        par_g1
    );
    once_ro!(
        /// Returns gas calibration parameter 2.
        par_g2,
        [par_g2_msb, par_g2_lsb]
    );
    once_ro!(
        /// Returns gas calibration parameter 3.
        par_g3
    );

    live_ro!(gas_r_2, [gas_r_msb_2, gas_r_lsb_2], |msb, lsb| (msb as u16)
        << 2
        | (lsb as u16) >> 6);

    live_ro!(
        /// Returns raw humidity value 0.
        hum_adc_0,
        [hum_msb_0, hum_lsb_0]
    );

    live_ro!(
        /// Returns raw temperature value 2.
        temp_adc_2,
        [temp_msb_2, temp_lsb_2, temp_xlsb_2],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(
        /// Returns raw pressure value 2.
        press_adc_2,
        [press_msb_2, press_lsb_2, press_xlsb_2],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(gas_r_1, [gas_r_msb_1, gas_r_lsb_1], |msb, lsb| (msb as u16)
        << 2
        | (lsb as u16) >> 6);

    live_ro!(
        /// Returns raw humidity value 1.
        hum_adc_1,
        [hum_msb_1, hum_lsb_1]
    );

    live_ro!(
        /// Returns raw temperature value 1.
        temp_adc_1,
        [temp_msb_1, temp_lsb_1, temp_xlsb_1],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(
        /// Returns raw humidity value 2.
        hum_adc_2,
        [hum_msb_2, hum_lsb_2]
    );

    live_ro!(
        /// Returns raw pressure value 1.
        press_adc_1,
        [press_msb_1, press_lsb_1, press_xlsb_1],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(gas_r_0, [gas_r_msb_0, gas_r_lsb_0], |msb, lsb| (msb as u16)
        << 2
        | (lsb as u16) >> 6);

    live_ro!(
        /// Returns raw temperature value 0.
        temp_adc_0,
        [temp_msb_0, temp_lsb_0, temp_xlsb_0],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(
        /// Returns raw pressure value 0.
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
        Ok(Filter::try_from((self.read_config()? & 0b0001_1100) >> 2)?)
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
    reg_rw!(ctrl_hum, u8);
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

    /// Returns the compensated temperature as a fixed-point value for field 0.
    pub fn get_temp_comp_0(&mut self) -> Result<i32> {
        Ok(((self.get_t_fine_0()? * 5) + 128) >> 8)
    }

    fn get_t_fine_1(&mut self) -> Result<i32> {
        let temp_adc_1 = self.get_temp_adc_1()? as i32;

        Ok(self.get_tfine(temp_adc_1)?)
    }

    /// Returns the compensated temperature as a fixed-point value for field 1.
    pub fn get_temp_comp_1(&mut self) -> Result<i32> {
        Ok(((self.get_t_fine_1()? * 5) + 128) >> 8)
    }

    fn get_t_fine_2(&mut self) -> Result<i32> {
        let temp_adc_2 = self.get_temp_adc_2()? as i32;

        Ok(self.get_tfine(temp_adc_2)?)
    }

    /// Returns the compensated temperature as a fixed-point value for field 2.
    pub fn get_temp_comp_2(&mut self) -> Result<i32> {
        Ok(((self.get_t_fine_2()? * 5) + 128) >> 8)
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
