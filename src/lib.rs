#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod fields;
mod memory_map;

pub type Result<T> = core::result::Result<T, ()>;

pub trait Layer {
    fn read_byte(&mut self, addr: u8) -> Result<u8>;
    fn read_block(&mut self, addr: u8, data: &mut [u8]) -> Result<()> {
        for (i, k) in data.iter_mut().enumerate() {
            *k = self.read_byte(addr + (i as u8))?;
        }
        Ok(())
    }
    fn write_byte(&mut self, addr: u8, data: u8) -> Result<()>;
}

pub struct BME680<L>
where
    L: Layer,
{
    transport_layer: L,
}

/// A helper macro for accessing read-only registers from `memory_map`.
macro_rules! reg_ro {
    ($name:ident, $output:expr) => {
        paste::paste! {
            pub(crate) fn[<read_ $name>](&mut self) -> Result<$output> {
                Ok(memory_map::$name::read(&mut self.transport_layer)?.into())
            }
        }
    };
}

/// A helper macro for accessing read-write registers from `memory_map`.
macro_rules! reg_rw {
    ($name:ident, $input:expr) => {
        paste::paste! {
            pub(crate) fn[<read_ $name>](&mut self) -> Result<$input> {
                Ok(memory_map::$name::read(&mut self.transport_layer)?.into())
            }
        }

        paste::paste! {
            pub(crate) fn[<write_ $name>](&mut self, data: $input) -> Result<()> {
                memory_map::$name::write(&mut self.transport_layer, data.into())
            }
        }
    };
}

