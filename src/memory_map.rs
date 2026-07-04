use crate::Layer;

macro_rules! map {
    ($name:ident, $addr:expr) => {
        pub(crate) struct $name;

        impl $name {
            pub(crate) const fn get_addr() -> u8 {
                $addr
            }
            pub(crate) fn read<L: Layer>(transport_layer: &mut L) -> crate::Result<u8> {
                transport_layer.read_byte(Self::get_addr())
            }
            // TODO: This should be conditional. Not all registers are writable.
            pub(crate) fn write<L: Layer>(transport_layer: &mut L, data: u8) -> crate::Result<()> {
                transport_layer.write_byte(Self::get_addr(), data)
            }
        }
    };
}

// TODO: identify duplicate addresses and alias when appropriate

map!(status, 0x73);
map!(variant_id, 0xF0);
map!(reset, 0xE0);
map!(chip_id, 0xD0);
map!(config, 0x75);
map!(ctrl_meas, 0x74);
map!(ctrl_hum, 0x72);
map!(ctrl_gas_1, 0x71);
map!(ctrl_gas_0, 0x70);
map!(gas_wait_shared, 0x6E);

// TODO: Split these into 9 separate registers each.
// map!(gas_wait_x, 0x6D, -10);
// map!(res_heat_x, 0x63, -10);
// map!(idac_heat_x, 0x59, -10);

map!(gas_r_lsb_2, 0x4F);
map!(gas_r_msb_2, 0x48);
map!(hum_lsb_2, 0x47);
map!(hum_msb_2, 0x46);
map!(temp_xlsb_2, 0x46);
map!(temp_lsb_2, 0x45);
map!(temp_msb_2, 0x44);
map!(press_xlsb_2, 0x43);
map!(press_lsb_2, 0x42);
map!(press_msb_2, 0x41);
map!(sub_meas_index_2, 0x40);
map!(meas_status_2, 0x3F);
map!(gas_r_lsb_1, 0x3E);
map!(gas_r_msb_1, 0x3D);
map!(hum_lsb_1, 0x37);
map!(hum_msb_1, 0x36);
map!(temp_xlsb_1, 0x35);
map!(temp_lsb_1, 0x34);
map!(temp_msb_1, 0x33);
map!(press_xlsb_1, 0x32);
map!(press_lsb_1, 0x31);
map!(press_msb_1, 0x30);
map!(sub_meas_index_1, 0x2F);
map!(meas_status_1, 0x2E);
map!(gas_r_lsb_0, 0x2D); // the datasheet lists this register as 'gas_r_lsb [2]', but it's probably wrong
map!(gas_r_msb_0, 0x2C);
map!(hum_lsb_0, 0x26);
map!(hum_msb_0, 0x25);
map!(temp_xlsb_0, 0x24);
map!(temp_lsb_0, 0x23);
map!(temp_msb_0, 0x22);
map!(press_xlsb_0, 0x21);
map!(press_lsb_0, 0x20);
map!(press_msb_0, 0x1F);
map!(sub_meas_index_0, 0x1E);
map!(meas_status_0, 0x1D);
map!(par_t1_lsb, 0xE9);
map!(par_t1_msb, 0xEA);
map!(par_t2_lsb, 0x8A);
map!(par_t2_msb, 0x8B);
map!(par_t3, 0x8C);
map!(par_p1_lsb, 0x8E);
map!(par_p1_msb, 0x8F);
map!(par_p2_lsb, 0x90);
map!(par_p2_msb, 0x91);
map!(par_p3, 0x92);
map!(par_p4_lsb, 0x94);
map!(par_p4_msb, 0x95);
map!(par_p5_lsb, 0x96);
map!(par_p5_msb, 0x97);
map!(par_p6, 0x99);
map!(par_p7, 0x98);
map!(par_p8_lsb, 0x9C);
map!(par_p8_msb, 0x9D);
map!(par_p9_lsb, 0x9E);
map!(par_p9_msb, 0x9F);
map!(par_p10, 0xA0);
map!(par_h1_lsb, 0xE2); // only bits 3:0
map!(par_h1_msb, 0xE3);
map!(par_h2_lsb, 0xE2); // only bits 7:4
map!(par_h2_msb, 0xE1);
map!(par_h3, 0xE4);
map!(par_h4, 0xE5);
map!(par_h5, 0xE6);
map!(par_h6, 0xE7);
map!(par_h7, 0xE8);
map!(par_g1, 0xED);
map!(par_g2_lsb, 0xEB);
map!(par_g2_msb, 0xEC);
map!(par_g3, 0xEE);
map!(temp_adc_0_lsb, 0x24); // only bits 7:4
map!(temp_adc_0_msb, 0x23);
map!(temp_adc_0_xlsb, 0x22);
map!(temp_adc_1_lsb, 0x35); // only bits 7:4
map!(temp_adc_1_msb, 0x34);
map!(temp_adc_1_xlsb, 0x33);
map!(temp_adc_2_lsb, 0x46); // only bits 7:4
map!(temp_adc_2_msb, 0x45);
map!(temp_adc_2_xlsb, 0x44);
map!(press_adc_0_lsb, 0x21); // only bits 7:4
map!(press_adc_0_msb, 0x20);
map!(press_adc_0_xlsb, 0x1F);
map!(press_adc_1_lsb, 0x32); // only bits 7:4
map!(press_adc_1_msb, 0x31);
map!(press_adc_1_xlsb, 0x30);
map!(press_adc_2_lsb, 0x43); // only bits 7:4
map!(press_adc_2_msb, 0x42);
map!(press_adc_2_xlsb, 0x41);
map!(hum_adc_0_lsb, 0x26);
map!(hum_adc_0_msb, 0x25);
map!(hum_adc_1_lsb, 0x37);
map!(hum_adc_1_msb, 0x36);
map!(hum_adc_2_lsb, 0x48);
map!(hum_adc_2_msb, 0x47);
map!(res_heat_range, 0x02); // only bits 5:4
map!(res_heat_val, 0x00);
map!(gas_adc_0_lsb, 0x2D); // only bits 7:6
map!(gas_adc_0_msb, 0x2C);
map!(gas_adc_1_lsb, 0x3E); // only bits 7:6
map!(gas_adc_1_msb, 0x3D);
map!(gas_adc_2_lsb, 0x4F); // only bits 7:6
map!(gas_adc_2_msb, 0x4E);
map!(gas_range_0, 0x2D); // only bits 3:0
map!(gas_range_1, 0x3E); // only bits 3:0
map!(gas_range_2, 0x4F); // only bits 3:0
