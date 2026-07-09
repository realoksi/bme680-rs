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
    reg_ro!(gas_r_lsb_1, fields::GasRLsb1);
    reg_ro!(gas_r_msb_1, u8);
    reg_ro!(hum_lsb_1, u8);
    reg_ro!(hum_msb_1, u8);
    reg_ro!(temp_xlsb_1, fields::TempXlsb1);
    reg_ro!(temp_lsb_1, u8);
    reg_ro!(temp_msb_1, u8);
    reg_ro!(press_xlsb_1, fields::PressXlsb1);
    reg_ro!(press_lsb_1, u8);
    reg_ro!(press_msb_1, u8);
    reg_ro!(sub_meas_index_1, u8);
    reg_ro!(meas_status_1, fields::MeasStatus1);
    reg_ro!(gas_r_lsb_0, fields::GasRLsb0);
    reg_ro!(gas_r_msb_0, u8);
    reg_ro!(hum_lsb_0, u8);
    reg_ro!(hum_msb_0, u8);
    reg_ro!(temp_xlsb_0, fields::TempXlsb0);
    reg_ro!(temp_lsb_0, u8);
    reg_ro!(temp_msb_0, u8);
    reg_ro!(press_xlsb_0, fields::PressXlsb0);
    reg_ro!(press_lsb_0, u8);
    reg_ro!(press_msb_0, u8);
    reg_ro!(sub_meas_index_0, u8);
    reg_ro!(meas_status_0, fields::MeasStatus0);

    reg_ro!(chip_id, u8);

    pub fn get_chip_id(&mut self) -> Result<u8> {
        match self.chip_id {
            Some(v) => Ok(v),
            None => {
                let chip_id = self.read_chip_id()?;
                self.chip_id = Some(chip_id);
                Ok(chip_id)
            }
        }
    }

    reg_ro!(variant_id, u8);

    pub fn get_variant_id(&mut self) -> Result<u8> {
        match self.variant_id {
            Some(v) => Ok(v),
            None => {
                let variant_id = self.read_variant_id()?;
                self.variant_id = Some(variant_id);
                Ok(variant_id)
            }
        }
    }

    reg_ro!(par_t1_lsb, u8);
    reg_ro!(par_t1_msb, u8);

    pub fn get_par_t1(&mut self) -> Result<u16> {
        match self.par_t1 {
            Some(v) => Ok(v),
            None => {
                let par_t1_lsb = self.read_par_t1_lsb()?;
                let par_t1_msb = self.read_par_t1_msb()?;
                let par_t1 = u16::from_le_bytes([par_t1_lsb, par_t1_msb]);

                self.par_t1 = Some(par_t1);
                Ok(par_t1)
            }
        }
    }

    reg_ro!(par_t2_lsb, u8);
    reg_ro!(par_t2_msb, u8);

    pub fn get_par_t2(&mut self) -> Result<u16> {
        match self.par_t2 {
            Some(v) => Ok(v),
            None => {
                let par_t2_lsb = self.read_par_t2_lsb()?;
                let par_t2_msb = self.read_par_t2_msb()?;
                let par_t2 = u16::from_le_bytes([par_t2_lsb, par_t2_msb]);

                self.par_t2 = Some(par_t2);
                Ok(par_t2)
            }
        }
    }

    reg_ro!(par_t3, u8);

    pub fn get_par_t3(&mut self) -> Result<u8> {
        match self.par_t3 {
            Some(v) => Ok(v),
            None => {
                let par_t3 = self.read_par_t3()?;

                self.par_t3 = Some(par_t3);
                Ok(par_t3)
            }
        }
    }

    reg_ro!(par_p1_lsb, u8);
    reg_ro!(par_p1_msb, u8);

    pub fn get_par_p1(&mut self) -> Result<u16> {
        match self.par_p1 {
            Some(v) => Ok(v),
            None => {
                let par_p1_lsb = self.read_par_p1_lsb()?;
                let par_p1_msb = self.read_par_p1_msb()?;
                let par_p1 = u16::from_le_bytes([par_p1_lsb, par_p1_msb]);

                self.par_p1 = Some(par_p1);
                Ok(par_p1)
            }
        }
    }

    reg_ro!(par_p2_lsb, u8);
    reg_ro!(par_p2_msb, u8);

    pub fn get_par_p2(&mut self) -> Result<u16> {
        match self.par_p2 {
            Some(v) => Ok(v),
            None => {
                let par_p2_lsb = self.read_par_p2_lsb()?;
                let par_p2_msb = self.read_par_p2_msb()?;
                let par_p2 = u16::from_le_bytes([par_p2_lsb, par_p2_msb]);

                self.par_p2 = Some(par_p2);
                Ok(par_p2)
            }
        }
    }

    reg_ro!(par_p3, u8);

    pub fn get_par_p3(&mut self) -> Result<u8> {
        match self.par_p3 {
            Some(v) => Ok(v),
            None => {
                let par_p3 = self.read_par_p3()?;

                self.par_p3 = Some(par_p3);
                Ok(par_p3)
            }
        }
    }

    reg_ro!(par_p4_lsb, u8);
    reg_ro!(par_p4_msb, u8);

    pub fn get_par_p4(&mut self) -> Result<u16> {
        match self.par_p4 {
            Some(v) => Ok(v),
            None => {
                let par_p4_lsb = self.read_par_p4_lsb()?;
                let par_p4_msb = self.read_par_p4_msb()?;
                let par_p4 = u16::from_le_bytes([par_p4_lsb, par_p4_msb]);

                self.par_p4 = Some(par_p4);
                Ok(par_p4)
            }
        }
    }

    reg_ro!(par_p5_lsb, u8);
    reg_ro!(par_p5_msb, u8);

    pub fn get_par_p5(&mut self) -> Result<u16> {
        match self.par_p5 {
            Some(v) => Ok(v),
            None => {
                let par_p5_lsb = self.read_par_p5_lsb()?;
                let par_p5_msb = self.read_par_p5_msb()?;
                let par_p5 = u16::from_le_bytes([par_p5_lsb, par_p5_msb]);

                self.par_p5 = Some(par_p5);
                Ok(par_p5)
            }
        }
    }

    reg_ro!(par_p6, u8);

    pub fn get_par_p6(&mut self) -> Result<u8> {
        match self.par_p6 {
            Some(v) => Ok(v),
            None => {
                let par_p6 = self.read_par_p6()?;

                self.par_p6 = Some(par_p6);
                Ok(par_p6)
            }
        }
    }

    reg_ro!(par_p7, u8);

    pub fn get_par_p7(&mut self) -> Result<u8> {
        match self.par_p7 {
            Some(v) => Ok(v),
            None => {
                let par_p7 = self.read_par_p7()?;

                self.par_p7 = Some(par_p7);
                Ok(par_p7)
            }
        }
    }

    reg_ro!(par_p8_lsb, u8);
    reg_ro!(par_p8_msb, u8);

    pub fn get_par_p8(&mut self) -> Result<u16> {
        match self.par_p8 {
            Some(v) => Ok(v),
            None => {
                let par_p8_lsb = self.read_par_p8_lsb()?;
                let par_p8_msb = self.read_par_p8_msb()?;
                let par_p8 = u16::from_le_bytes([par_p8_lsb, par_p8_msb]);

                self.par_p8 = Some(par_p8);
                Ok(par_p8)
            }
        }
    }

    reg_ro!(par_p9_lsb, u8);
    reg_ro!(par_p9_msb, u8);

    pub fn get_par_p9(&mut self) -> Result<u16> {
        match self.par_p9 {
            Some(v) => Ok(v),
            None => {
                let par_p9_lsb = self.read_par_p9_lsb()?;
                let par_p9_msb = self.read_par_p9_msb()?;
                let par_p9 = u16::from_le_bytes([par_p9_lsb, par_p9_msb]);

                self.par_p9 = Some(par_p9);
                Ok(par_p9)
            }
        }
    }

    reg_ro!(par_p10, u8);

    pub fn get_par_p10(&mut self) -> Result<u8> {
        match self.par_p10 {
            Some(v) => Ok(v),
            None => {
                let par_p10 = self.read_par_p10()?;

                self.par_p10 = Some(par_p10);
                Ok(par_p10)
            }
        }
    }

    reg_ro!(par_h1_lsb, u8);
    reg_ro!(par_h1_msb, u8);

    pub fn get_par_h1(&mut self) -> Result<u16> {
        match self.par_h1 {
            Some(v) => Ok(v),
            None => {
                let par_h1_lsb = self.read_par_h1_lsb()? & 0b1111;
                let par_h1_msb = self.read_par_h1_msb()?;
                let par_h1 = (par_h1_msb as u16) << 4 | par_h1_lsb as u16;

                self.par_h1 = Some(par_h1);
                Ok(par_h1)
            }
        }
    }

    reg_ro!(par_h2_lsb, u8);
    reg_ro!(par_h2_msb, u8);

    pub fn get_par_h2(&mut self) -> Result<u16> {
        match self.par_h2 {
            Some(v) => Ok(v),
            None => {
                let par_h2_lsb = self.read_par_h2_lsb()?;
                let par_h2_msb = self.read_par_h2_msb()?;
                let par_h2 = (par_h2_msb as u16) << 4 | par_h2_lsb as u16;

                self.par_h2 = Some(par_h2);
                Ok(par_h2)
            }
        }
    }

    reg_ro!(par_h3, u8);

    pub fn get_par_h3(&mut self) -> Result<u8> {
        match self.par_h3 {
            Some(v) => Ok(v),
            None => {
                let par_h3 = self.read_par_h3()?;

                self.par_h3 = Some(par_h3);
                Ok(par_h3)
            }
        }
    }

    reg_ro!(par_h4, u8);

    pub fn get_par_h4(&mut self) -> Result<u8> {
        match self.par_h4 {
            Some(v) => Ok(v),
            None => {
                let par_h4 = self.read_par_h4()?;

                self.par_h4 = Some(par_h4);
                Ok(par_h4)
            }
        }
    }

    reg_ro!(par_h5, u8);

    pub fn get_par_h5(&mut self) -> Result<u8> {
        match self.par_h5 {
            Some(v) => Ok(v),
            None => {
                let par_h5 = self.read_par_h5()?;

                self.par_h5 = Some(par_h5);
                Ok(par_h5)
            }
        }
    }

    reg_ro!(par_h6, u8);

    pub fn get_par_h6(&mut self) -> Result<u8> {
        match self.par_h6 {
            Some(v) => Ok(v),
            None => {
                let par_h6 = self.read_par_h6()?;

                self.par_h6 = Some(par_h6);
                Ok(par_h6)
            }
        }
    }

    reg_ro!(par_h7, u8);

    pub fn get_par_h7(&mut self) -> Result<u8> {
        match self.par_h7 {
            Some(v) => Ok(v),
            None => {
                let par_h7 = self.read_par_h7()?;

                self.par_h7 = Some(par_h7);
                Ok(par_h7)
            }
        }
    }

    reg_ro!(par_g1, u8);

    pub fn get_par_g1(&mut self) -> Result<u8> {
        match self.par_g1 {
            Some(v) => Ok(v),
            None => {
                let par_g1 = self.read_par_g1()?;

                self.par_g1 = Some(par_g1);
                Ok(par_g1)
            }
        }
    }

    reg_ro!(par_g2_lsb, u8);
    reg_ro!(par_g2_msb, u8);

    pub fn get_par_g2(&mut self) -> Result<u16> {
        match self.par_g2 {
            Some(v) => Ok(v),
            None => {
                let par_g2_lsb = self.read_par_g2_lsb()?;
                let par_g2_msb = self.read_par_g2_msb()?;
                let par_g2 = u16::from_le_bytes([par_g2_lsb, par_g2_msb]);

                self.par_g2 = Some(par_g2);
                Ok(par_g2)
            }
        }
    }

    reg_ro!(par_g3, u8);

    pub fn get_par_g3(&mut self) -> Result<u8> {
        match self.par_g3 {
            Some(v) => Ok(v),
            None => {
                let par_g3 = self.read_par_g3()?;

                self.par_g3 = Some(par_g3);
                Ok(par_g3)
            }
        }
    }

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
