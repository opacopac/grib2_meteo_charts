use crate::meteo_chart::forecast_renderer::meteo_forecast_renderer_helper::MeteoForecastRendererHelper;
use crate::meteo_chart::meteo_layer::meteo_cloud_precip_layer::MeteoCloudPrecipLayer;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::metobin::meteobin_type::MeteobinType;
use log::info;
use std::fs::File;
use std::io::{BufWriter, Write};


pub struct PrecipMeteoBin {}


impl PrecipMeteoBin {
    const NONE_BIN_VALUE: u8 = 0xFF;


    pub fn create_meteobin_file(
        layer: &MeteoCloudPrecipLayer,
        fc_run: &dyn MeteoForecastRun,
        fc_step: usize,
    ) {
        let bin_data = Self::create_bin_values(layer);
        let filename = format!(
            "{}{}",
            MeteoForecastRendererHelper::get_output_path(fc_run, fc_step, layer.get_type()),
            MeteobinType::Precip.get_output_file()
        );

        info!("writing precip meteobin file {}", &filename);

        let mut file = BufWriter::new(File::create(&filename).expect("Unable to create precip meteobin file"));
        let _ = file.write_all(&bin_data);
    }


    pub fn create_bin_values(layer: &MeteoCloudPrecipLayer) -> Vec<u8> {
        let dim = layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim.1 {
            for x in 0..dim.0 {
                let result_cloud_precip = layer.get_cloud_and_precip_by_xy(x, y);
                let out_val_precip = Self::calc_precip_value(result_cloud_precip);

                out_values.push(out_val_precip);
            }
        }

        out_values
    }


    fn calc_precip_value(value_cloud_precip: Option<(f32, f32)>) -> u8 {
        match value_cloud_precip {
            Some(val) => {
                if val.1 >= 0.2 && val.1 < 0.5 {
                    1 // 0.5 * 2
                } else {
                    (val.1 * 2.0).round() as u8
                }
            }
            None => Self::NONE_BIN_VALUE
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::metobin::precip_metobin::PrecipMeteoBin;

    #[test]
    fn it_calculates_the_correct_precip_meteobin_values() {
        assert_eq!(0, PrecipMeteoBin::calc_precip_value(Some((0.0, 0.0))));
        assert_eq!(2, PrecipMeteoBin::calc_precip_value(Some((0.0, 1.0))));
        assert_eq!(1, PrecipMeteoBin::calc_precip_value(Some((0.0, 0.2))));
        assert_eq!(1, PrecipMeteoBin::calc_precip_value(Some((0.0, 0.5))));
        assert_eq!(1, PrecipMeteoBin::calc_precip_value(Some((0.0, 0.74))));
        assert_eq!(2, PrecipMeteoBin::calc_precip_value(Some((0.0, 0.75))));
        assert_eq!(72, PrecipMeteoBin::calc_precip_value(Some((0.0, 35.8))));
        assert_eq!(254, PrecipMeteoBin::calc_precip_value(Some((0.0, 127.0))));
        assert_eq!(0xFF, PrecipMeteoBin::calc_precip_value(None));
    }
}
