use crate::meteo_chart::meteo_layer::meteo_temp_2m_layer::MeteoTemp2mLayer;
use crate::meteo_common::meteo_forecast_renderer_helper::MeteoForecastFileHelper;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::metobin::meteobin_type::MeteobinType;
use crate::physics::temperature::Temperature;
use log::info;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct TempMeteoBin {}


impl TempMeteoBin {
    const NONE_BIN_VALUE: u8 = 0xFF;


    pub fn create_meteobin_file(
        layer: &MeteoTemp2mLayer,
        fc_run: &MeteoForecastRun2,
        fc_step: usize,
    ) {
        let bin_data = Self::create_bin_values(layer);
        let filename = format!(
            "{}{}",
            MeteoForecastFileHelper::get_output_path2(fc_run, fc_step, layer.get_type()),
            MeteobinType::Temp2m.get_output_file()
        );

        info!("writing temp meteobin file {}", &filename);

        let mut file = BufWriter::new(File::create(&filename).expect("Unable to create temp meteobin file"));
        let _ = file.write_all(&bin_data);
    }


    fn create_bin_values(layer: &MeteoTemp2mLayer) -> Vec<u8> {
        let dim = layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim.1 {
            for x in 0..dim.0 {
                let result_temp = layer.get_temp_by_xy(x, y);
                let out_val_temp = Self::calc_temp_value(result_temp);

                out_values.push(out_val_temp);
            }
        }

        out_values
    }


    fn calc_temp_value(value_temp: Option<f32>) -> u8 {
        match value_temp {
            Some(val) => ((Temperature::from_kelvin_to_celsius(val) * 2.0).round() + 128.0) as u8,
            None => Self::NONE_BIN_VALUE
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::metobin::temp_metobin::TempMeteoBin;

    #[test]
    fn it_calculates_the_correct_precip_meteobin_values() {
        assert_eq!(128, TempMeteoBin::calc_temp_value(Some(273.15))); // 0°C
        assert_eq!(130, TempMeteoBin::calc_temp_value(Some(274.15))); // 1°C
        assert_eq!(126, TempMeteoBin::calc_temp_value(Some(272.15))); // -1°C
        assert_eq!(129, TempMeteoBin::calc_temp_value(Some(273.65))); // 0.5°C
        assert_eq!(127, TempMeteoBin::calc_temp_value(Some(272.65))); // -0.5°C
        assert_eq!(211, TempMeteoBin::calc_temp_value(Some(314.65))); // 41.5°C
        assert_eq!(68, TempMeteoBin::calc_temp_value(Some(243.4))); // -29.75°C
        assert_eq!(254, TempMeteoBin::calc_temp_value(Some(336.15))); // 63°C
        assert_eq!(0, TempMeteoBin::calc_temp_value(Some(209.15))); // -64°C
        assert_eq!(0xFF, TempMeteoBin::calc_temp_value(None));
    }
}
