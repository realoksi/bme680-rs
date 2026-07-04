#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

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

/// A helper macro for accessing simple registers from `memory_map`.
macro_rules! reg {
    ($name:ident, $result:ident) => {
        paste::paste! {
            pub(crate) fn[<get_ $name>](&mut self) -> Result<$result> {
                memory_map::$name::read(&mut self.transport_layer)
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

    reg!(status, u8);
    reg!(variant_id, u8);
    reg!(reset, u8);
    reg!(chip_id, u8);
    reg!(config, u8);
    reg!(ctrl_meas, u8);
    reg!(ctrl_hum, u8);
    reg!(ctrl_gas_1, u8);
    reg!(ctrl_gas_0, u8);
    reg!(gas_wait_shared, u8);
    reg!(gas_r_lsb_2, u8);
    reg!(gas_r_msb_2, u8);
    reg!(hum_lsb_2, u8);
    reg!(hum_msb_2, u8);
    reg!(temp_xlsb_2, u8);
    reg!(temp_lsb_2, u8);
    reg!(temp_msb_2, u8);
    reg!(press_xlsb_2, u8);
    reg!(press_lsb_2, u8);
    reg!(press_msb_2, u8);
    reg!(sub_meas_index_2, u8);
    reg!(meas_status_2, u8);
    reg!(gas_r_lsb_1, u8);
    reg!(gas_r_msb_1, u8);
    reg!(hum_lsb_1, u8);
    reg!(hum_msb_1, u8);
    reg!(temp_xlsb_1, u8);
    reg!(temp_lsb_1, u8);
    reg!(temp_msb_1, u8);
    reg!(press_xlsb_1, u8);
    reg!(press_lsb_1, u8);
    reg!(press_msb_1, u8);
    reg!(sub_meas_index_1, u8);
    reg!(meas_status_1, u8);
    reg!(gas_r_lsb_0, u8);
    reg!(gas_r_msb_0, u8);
    reg!(hum_lsb_0, u8);
    reg!(hum_msb_0, u8);
    reg!(temp_xlsb_0, u8);
    reg!(temp_lsb_0, u8);
    reg!(temp_msb_0, u8);
    reg!(press_xlsb_0, u8);
    reg!(press_lsb_0, u8);
    reg!(press_msb_0, u8);
    reg!(sub_meas_index_0, u8);
    reg!(meas_status_0, u8);
    reg!(par_t1_lsb, u8);
    reg!(par_t1_msb, u8);
    reg!(par_t2_lsb, u8);
    reg!(par_t2_msb, u8);
    reg!(par_t3, u8);
    reg!(par_p1_lsb, u8);
    reg!(par_p1_msb, u8);
    reg!(par_p2_lsb, u8);
    reg!(par_p2_msb, u8);
    reg!(par_p3, u8);
    reg!(par_p4_lsb, u8);
    reg!(par_p4_msb, u8);
    reg!(par_p5_lsb, u8);
    reg!(par_p5_msb, u8);
    reg!(par_p6, u8);
    reg!(par_p7, u8);
    reg!(par_p8_lsb, u8);
    reg!(par_p8_msb, u8);
    reg!(par_p9_lsb, u8);
    reg!(par_p9_msb, u8);
    reg!(par_p10, u8);
    reg!(par_h1_lsb, u8);
    reg!(par_h1_msb, u8);
    reg!(par_h2_lsb, u8);
    reg!(par_h2_msb, u8);
    reg!(par_h3, u8);
    reg!(par_h4, u8);
    reg!(par_h5, u8);
    reg!(par_h6, u8);
    reg!(par_h7, u8);
    reg!(par_g1, u8);
    reg!(par_g2_lsb, u8);
    reg!(par_g2_msb, u8);
    reg!(par_g3, u8);
    reg!(temp_adc_0_lsb, u8);
    reg!(temp_adc_0_msb, u8);
    reg!(temp_adc_0_xlsb, u8);
    reg!(temp_adc_1_lsb, u8);
    reg!(temp_adc_1_msb, u8);
    reg!(temp_adc_1_xlsb, u8);
    reg!(temp_adc_2_lsb, u8);
    reg!(temp_adc_2_msb, u8);
    reg!(temp_adc_2_xlsb, u8);
    reg!(press_adc_0_lsb, u8);
    reg!(press_adc_0_msb, u8);
    reg!(press_adc_0_xlsb, u8);
    reg!(press_adc_1_lsb, u8);
    reg!(press_adc_1_msb, u8);
    reg!(press_adc_1_xlsb, u8);
    reg!(press_adc_2_lsb, u8);
    reg!(press_adc_2_msb, u8);
    reg!(press_adc_2_xlsb, u8);
    reg!(hum_adc_0_lsb, u8);
    reg!(hum_adc_0_msb, u8);
    reg!(hum_adc_1_lsb, u8);
    reg!(hum_adc_1_msb, u8);
    reg!(hum_adc_2_lsb, u8);
    reg!(hum_adc_2_msb, u8);
    reg!(res_heat_range, u8);
    reg!(res_heat_val, u8);
    reg!(gas_adc_0_lsb, u8);
    reg!(gas_adc_0_msb, u8);
    reg!(gas_adc_1_lsb, u8);
    reg!(gas_adc_1_msb, u8);
    reg!(gas_adc_2_lsb, u8);
    reg!(gas_adc_2_msb, u8);
    reg!(gas_range_0, u8);
    reg!(gas_range_1, u8);
    reg!(gas_range_2, u8);

}
