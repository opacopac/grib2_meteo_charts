use crate::meteo_dwd::dwd_wind_layer::DwdWindLayer;

pub struct WindMeteobin {
    wind_layer: DwdWindLayer
}


impl WindMeteobin {
    const KNOTS_PER_MPS: f32 = 1.94384;
    const NONE_BIN_VALUE: u16 = 0xFFFF;


    pub fn new(wind_layer: DwdWindLayer) -> WindMeteobin {
        return WindMeteobin { wind_layer };
    }


    pub fn create_bin_values(&self) -> Vec<u16> {
        let dim = self.wind_layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim.1 {
            for x in 0..dim.0 {
                let val = self.wind_layer.get_wind_speed_dir_by_xy(x, y);
                let out_val = Self::calc_bin_value(val);
                out_values.push(out_val);
            }
        }

        return out_values;
    }


    // format: 7bit speed, 9bit dir
    fn calc_bin_value(value_speed_dir: Option<(f32, f32)>) -> u16 {
        return match value_speed_dir {
            Some(speed_dir) => {
                let speed_kt = (speed_dir.0 * Self::KNOTS_PER_MPS).round().max(0.0).min(127.0) as u16;
                let dir_deg = speed_dir.1.round().max(0.0).min(359.0) as u16;

                speed_kt << 9 | dir_deg
            }
            None => WindMeteobin::NONE_BIN_VALUE
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::metobin::wind_metobin::WindMeteobin;

    #[test]
    fn it_calculates_the_bin_value_for_5kt_270deg() {
        let in_value = (5.0 / WindMeteobin::KNOTS_PER_MPS, 270.0);
        let result = WindMeteobin::calc_bin_value(Some(in_value));

        assert_eq!(0b00001011_00001110, result);
    }


    #[test]
    fn it_limits_the_max_bin_value_to_127kt_359deg() {
        let in_value = (150.0 / WindMeteobin::KNOTS_PER_MPS, 400.0);
        let result = WindMeteobin::calc_bin_value(Some(in_value));

        assert_eq!(0b11111111_01100111, result);
    }


    #[test]
    fn it_limits_the_min_bin_value_to_0kt_0deg() {
        let in_value = (-15.0 / WindMeteobin::KNOTS_PER_MPS, -90.0);
        let result = WindMeteobin::calc_bin_value(Some(in_value));

        assert_eq!(0b00000000_00000000, result);
    }


    #[test]
    fn it_calculates_the_bin_value_for_none() {
        let result = WindMeteobin::calc_bin_value(None);

        assert_eq!(0b11111111_11111111, result);
    }
}
