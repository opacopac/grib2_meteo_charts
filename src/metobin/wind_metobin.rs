use crate::meteo_layer::meteo_wind_layer::MeteoWindLayer;
use crate::physics::speed::Speed;

pub struct WindMeteobin {}


impl WindMeteobin {
    const NONE_BIN_VALUE: u8 = 0xFF;


    pub fn create_bin_values(wind_layer: &MeteoWindLayer) -> Vec<u8> {
        let dim = wind_layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim.1 {
            for x in 0..dim.0 {
                let wind_result = wind_layer.get_wind_speed_e_n_by_xy(x, y);
                let gust_result = wind_layer.get_gusts_by_xy(x, y);
                let out_val = match wind_result {
                    Some(val_e_n) => (
                        Self::calc_speed_kt_value(val_e_n.0),
                        Self::calc_speed_kt_value(val_e_n.1),
                        Self::calc_gust_kt_value(gust_result)
                    ),
                    None => (Self::NONE_BIN_VALUE, Self::NONE_BIN_VALUE, Self::NONE_BIN_VALUE)
                };

                out_values.push(out_val.0);
                out_values.push(out_val.1);
                out_values.push(out_val.2);
            }
        }

        out_values
    }


    fn calc_speed_kt_value(value_mps: f32) -> u8 {
        (Speed::from_mps_to_knots(value_mps) + 128.0).round().min(254.0).max(0.0) as u8
    }


    fn calc_gust_kt_value(value_mps: Option<f32>) -> u8 {
        match value_mps {
            None => Self::NONE_BIN_VALUE,
            Some(val_mps) => Speed::from_mps_to_knots(val_mps).round().min(254.0).max(0.0) as u8
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::metobin::wind_metobin::WindMeteobin;
    use crate::physics::speed::Speed;


    #[test]
    fn it_calculates_the_bin_value_for_3kt() {
        let in_value = Speed::from_knots_to_mps(3.0);
        let result = WindMeteobin::calc_speed_kt_value(in_value);

        assert_eq!(3 + 128, result);
    }


    #[test]
    fn it_limits_the_max_bin_value_to_plus127() {
        let in_value = Speed::from_knots_to_mps(150.0);
        let result = WindMeteobin::calc_speed_kt_value(in_value);

        assert_eq!(254, result);
    }


    #[test]
    fn it_limits_the_min_bin_value_to_neg128() {
        let in_value = Speed::from_knots_to_mps(-200.0);
        let result = WindMeteobin::calc_speed_kt_value(in_value);

        assert_eq!(0 as u8, result);
    }
}