impl<L> BME680<L>
where
    L: Layer,
{
    pub fn new(transport_layer: L) -> Self {
        Self { transport_layer }
    }

    reg_rw!(status, fields::Status);
    reg_ro!(variant_id, u8);
    reg_rw!(reset, u8);
    reg_ro!(chip_id, u8);
    reg_rw!(config, fields::Config);
    reg_rw!(ctrl_meas, fields::CtrlMeas);
    reg_rw!(ctrl_hum, fields::CtrlHum);
    reg_rw!(ctrl_gas_1, fields::CtrlGas1);
    reg_rw!(ctrl_gas_0, fields::CtrlGas0);
    reg_rw!(gas_wait_shared, u8);
    reg_ro!(gas_r_lsb_2, fields::GasRLsb2);
    reg_ro!(gas_r_msb_2, u8);
    reg_ro!(hum_lsb_2, u8);
    reg_ro!(hum_msb_2, u8);
    reg_ro!(temp_xlsb_2, fields::TempXlsb2);
    reg_ro!(temp_lsb_2, u8);
    reg_ro!(temp_msb_2, u8);
    reg_ro!(press_xlsb_2, fields::PressXlsb2);
    reg_ro!(press_lsb_2, u8);
    reg_ro!(press_msb_2, u8);
    reg_ro!(sub_meas_index_2, u8);
    reg_ro!(meas_status_2, fields::MeasStatus2);
    reg_ro!(gas_r_lsb_1, u8);
    reg_ro!(gas_r_msb_1, u8);
    reg_ro!(hum_lsb_1, u8);
    reg_ro!(hum_msb_1, u8);
    reg_ro!(temp_xlsb_1, u8);
    reg_ro!(temp_lsb_1, u8);
    reg_ro!(temp_msb_1, u8);
    reg_ro!(press_xlsb_1, u8);
    reg_ro!(press_lsb_1, u8);
    reg_ro!(press_msb_1, u8);
    reg_ro!(sub_meas_index_1, u8);
    reg_ro!(meas_status_1, u8);
    reg_ro!(gas_r_lsb_0, u8);
    reg_ro!(gas_r_msb_0, u8);
    reg_ro!(hum_lsb_0, u8);
    reg_ro!(hum_msb_0, u8);
    reg_ro!(temp_xlsb_0, u8);
    reg_ro!(temp_lsb_0, u8);
    reg_ro!(temp_msb_0, u8);
    reg_ro!(press_xlsb_0, u8);
    reg_ro!(press_lsb_0, u8);
    reg_ro!(press_msb_0, u8);
    reg_ro!(sub_meas_index_0, u8);
    reg_ro!(meas_status_0, u8);
    reg_ro!(par_t1_lsb, u8);
    reg_ro!(par_t1_msb, u8);
    reg_ro!(par_t2_lsb, u8);
    reg_ro!(par_t2_msb, u8);
    reg_ro!(par_t3, u8);
    reg_ro!(par_p1_lsb, u8);
    reg_ro!(par_p1_msb, u8);
    reg_ro!(par_p2_lsb, u8);
    reg_ro!(par_p2_msb, u8);
    reg_ro!(par_p3, u8);
    reg_ro!(par_p4_lsb, u8);
    reg_ro!(par_p4_msb, u8);
    reg_ro!(par_p5_lsb, u8);
    reg_ro!(par_p5_msb, u8);
    reg_ro!(par_p6, u8);
    reg_ro!(par_p7, u8);
    reg_ro!(par_p8_lsb, u8);
    reg_ro!(par_p8_msb, u8);
    reg_ro!(par_p9_lsb, u8);
    reg_ro!(par_p9_msb, u8);
    reg_ro!(par_p10, u8);
    reg_ro!(par_h1_lsb, u8);
    reg_ro!(par_h1_msb, u8);
    reg_ro!(par_h2_lsb, u8);
    reg_ro!(par_h2_msb, u8);
    reg_ro!(par_h3, u8);
    reg_ro!(par_h4, u8);
    reg_ro!(par_h5, u8);
    reg_ro!(par_h6, u8);
    reg_ro!(par_h7, u8);
    reg_ro!(par_g1, u8);
    reg_ro!(par_g2_lsb, u8);
    reg_ro!(par_g2_msb, u8);
    reg_ro!(par_g3, u8);
    reg_ro!(temp_adc_0_lsb, u8);
    reg_ro!(temp_adc_0_msb, u8);
    reg_ro!(temp_adc_0_xlsb, u8);
    reg_ro!(temp_adc_1_lsb, u8);
    reg_ro!(temp_adc_1_msb, u8);
    reg_ro!(temp_adc_1_xlsb, u8);
    reg_ro!(temp_adc_2_lsb, u8);
    reg_ro!(temp_adc_2_msb, u8);
    reg_ro!(temp_adc_2_xlsb, u8);
    reg_ro!(press_adc_0_lsb, u8);
    reg_ro!(press_adc_0_msb, u8);
    reg_ro!(press_adc_0_xlsb, u8);
    reg_ro!(press_adc_1_lsb, u8);
    reg_ro!(press_adc_1_msb, u8);
    reg_ro!(press_adc_1_xlsb, u8);
    reg_ro!(press_adc_2_lsb, u8);
    reg_ro!(press_adc_2_msb, u8);
    reg_ro!(press_adc_2_xlsb, u8);
    reg_ro!(hum_adc_0_lsb, u8);
    reg_ro!(hum_adc_0_msb, u8);
    reg_ro!(hum_adc_1_lsb, u8);
    reg_ro!(hum_adc_1_msb, u8);
    reg_ro!(hum_adc_2_lsb, u8);
    reg_ro!(hum_adc_2_msb, u8);
    reg_ro!(res_heat_range, u8);
    reg_ro!(res_heat_val, u8);
    reg_ro!(gas_adc_0_lsb, u8);
    reg_ro!(gas_adc_0_msb, u8);
    reg_ro!(gas_adc_1_lsb, u8);
    reg_ro!(gas_adc_1_msb, u8);
    reg_ro!(gas_adc_2_lsb, u8);
    reg_ro!(gas_adc_2_msb, u8);
    reg_ro!(gas_range_0, u8);
    reg_ro!(gas_range_1, u8);
    reg_ro!(gas_range_2, u8);
}
