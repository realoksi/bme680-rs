#![no_std]
#![allow(dead_code)]
#![deny(unsafe_code)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

mod macros;

use macros::*;

pub type Result<T> = core::result::Result<T, ()>;

pub trait Register {
    const I2C_ADDR: u8;
    const SPI_ADDR: u8;
    const PAGE: u8;
}

physical!( status i2c(0x73) spi(0x73, 1) );
physical!( variant_id i2c(0xF0) spi(0x70, 0) );
physical!( reset i2c(0xE0) spi(0x60, 0) );
physical!( chip_id i2c(0xD0) spi(0x50, 0) );
physical!( config i2c(0x75) spi(0x75, 1) );
physical!( ctrl_meas i2c(0x74) spi(0x74, 1) );
physical!( ctrl_hum i2c(0x72) spi(0x72, 1) );
physical!( ctrl_gas_1 i2c(0x71) spi(0x71, 1) );
physical!( ctrl_gas_0 i2c(0x70) spi(0x70, 1) );
physical!( gas_wait_shared i2c(0x6E) spi(0x6E, 1) );
physical!( gas_wait_9 i2c(0x6D) spi(0x6D, 1) );
physical!( gas_wait_8 i2c(0x6C) spi(0x6C, 1) );
physical!( gas_wait_7 i2c(0x6B) spi(0x6B, 1) );
physical!( gas_wait_6 i2c(0x6A) spi(0x6A, 1) );
physical!( gas_wait_5 i2c(0x69) spi(0x69, 1) );
physical!( gas_wait_4 i2c(0x68) spi(0x68, 1) );
physical!( gas_wait_3 i2c(0x67) spi(0x67, 1) );
physical!( gas_wait_2 i2c(0x66) spi(0x66, 1) );
physical!( gas_wait_1 i2c(0x65) spi(0x65, 1) );
physical!( gas_wait_0 i2c(0x64) spi(0x64, 1) );
physical!( res_heat_9 i2c(0x63) spi(0x63, 1) );
physical!( res_heat_8 i2c(0x62) spi(0x62, 1) );
physical!( res_heat_7 i2c(0x61) spi(0x61, 1) );
physical!( res_heat_6 i2c(0x60) spi(0x60, 1) );
physical!( res_heat_5 i2c(0x5F) spi(0x5F, 1) );
physical!( res_heat_4 i2c(0x5E) spi(0x5E, 1) );
physical!( res_heat_3 i2c(0x5D) spi(0x5D, 1) );
physical!( res_heat_2 i2c(0x5C) spi(0x5C, 1) );
physical!( res_heat_1 i2c(0x5B) spi(0x5B, 1) );
physical!( res_heat_0 i2c(0x5A) spi(0x5A, 1) );
physical!( idac_heat_9 i2c(0x59) spi(0x59, 1) );
physical!( idac_heat_8 i2c(0x58) spi(0x58, 1) );
physical!( idac_heat_7 i2c(0x57) spi(0x57, 1) );
physical!( idac_heat_6 i2c(0x56) spi(0x56, 1) );
physical!( idac_heat_5 i2c(0x55) spi(0x55, 1) );
physical!( idac_heat_4 i2c(0x54) spi(0x54, 1) );
physical!( idac_heat_3 i2c(0x53) spi(0x53, 1) );
physical!( idac_heat_2 i2c(0x52) spi(0x52, 1) );
physical!( idac_heat_1 i2c(0x51) spi(0x51, 1) );
physical!( idac_heat_0 i2c(0x50) spi(0x50, 1) );
physical!( gas_r_lsb_2 i2c(0x4F) spi(0x4F, 1) );
physical!( gas_r_msb_2 i2c(0x4E) spi(0x4E, 1) );
physical!( hum_lsb_2 i2c(0x48) spi(0x48, 1) );
physical!( hum_msb_2 i2c(0x47) spi(0x47, 1) );
physical!( temp_xlsb_2 i2c(0x46) spi(0x46, 1) );
physical!( temp_lsb_2 i2c(0x45) spi(0x45, 1) );
physical!( temp_msb_2 i2c(0x44) spi(0x44, 1) );
physical!( press_xlsb_2 i2c(0x43) spi(0x43, 1) );
physical!( press_lsb_2 i2c(0x42) spi(0x42, 1) );
physical!( press_msb_2 i2c(0x41) spi(0x41, 1) );
physical!( sub_meas_index_2 i2c(0x40) spi(0x40, 1) );
physical!( meas_status_2 i2c(0x3F) spi(0x3F, 1) );
physical!( gas_r_lsb_1 i2c(0x3E) spi(0x3E, 1) );
physical!( gas_r_msb_1 i2c(0x3D) spi(0x3D, 1) );
physical!( hum_lsb_1 i2c(0x37) spi(0x37, 1) );
physical!( hum_msb_1 i2c(0x36) spi(0x36, 1) );
physical!( temp_xlsb_1 i2c(0x35) spi(0x35, 1) );
physical!( temp_lsb_1 i2c(0x34) spi(0x34, 1) );
physical!( temp_msb_1 i2c(0x33) spi(0x33, 1) );
physical!( press_xlsb_1 i2c(0x32) spi(0x32, 1) );
physical!( press_lsb_1 i2c(0x31) spi(0x31, 1) );
physical!( press_msb_1 i2c(0x30) spi(0x30, 1) );
physical!( sub_meas_index_1 i2c(0x2F) spi(0x2F, 1) );
physical!( meas_status_1 i2c(0x2E) spi(0x2E, 1) );
physical!( gas_r_lsb_0 i2c(0x2D) spi(0x2D, 1) );
physical!( gas_r_msb_0 i2c(0x2C) spi(0x2C, 1) );
physical!( hum_lsb_0 i2c(0x26) spi(0x26, 1) );
physical!( hum_msb_0 i2c(0x25) spi(0x25, 1) );
physical!( temp_xlsb_0 i2c(0x24) spi(0x24, 1) );
physical!( temp_lsb_0 i2c(0x23) spi(0x23, 1) );
physical!( temp_msb_0 i2c(0x22) spi(0x22, 1) );
physical!( press_xlsb_0 i2c(0x21) spi(0x21, 1) );
physical!( press_lsb_0 i2c(0x20) spi(0x20, 1) );
physical!( press_msb_0 i2c(0x1F) spi(0x1F, 1) );
physical!( sub_meas_index_0 i2c(0x1E) spi(0x1E, 1) );
physical!( meas_status_0 i2c(0x1D) spi(0x1D, 1) );
physical!( par_g1 i2c(0xED) spi(0xED, 1) );
physical!( par_g2_lsb i2c(0xEB) spi(0xEB, 1) );
physical!( par_g2_msb i2c(0xEC) spi(0xEC, 1) );
physical!( par_g3 i2c(0xEE) spi(0xEE, 1) );
physical!( res_heat_range i2c(0x02) spi(0x02, 1) );
physical!( res_heat_val i2c(0x00) spi(0x00, 1) );
physical!( par_h1_lsb i2c(0xE2) spi(0xE2, 1) );
physical!( par_h1_msb i2c(0xE3) spi(0xE3, 1) );
physical!( par_h2_lsb i2c(0xE2) spi(0xE2, 1) );
physical!( par_h2_msb i2c(0xE1) spi(0xE1, 1) );
physical!( par_h3 i2c(0xE4) spi(0xE4, 1) );
physical!( par_h4 i2c(0xE5) spi(0xE5, 1) );
physical!( par_h5 i2c(0xE6) spi(0xE6, 1) );
physical!( par_h6 i2c(0xE7) spi(0xE7, 1) );
physical!( par_h7 i2c(0xE8) spi(0xE8, 1) );
physical!( par_p10 i2c(0xA0) spi(0xA0, 1) );
physical!( par_p1_lsb i2c(0x8E) spi(0x8E, 1) );
physical!( par_p1_msb i2c(0x8F) spi(0x8F, 1) );
physical!( par_p2_lsb i2c(0x90) spi(0x90, 1) );
physical!( par_p2_msb i2c(0x91) spi(0x91, 1) );
physical!( par_p3 i2c(0x92) spi(0x92, 1) );
physical!( par_p4_lsb i2c(0x94) spi(0x94, 1) );
physical!( par_p4_msb i2c(0x95) spi(0x95, 1) );
physical!( par_p5_lsb i2c(0x96) spi(0x96, 1) );
physical!( par_p5_msb i2c(0x97) spi(0x97, 1) );
physical!( par_p6 i2c(0x99) spi(0x99, 1) );
physical!( par_p7 i2c(0x98) spi(0x98, 1) );
physical!( par_p8_lsb i2c(0x9C) spi(0x9C, 1) );
physical!( par_p8_msb i2c(0x9D) spi(0x9D, 1) );
physical!( par_p9_lsb i2c(0x9E) spi(0x9E, 1) );
physical!( par_p9_msb i2c(0x9F) spi(0x9F, 1) );
physical!( par_t1_lsb i2c(0xE9) spi(0xE9, 1) );
physical!( par_t1_msb i2c(0xEA) spi(0xEA, 1) );
physical!( par_t2_lsb i2c(0x8A) spi(0x8A, 1) );
physical!( par_t2_msb i2c(0x8B) spi(0x8B, 1) );
physical!( par_t3 i2c(0x8C) spi(0x8C, 1) );
physical!( gas_range_0 i2c(0x2D) spi(0x2D, 1));
physical!( gas_range_1 i2c(0x3E) spi(0x3E, 1));
physical!( gas_range_2 i2c(0x4F) spi(0x4F, 1));

