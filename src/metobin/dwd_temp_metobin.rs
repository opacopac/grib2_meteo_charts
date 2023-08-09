use crate::dwd_layer::dwd_temp_layer::DwdTempLayer;

pub struct DwdTempMeteoBin {
    layer: DwdTempLayer
}


impl DwdTempMeteoBin {
    const NONE_BIN_VALUE: u8 = 0xFF;
    const KELVIN_OFFSET: f32 = 273.15;


    pub fn new(temp_layer: DwdTempLayer) -> DwdTempMeteoBin {
        return DwdTempMeteoBin { layer: temp_layer };
    }


    pub fn create_bin_values(&self) -> Vec<u8> {
        let dim = self.layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim.1 {
            for x in 0..dim.0 {
                let result_temp = self.layer.get_temp_by_xy(x, y);
                let out_val_temp = Self::calc_temp_value(result_temp);

                out_values.push(out_val_temp);
            }
        }

        return out_values;
    }


    fn calc_temp_value(value_temp: Option<f32>) -> u8 {
        return match value_temp {
            Some(val) => (((val - Self::KELVIN_OFFSET) * 2.0).round() + 128.0) as u8,
            None => Self::NONE_BIN_VALUE
        };
    }
}


#[cfg(test)]
mod tests {
    use crate::metobin::dwd_temp_metobin::DwdTempMeteoBin;

    #[test]
    fn it_calculates_the_correct_precip_meteobin_values() {
        assert_eq!(128, DwdTempMeteoBin::calc_temp_value(Some(273.15))); // 0°C
        assert_eq!(130, DwdTempMeteoBin::calc_temp_value(Some(274.15))); // 1°C
        assert_eq!(126, DwdTempMeteoBin::calc_temp_value(Some(272.15))); // -1°C
        assert_eq!(129, DwdTempMeteoBin::calc_temp_value(Some(273.65))); // 0.5°C
        assert_eq!(127, DwdTempMeteoBin::calc_temp_value(Some(272.65))); // -0.5°C
        assert_eq!(211, DwdTempMeteoBin::calc_temp_value(Some(314.65))); // 41.5°C
        assert_eq!(68, DwdTempMeteoBin::calc_temp_value(Some(243.4))); // -29.75°C
        assert_eq!(254, DwdTempMeteoBin::calc_temp_value(Some(336.15))); // 63°C
        assert_eq!(0, DwdTempMeteoBin::calc_temp_value(Some(209.15))); // -64°C
        assert_eq!(0xFF, DwdTempMeteoBin::calc_temp_value(None));
    }
}
