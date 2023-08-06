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
            Some(val) => ((val - Self::KELVIN_OFFSET) * 2.0 + 128.0) as u8,
            None => Self::NONE_BIN_VALUE
        };
    }
}
