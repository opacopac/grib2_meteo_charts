use crate::meteo_dwd::dwd_weather_layer::DwdWeatherLayer;
use crate::meteo_dwd::weather_interpretation::WeatherInterpretation;

pub struct WeatherMeteoBin {
    layer: DwdWeatherLayer
}


impl WeatherMeteoBin {
    const NONE_BIN_VALUE: u8 = 0xFF;
    const FEET_PER_MP: f32 = 3.28084;


    pub fn new(weather_layer: DwdWeatherLayer) -> WeatherMeteoBin {
        return WeatherMeteoBin { layer: weather_layer };
    }


    pub fn create_bin_values(&self) -> Vec<u8> {
        let dim = self.layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim.1 {
            for x in 0..dim.0 {
                let result_ww = self.layer.get_ww_by_xy(x, y);
                let result_ceiling = self.layer.get_ceiling_by_xy(x, y);
                let out_val_ww = Self::calc_ww_value(result_ww);
                let out_val_ceiling = Self::calc_ceiling_100ft_value(result_ceiling);

                out_values.push(out_val_ww);
                out_values.push(out_val_ceiling);
            }
        }

        return out_values;
    }


    fn calc_ww_value(value_ww: Option<WeatherInterpretation>) -> u8 {
        return match value_ww {
            Some(val_ww) => val_ww.to_value(),
            None => Self::NONE_BIN_VALUE
        };
    }


    fn calc_ceiling_100ft_value(value_m: Option<f32>) -> u8 {
        return match value_m {
            None => Self::NONE_BIN_VALUE,
            Some(val) if (val * Self::FEET_PER_MP / 200.0).round() >= 255.0 => Self::NONE_BIN_VALUE,
            Some(val) => (val * Self::FEET_PER_MP / 200.0).round().min(254.0).max(0.0) as u8
        }
    }
}
