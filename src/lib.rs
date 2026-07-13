#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod fields;
#[cfg(any(feature = "embedded-hal"))]
mod impls;
mod macros;
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

    once_ro!(chip_id);
    once_ro!(variant_id);
    once_ro!(par_t1, [par_t1_msb, par_t1_lsb]);
    once_ro!(par_t2, [par_t2_msb, par_t2_lsb]);
    once_ro!(par_t3);
    once_ro!(par_p1, [par_p1_msb, par_p1_lsb]);
    once_ro!(par_p2, [par_p2_msb, par_p2_lsb]);
    once_ro!(par_p3);
    once_ro!(par_p4, [par_p4_msb, par_p4_lsb]);
    once_ro!(par_p5, [par_p5_msb, par_p5_lsb]);
    once_ro!(par_p6);
    once_ro!(par_p7);
    once_ro!(par_p8, [par_p8_msb, par_p8_lsb]);
    once_ro!(par_p9, [par_p9_msb, par_p9_lsb]);
    once_ro!(par_p10);
    once_ro!(par_h1, [par_h1_msb, par_h1_lsb], |msb, lsb| (msb as u16) << 4 | (lsb as u16) & 0b0000_1111);
    once_ro!(par_h2, [par_h2_msb, par_h2_lsb], |msb, lsb| (msb as u16) << 4 | (lsb as u16) >> 4);
    once_ro!(par_h3);
    once_ro!(par_h4);
    once_ro!(par_h5);
    once_ro!(par_h6);
    once_ro!(par_h7);
    once_ro!(par_g1);
    once_ro!(par_g2, [par_g2_msb, par_g2_lsb]);
    once_ro!(par_g3);

    live_ro!(gas_r_2, [gas_r_msb_2, gas_r_lsb_2], |msb, lsb| (msb as u16) << 2 | (lsb as u16) >> 6);
    live_ro!(hum_adc_0, [hum_msb_0, hum_lsb_0]);

    live_ro!(
        temp_adc_2, [temp_msb_2, temp_lsb_2, temp_xlsb_2],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(
        press_adc_2, [press_msb_2, press_lsb_2, press_xlsb_2],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(
        gas_r_1, [gas_r_msb_1, gas_r_lsb_1],
        |msb, lsb| (msb as u16) << 2 | (lsb as u16) >> 6
    );

    live_ro!(hum_adc_1, [hum_msb_1, hum_lsb_1]);

    live_ro!(
        temp_adc_1, [temp_msb_1, temp_lsb_1, temp_xlsb_1],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(hum_adc_2, [hum_msb_2, hum_lsb_2]);

    live_ro!(
        press_adc_1, [press_msb_1, press_lsb_1, press_xlsb_1],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(
        gas_r_0, [gas_r_msb_0, gas_r_lsb_0],
        |msb, lsb| (msb as u16) << 2 | (lsb as u16) >> 6
    );

    live_ro!(
        temp_adc_0, [temp_msb_0, temp_lsb_0, temp_xlsb_0],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    live_ro!(
        press_adc_0, [press_msb_0, press_lsb_0, press_xlsb_0],
        |msb, lsb, xlsb| (msb as u32) << 12 | (lsb as u32) << 4 | (xlsb as u32) >> 4
    );

    reg_rw!(status, fields::Status);
    reg_rw!(reset, u8);
    reg_rw!(config, fields::Config);
    reg_rw!(ctrl_meas, fields::CtrlMeas);
    reg_rw!(ctrl_hum, fields::CtrlHum);
    reg_rw!(ctrl_gas_1, fields::CtrlGas1);
    reg_rw!(ctrl_gas_0, fields::CtrlGas0);
    reg_rw!(gas_wait_shared, u8);
    reg_ro!(sub_meas_index_0, u8);
    reg_ro!(meas_status_0, fields::MeasStatus0);
    reg_ro!(sub_meas_index_2, u8);
    reg_ro!(meas_status_2, fields::MeasStatus2);
    reg_ro!(sub_meas_index_1, u8);
    reg_ro!(meas_status_1, fields::MeasStatus1);
    reg_ro!(hum_adc_0_lsb, u8);
    reg_ro!(hum_adc_0_msb, u8);
    reg_ro!(hum_adc_1_lsb, u8);
    reg_ro!(hum_adc_1_msb, u8);
    reg_ro!(hum_adc_2_lsb, u8);
    reg_ro!(hum_adc_2_msb, u8);
    reg_ro!(res_heat_range, u8);
    reg_ro!(res_heat_val, u8);
    reg_ro!(gas_range_0, u8);
    reg_ro!(gas_range_1, u8);
    reg_ro!(gas_range_2, u8);
}
