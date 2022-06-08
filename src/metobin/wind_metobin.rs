use crate::meteo_dwd::dwd_wind_layer::DwdWindLayer;

pub struct WindMeteobin {
    wind_layer: DwdWindLayer
}


impl WindMeteobin {
    const KNOTS_PER_MPS: f32 = 1.94384;
    const NONE_BIN_VALUE: u8 = 0xFF;


    pub fn new(wind_layer: DwdWindLayer) -> WindMeteobin {
        return WindMeteobin { wind_layer };
    }


    pub fn create_bin_values(&self) -> Vec<u8> {
        let dim = self.wind_layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim.1 {
            for x in 0..dim.0 {
                let result = self.wind_layer.get_wind_speed_e_n_by_xy(x, y);
                let out_val = match result {
                    Some(val_e_n) => (
                        Self::calc_speed_kt_value(val_e_n.0),
                        Self::calc_speed_kt_value(val_e_n.1)
                    ),
                    None => (Self::NONE_BIN_VALUE, Self::NONE_BIN_VALUE)
                };
                out_values.push(out_val.0);
                out_values.push(out_val.1);
            }
        }

        return out_values;
    }


    fn calc_speed_kt_value(value_mps: f32) -> u8 {
        return (value_mps * Self::KNOTS_PER_MPS + 128.0).round().min(254.0).max(0.0) as u8;
    }
}


#[cfg(test)]
mod tests {
    use crate::metobin::wind_metobin::WindMeteobin;

    #[test]
    fn it_calculates_the_bin_value_for_3kt() {
        let in_value = 3.0 / WindMeteobin::KNOTS_PER_MPS;
        let result = WindMeteobin::calc_speed_kt_value(in_value);

        assert_eq!(3 + 128, result);
    }


    #[test]
    fn it_limits_the_max_bin_value_to_plus127() {
        let in_value = 150.0 / WindMeteobin::KNOTS_PER_MPS;
        let result = WindMeteobin::calc_speed_kt_value(in_value);

        assert_eq!(254, result);
    }


    #[test]
    fn it_limits_the_min_bin_value_to_neg128() {
        let in_value = -200.0 / WindMeteobin::KNOTS_PER_MPS;
        let result = WindMeteobin::calc_speed_kt_value(in_value);

        assert_eq!(0 as u8, result);
    }
}
