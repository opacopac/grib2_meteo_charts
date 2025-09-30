use crate::meteo_common::meteo_forecast_renderer_helper::MeteoForecastFileHelper;
use crate::meteo_chart::meteo_layer::weather_interpretation::WeatherInterpretation;
use crate::meteo_chart::meteo_layer::weather_layer::WeatherLayer;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::metobin::meteobin_type::MeteobinType;
use crate::physics::length::Length;
use log::info;
use std::fs::File;
use std::io::{BufWriter, Write};


pub struct WeatherMeteoBin {}


impl WeatherMeteoBin {
    const NONE_BIN_VALUE: u8 = 0xFF;


    pub fn create_meteobin_file(
        layer: &WeatherLayer,
        fc_run: &dyn MeteoForecastRun,
        fc_step: usize,
    ) {
        let bin_data = Self::create_bin_values(layer);
        let filename = format!(
            "{}{}",
            MeteoForecastFileHelper::get_output_path(fc_run, fc_step, layer.get_type()),
            MeteobinType::Weather.get_output_file()
        );

        info!("writing weather meteobin file {}", &filename);

        let mut file = BufWriter::new(File::create(&filename).expect("Unable to create weather meteobin file"));
        let _ = file.write_all(&bin_data);
    }


    fn create_bin_values(layer: &WeatherLayer) -> Vec<u8> {
        let dim = layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim.1 {
            for x in 0..dim.0 {
                let result_ww = layer.get_ww_by_xy(x, y);
                let result_clct = layer.get_clct_by_xy(x, y);
                let result_ceiling = layer.get_ceiling_by_xy(x, y);
                let out_val_ww = Self::calc_ww_value(result_ww, result_clct);
                let out_val_ceiling = Self::calc_ceiling_100ft_value(result_ceiling);

                out_values.push(out_val_ww);
                out_values.push(out_val_ceiling);
            }
        }

        out_values
    }


    fn calc_ww_value(
        value_ww: Option<WeatherInterpretation>,
        value_clct: Option<f32>,
    ) -> u8 {
        // check if ww values is present and > 4 then return it directly, otherwise calculate from clct
        match value_ww {
            Some(val_ww) if val_ww.to_value() > 4 => val_ww.to_value(),
            _ => {
                let derived_ww = Self::calc_ww_value_from_clct(value_clct);
                match derived_ww {
                    Some(val_clct) => val_clct.to_value(),
                    None => Self::NONE_BIN_VALUE
                }
            }
        }
    }


    fn calc_ww_value_from_clct(
        value_clct: Option<f32>
    ) -> Option<WeatherInterpretation> {
        // source: https://www.dwd.de/DE/forschung/wettervorhersage/num_modellierung/01_num_vorhersagemodelle/01c_wetterinterpretation/wetter_interpretation.pdf?__blob=publicationFile&v=6
        // chapter 5.7.2 Kein signifikantes Wetter
        match value_clct {
            None => None,
            Some(val) if val <= 6.25 / 100.0 => Some(WeatherInterpretation::ClearSky),
            Some(val) if val <= 43.75 / 100.0 => Some(WeatherInterpretation::MainlyClearSky),
            Some(val) if val <= 81.25 / 100.0 => Some(WeatherInterpretation::PartlyCloudy),
            Some(_) => Some(WeatherInterpretation::Overcast)
        }
    }


    fn calc_ceiling_100ft_value(value_m: Option<f32>) -> u8 {
        match value_m {
            None => Self::NONE_BIN_VALUE,
            Some(val) if (Length::from_meters_to_feet(val) / 200.0).round() >= 255.0 => Self::NONE_BIN_VALUE,
            Some(val) => (Length::from_meters_to_feet(val) / 200.0).round().min(254.0).max(0.0) as u8
        }
    }
}
