use crate::meteo_chart::forecast_renderer::meteo_forecast_renderer_helper::MeteoForecastRendererHelper;
use crate::meteo_chart::meteo_layer::meteo_vertical_wind_layer::MeteoVerticalWindLayer;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::metobin::meteobin_error::MeteoBinError;
use crate::metobin::meteobin_type::MeteobinType;
use log::info;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};


pub struct VerticalWindMeteobin {}


impl VerticalWindMeteobin {
    const MISSING_VALUE: u8 = 255;


    pub fn create_meteobin_file(
        layer: &MeteoVerticalWindLayer,
        fc_run: &dyn MeteoForecastRun,
        fc_step: usize,
    ) -> Result<(), MeteoBinError> {
        let bin_data = Self::create_bin_values(layer);
        let path = MeteoForecastRendererHelper::get_output_path(fc_run, fc_step, layer.get_type());
        let filename = format!(
            "{}{}",
            &path,
            MeteobinType::VerticalWind.get_output_file()
        );

        info!("writing vertical wind meteobin file {}", &filename);

        fs::create_dir_all(&path)?;
        let mut file = BufWriter::new(File::create(&filename)
            .expect("Unable to create vertical wind meteobin file"));
        let _ = file.write_all(&bin_data)?;

        Ok(())
    }


    fn create_bin_values(layer: &MeteoVerticalWindLayer) -> Vec<u8> {
        let (dim_x, dim_y, dim_level) = layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim_y {
            for x in 0..dim_x {
                for level in 0..dim_level {
                    let hhl_value = layer.get_hhl_value(x, y, level);
                    if hhl_value.is_some() {
                        out_values.push(hhl_value.unwrap());
                    } else {
                        out_values.push(Self::MISSING_VALUE);
                    }

                    let u_v_values = layer.get_u_v_values(x, y, level);
                    if u_v_values.is_some() {
                        out_values.push(u_v_values.unwrap().0);
                        out_values.push(u_v_values.unwrap().1);
                    } else {
                        out_values.push(Self::MISSING_VALUE);
                        out_values.push(Self::MISSING_VALUE);
                    }
                }
            }
        }

        out_values
    }
}