/// iir filter coefficient. useful for suppressing environmental disturbances
///
/// see [BME680::set_filter]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Os {
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

impl TryFrom<u8> for Os {
    type Error = ();

    fn try_from(value: u8) -> Result<Os> {
        Ok(match value {
            0b000 => Os::X0,
            0b001 => Os::X1,
            0b010 => Os::X2,
            0b011 => Os::X4,
            0b100 => Os::X8,
            0b101 => Os::X16,
            _ => return Err(()),
        })
    }
}

/// sensor power mode
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

pub trait Bus {
    fn read<R: Register>(&mut self) -> Result<u8>;
    fn write<R: Register>(&mut self, value: u8) -> Result<()>;
}

pub struct I2cBus<I> {
    inner: I,
    slave_addr: u8,
}

impl<I> Bus for I2cBus<I>
where
    I: embedded_hal::i2c::I2c,
{
    fn read<R: Register>(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];

        self.inner
            .write_read(self.slave_addr, &[R::I2C_ADDR], &mut buf)
            .map_err(|_| ())?;

        Ok(buf[0])
    }

    fn write<R: Register>(&mut self, value: u8) -> Result<()> {
        self.inner
            .write(self.slave_addr, &[R::I2C_ADDR, value])
            .map_err(|_| ())
    }
}

