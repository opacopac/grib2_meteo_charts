use crate::meteo_chart::meteo_layer::meteo_vertical_cloud_layer::MeteoVerticalCloudLayer;
use crate::meteo_common::meteo_forecast_renderer_helper::MeteoForecastFileHelper;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::metobin::meteobin_error::MeteoBinError;
use crate::metobin::meteobin_type::MeteobinType;
use log::info;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};


pub struct VerticalCloudMeteobin {}


impl VerticalCloudMeteobin {
    const MISSING_VALUE: u8 = 255;


    pub fn create_meteobin_file(
        layer: &MeteoVerticalCloudLayer,
        fc_run: &dyn MeteoForecastRun,
        fc_step: usize,
    ) -> Result<(), MeteoBinError> {
        let bin_data = Self::create_bin_values(layer);
        let path = MeteoForecastFileHelper::get_output_path(fc_run, fc_step, layer.get_type());
        let filename = format!(
            "{}{}",
            &path,
            MeteobinType::VerticalClouds.get_output_file()
        );

        info!("writing vertical cloud meteobin file {}", &filename);

        fs::create_dir_all(&path)?;
        let mut file = BufWriter::new(File::create(&filename)
            .expect("Unable to create vertical cloud meteobin file"));
        let _ = file.write_all(&bin_data)?;

        Ok(())
    }


    pub fn create_meteobin_file2(
        layer: &MeteoVerticalCloudLayer,
        fc_run: &MeteoForecastRun2,
        fc_step: usize,
    ) -> Result<(), MeteoBinError> {
        let bin_data = Self::create_bin_values(layer);
        let path = MeteoForecastFileHelper::get_output_path2(fc_run, fc_step, layer.get_type());
        let filename = format!(
            "{}{}",
            &path,
            MeteobinType::VerticalClouds.get_output_file()
        );

        info!("writing vertical cloud meteobin file {}", &filename);

        fs::create_dir_all(&path)?;
        let mut file = BufWriter::new(File::create(&filename)
            .expect("Unable to create vertical cloud meteobin file"));
        let _ = file.write_all(&bin_data)?;

        Ok(())
    }


    fn create_bin_values(layer: &MeteoVerticalCloudLayer) -> Vec<u8> {
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

                    let clc_value = layer.get_clc_value(x, y, level);
                    if clc_value.is_some() {
                        out_values.push(clc_value.unwrap());
                    } else {
                        out_values.push(Self::MISSING_VALUE);
                    }
                }
            }
        }

        out_values
    }
}