pub struct SpiBus<S> {
    inner: S,
    page: Option<u8>,
}

impl<S> Bus for SpiBus<S>
where
    S: embedded_hal::spi::SpiDevice<u8>,
{
    fn read<R: Register>(&mut self) -> Result<u8> {
        self.set_page(R::PAGE)?;

        let mut buf = [0u8; 1];

        self.inner
            .transaction(&mut [
                embedded_hal::spi::Operation::Write(&[R::SPI_ADDR | 0x80]),
                embedded_hal::spi::Operation::Read(&mut buf),
            ])
            .map_err(|_| ())?;

        Ok(buf[0])
    }

    fn write<R: Register>(&mut self, value: u8) -> Result<()> {
        self.set_page(R::PAGE)?;

        self.inner
            .write(&[R::SPI_ADDR & 0x7F, value])
            .map_err(|_| ())
    }
}

impl<S> SpiBus<S>
where
    S: embedded_hal::spi::SpiDevice<u8>,
{
    fn set_page(&mut self, page: u8) -> Result<()> {
        if self.page == Some(page) {
            return Ok(());
        }

        let mut status = [0u8; 1];

        self.inner
            .transaction(&mut [
                embedded_hal::spi::Operation::Write(&[Status::SPI_ADDR | 0x80]),
                embedded_hal::spi::Operation::Read(&mut status),
            ])
            .map_err(|_| ())?;

        self.inner
            .write(&[Status::SPI_ADDR, (status[0] & !0b0001_0000) | page << 4])
            .map_err(|_| ())?;

        self.page = Some(page);

        Ok(())
    }
}

/// # Example
///
/// ```
/// # use bme680_rs::*;
/// # use embedded_hal_mock::eh1::i2c::{Transaction, Mock};
/// #
/// # let i2c = Mock::new(&[
/// #     Transaction::write_read(0x76, vec![0xF0], vec![0x01]),
/// # ]);
/// #
/// let mut bme688 = BME680::from_i2c(i2c, 0x76);
///
/// assert_eq!(bme688.get_variant_id().unwrap(), 0x01);
/// #
/// # bme688.into_inner().done();
/// ```
pub struct BME680<T> {
    bus: T,
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

impl<T> BME680<T>
where
    T: Bus,
{
    fn new(bus: T) -> Self {
        Self {
            bus,
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

    logical_ro!( once: chip_id u8 );
    logical_ro!( once: variant_id u8 );
    logical_ro!( once: par_t1 u16 [par_t1_msb, par_t1_lsb] );
    logical_ro!( once: par_t2 i16 [par_t2_msb, par_t2_lsb] );
    logical_ro!( once: par_t3 i8 );
    logical_ro!( once: par_p1 u16 [par_p1_msb, par_p1_lsb] );
    logical_ro!( once: par_p2 i16 [par_p2_msb, par_p2_lsb] );
    logical_ro!( once: par_p3 i8 );
    logical_ro!( once: par_p4 i16 [par_p4_msb, par_p4_lsb] );
    logical_ro!( once: par_p5 i16 [par_p5_msb, par_p5_lsb] );
    logical_ro!( once: par_p6 i8 );
    logical_ro!( once: par_p7 i8 );
    logical_ro!( once: par_p8 i16 [par_p8_msb, par_p8_lsb] );
    logical_ro!( once: par_p9 i16 [par_p9_msb, par_p9_lsb] );
    logical_ro!( once: par_p10 u8 );
    logical_ro!( once: par_h1 u16 [par_h1_msb, par_h1_lsb] |msb, lsb| (msb as u16) << 4 | (lsb as u16) & 0b0000_1111);
    logical_ro!( once: par_h2 u16 [par_h2_msb, par_h2_lsb] |msb, lsb| (msb as u16) << 4 | (lsb as u16) >> 4 );
    logical_ro!( once: par_h3 i8 );
    logical_ro!( once: par_h4 i8 );
    logical_ro!( once: par_h5 i8 );
    logical_ro!( once: par_h6 u8 );
    logical_ro!( once: par_h7 u8 );
    logical_ro!( once: par_g1 i8 );
    logical_ro!( once: par_g2 i16 [par_g2_msb, par_g2_lsb] );
    logical_ro!( once: par_g3 i8 );

    logical_ro!( always: gas_r_2 u16 [gas_r_msb_2, gas_r_lsb_2] |msb, lsb| (msb as u16) << 2 | (lsb as u16) >> 6 );
    logical_ro!( always: hum_adc_0 u16 [hum_msb_0, hum_lsb_0] );
    logical_ro!( always: temp_adc_2 u32 [temp_msb_2, temp_lsb_2, temp_xlsb_2] |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4 );
    logical_ro!( always: press_adc_2 u32 [press_msb_2, press_lsb_2, press_xlsb_2] |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4 );
    logical_ro!( always: gas_r_1 u16 [gas_r_msb_1, gas_r_lsb_1] |msb, lsb| (msb as u16) << 2 | (lsb as u16) >> 6 );
    logical_ro!( always: hum_adc_1 u16 [hum_msb_1, hum_lsb_1] );
    logical_ro!( always: temp_adc_1 u32 [temp_msb_1, temp_lsb_1, temp_xlsb_1] |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4 );
    logical_ro!( always: hum_adc_2 u16 [hum_msb_2, hum_lsb_2] );
    logical_ro!( always: press_adc_1 u32 [press_msb_1, press_lsb_1, press_xlsb_1] |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4 );
    logical_ro!( always: gas_r_0 u16 [gas_r_msb_0, gas_r_lsb_0]  |msb, lsb| (msb as u16) << 2 | (lsb as u16) >> 6);
    logical_ro!( always: temp_adc_0 u32 [temp_msb_0, temp_lsb_0, temp_xlsb_0] |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4 );
    logical_ro!( always: press_adc_0 u32 [press_msb_0, press_lsb_0, press_xlsb_0] |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4 );
    logical_ro!( always: meas_status_0 u8 );
    logical_ro!( always: meas_status_1 u8 );
    logical_ro!( always: meas_status_2 u8 );
    logical_ro!( always: sub_meas_index_0 u8 );
    logical_ro!( always: sub_meas_index_1 u8 );
    logical_ro!( always: sub_meas_index_2 u8 );
    logical_ro!( always: res_heat_range u8 |a| (a >> 4) & 0b11 );
    logical_ro!( always: res_heat_val i8 ); // Should this be `once`?
    logical_ro!( always: gas_range_0 u8 |a| a & 0b1111 );
    logical_ro!( always: gas_range_1 u8 |a| a & 0b1111 );
    logical_ro!( always: gas_range_2 u8 |a| a & 0b1111 );

    logical_rw!( always: ctrl_gas_1 u8 );
    logical_rw!( always: ctrl_gas_0 u8 );
    logical_rw!( always: gas_wait_shared u8 );
    logical_rw!( always: status u8 );

    pub fn get_spi_mem_page(&mut self) -> Result<bool> {
        Ok(((self.get_status()? & 0b0001_0000) >> 4) != 0)
    }

    pub fn set_spi_mem_page(&mut self, value: bool) -> Result<()> {
        let status = (self.get_status()? & !0b0001_0000) | ((value as u8) << 4);
        self.set_status(status)
    }

    logical_rw!( always: reset u8 );

    pub fn soft_reset(&mut self) -> Result<()> {
        self.set_reset(0xB6)
    }

    logical_rw!( always: config u8 );

    pub fn get_filter(&mut self) -> Result<Filter> {
        Filter::try_from((self.get_config()? & 0b0001_1100) >> 2)
    }

    pub fn set_filter(&mut self, value: Filter) -> Result<()> {
        let config = (self.get_config()? & !0b0001_1100) | ((value as u8) << 2);

        self.set_config(config)
    }

    pub fn get_spi_3w_en(&mut self) -> Result<bool> {
        Ok((self.get_config()? & 0b1) != 0)
    }

    pub fn set_spi_3w_en(&mut self, value: bool) -> Result<()> {
        let config = (self.get_config()? & !0b1) | (value as u8);

        self.set_config(config)
    }

    logical_rw!( always: ctrl_meas u8 );

    pub fn get_osrs_t(&mut self) -> Result<Os> {
        Os::try_from((self.get_ctrl_meas()? & 0b1110_0000) >> 5)
    }

    pub fn set_osrs_t(&mut self, value: Os) -> Result<()> {
        let ctrl_meas = (self.get_ctrl_meas()? & !0b1110_0000) | ((value as u8) << 5);

        self.set_ctrl_meas(ctrl_meas)
    }

    pub fn get_osrs_p(&mut self) -> Result<Os> {
        Os::try_from((self.get_ctrl_meas()? & 0b0001_1100) >> 2)
    }

    pub fn set_osrs_p(&mut self, value: Os) -> Result<()> {
        let ctrl_meas = (self.get_ctrl_meas()? & !0b0001_1100) | ((value as u8) << 2);

        self.set_ctrl_meas(ctrl_meas)
    }

    pub fn get_mode(&mut self) -> Result<Mode> {
        Mode::try_from(self.get_ctrl_meas()? & 0b11)
    }

    pub fn set_mode(&mut self, value: Mode) -> Result<()> {
        let ctrl_meas = (self.get_ctrl_meas()? & !0b11) | (value as u8);

        self.set_ctrl_meas(ctrl_meas)
    }

    logical_rw!( always: ctrl_hum u8 );

    pub fn get_osrs_h(&mut self) -> Result<Os> {
        Os::try_from(self.get_ctrl_hum()? & 0b0000_0111)
    }

    pub fn set_osrs_h(&mut self, value: Os) -> Result<()> {
        let ctrl_hum = (self.get_ctrl_hum()? & !0b0000_0111) | (value as u8);

        self.set_ctrl_hum(ctrl_hum)
    }

    fn get_t_fine(&mut self, temp_adc: i32) -> Result<i32> {
        let par_t1 = self.get_par_t1()? as i32;
        let par_t2 = self.get_par_t2()? as i32;
        let par_t3 = self.get_par_t3()? as i32;

        let var1 = (temp_adc >> 3) - (par_t1 << 1);
        let var2 = (var1 * par_t2) >> 11;
        let var3 = ((((var1 >> 1) * (var1 >> 1)) >> 12) * (par_t3 << 4)) >> 14;

        Ok(var2 + var3)
    }

    pub fn get_t_fine_0(&mut self) -> Result<i32> {
        let temp_adc_0 = self.get_temp_adc_0()? as i32;

        Ok(self.get_t_fine(temp_adc_0)?)
    }

    pub fn get_temp_comp_0(&mut self) -> Result<i32> {
        Ok(((self.get_t_fine_0()? * 5) + 128) >> 8)
    }

    fn get_t_fine_1(&mut self) -> Result<i32> {
        let temp_adc_1 = self.get_temp_adc_1()? as i32;

        Ok(self.get_t_fine(temp_adc_1)?)
    }

    pub fn get_temp_comp_1(&mut self) -> Result<i32> {
        Ok(((self.get_t_fine_1()? * 5) + 128) >> 8)
    }

    fn get_t_fine_2(&mut self) -> Result<i32> {
        let temp_adc_2 = self.get_temp_adc_2()? as i32;

        Ok(self.get_t_fine(temp_adc_2)?)
    }

    pub fn get_temp_comp_2(&mut self) -> Result<i32> {
        Ok(((self.get_t_fine_2()? * 5) + 128) >> 8)
    }

    fn get_press_comp(&mut self, t_fine: i32, press_adc: u32) -> Result<i32> {
        let mut var1 = (t_fine >> 1) - 64000;
        let mut var2 = ((((var1 >> 2) * (var1 >> 2)) >> 11) * (self.get_par_p6()? as i32)) >> 2;

        var2 = var2 + ((var1 * (self.get_par_p5()? as i32)) << 1);
        var2 = (var2 >> 2) + ((self.get_par_p4()? as i32) << 16);
        var1 = (((((var1 >> 2) * (var1 >> 2)) >> 13) * ((self.get_par_p3()? as i32) << 5)) >> 3)
            + (((self.get_par_p2()? as i32) * var1) >> 1);
        var1 = var1 >> 18;
        var1 = ((32768 + var1) * (self.get_par_p1()? as i32)) >> 15;

        let press_comp = 1048576 - press_adc as i32;
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

    pub fn get_press_comp_0(&mut self) -> Result<i32> {
        let t_fine_0 = self.get_t_fine_0()?;
        let press_adc_0 = self.get_press_adc_0()?;

        self.get_press_comp(t_fine_0, press_adc_0)
    }

    pub fn get_press_comp_1(&mut self) -> Result<i32> {
        let t_fine_1 = self.get_t_fine_1()?;
        let press_adc_1 = self.get_press_adc_1()?;

        self.get_press_comp(t_fine_1, press_adc_1)
    }

    pub fn get_press_comp_2(&mut self) -> Result<i32> {
        let t_fine_2 = self.get_t_fine_2()?;
        let press_adc_2 = self.get_press_adc_2()?;

        self.get_press_comp(t_fine_2, press_adc_2)
    }

    pub fn get_hum_comp_0(&mut self) -> Result<i32> {
        let temp_comp_0 = self.get_temp_comp_0()?;

        let var1 = (self.get_hum_adc_0()? as i32)
            - ((self.get_par_h1()? as i32) << 4)
            - (((temp_comp_0 * (self.get_par_h3()? as i32)) / 100) >> 1);
        let var2 = ((self.get_par_h2()? as i32)
            * (((temp_comp_0 * (self.get_par_h4()? as i32)) / 100)
                + (((temp_comp_0 * ((temp_comp_0 * (self.get_par_h5()? as i32)) / 100)) >> 6)
                    / 100)
                + (1 << 14)))
            >> 10;
        let var3 = var1 * var2;
        let var4 = (((self.get_par_h6()? as i32) << 7)
            + ((temp_comp_0 * (self.get_par_h7()? as i32)) / 100))
            >> 4;
        let var5 = ((var3 >> 14) * (var3 >> 14)) >> 10;
        let var6 = (var4 * var5) >> 1;

        let hum_comp_0 = (((var3 + var6) >> 10) * 1000) >> 12;

        Ok(hum_comp_0)
    }
}

impl<I> BME680<I2cBus<I>>
where
    I: embedded_hal::i2c::I2c,
{
    /// `slave_address` must be either `0x76` or `0x77`.
    pub fn from_i2c(i2c: I, slave_address: u8) -> Self {
        Self::new(I2cBus {
            inner: i2c,
            slave_addr: slave_address,
        })
    }

    pub fn into_inner(self) -> I {
        self.bus.inner
    }
}

impl<S> BME680<SpiBus<S>>
where
    S: embedded_hal::spi::SpiDevice<u8>,
{
    pub fn from_spi(spi: S) -> Self {
        Self::new(SpiBus {
            inner: spi,
            page: None,
        })
    }

    pub fn into_inner(self) -> S {
        self.bus.inner
    }
}
