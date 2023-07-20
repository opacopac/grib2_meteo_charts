use std::fs::File;
use std::io::{BufWriter, Write};

use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::chart::cloud_precip_chart_renderer::CloudPrecipChartRenderer;
use crate::dwd::dwd_file_reader::icon_d2_ceiling_reader::IconD2CeilingReader;
use crate::dwd::dwd_file_reader::icon_d2_clct_mod_reader::IconD2ClctModReader;
use crate::dwd::dwd_file_reader::icon_d2_tot_prec_reader::IconD2TotPrecReader;
use crate::dwd::dwd_file_reader::icon_d2_ww_reader::IconD2WwReader;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd_forecast_renderer::icon_d2_forecast_renderer_helper::IconD2ForecastRendererHelper;
use crate::dwd_layer::dwd_cloud_precip_layer::DwdCloudPrecipLayer;
use crate::dwd_layer::dwd_weather_layer::DwdWeatherLayer;
use crate::imaging::drawable::Drawable;
use crate::metobin::dwd_precip_metobin::DwdPrecipMeteoBin;
use crate::metobin::dwd_weather_metobin::DwdWeatherMeteoBin;

pub struct IconD2CloudPrecipRenderer;


const WEATHER_LAYER: &str = "clct_precip";


impl IconD2CloudPrecipRenderer {
    pub fn create(forecast_run: &DwdForecastRun) -> Result<(), ForecastRendererError> {
        DwdForecastStep::get_step_range()
            .into_par_iter()
            .try_for_each(|step| {
                info!("creating weather charts, time step {}", step);

                let fc_step = DwdForecastStep::new_from_run(forecast_run, step);
                let fc_previous_step = DwdForecastStep::new_from_run(forecast_run, step - 1);

                // map tiles
                let clct_grid = IconD2ClctModReader::read_grid_from_file(&fc_step)
                    .map_err(|err| ForecastRendererError::ReadGridFromClctFileError(err))?;
                let precip_grid0 = IconD2TotPrecReader::read_grid_from_file(&fc_previous_step)
                    .map_err(|err| ForecastRendererError::ReadGridFromPrecipFileError(err))?;
                let precip_grid1 = IconD2TotPrecReader::read_grid_from_file(&fc_step)
                    .map_err(|err| ForecastRendererError::ReadGridFromPrecipFileError(err))?;

                let layer = DwdCloudPrecipLayer::new(clct_grid, precip_grid0, precip_grid1)?;

                let _ = CloudPrecipChartRenderer::render_map_tiles(
                    &layer,
                    (0, 7),
                    |tile: &Drawable, zoom: u32, x: u32, y: u32| IconD2ForecastRendererHelper::save_tile_step(tile, zoom, x, y, WEATHER_LAYER, &fc_step)
                );

                // precip meteobin
                let precip_bin = DwdPrecipMeteoBin::new(layer);
                let data = precip_bin.create_bin_values();
                let filename = format!(
                    "{}PRECIP_D2.meteobin",
                    IconD2ForecastRendererHelper::get_output_path(&fc_step, WEATHER_LAYER),
                );
                let mut file = BufWriter::new(File::create(&filename).expect("Unable to create file"));
                let _ = file.write_all(&data);


                // ww meteobin
                let ww_grid = IconD2WwReader::read_grid_from_file(&fc_step)?;
                let ceiling_grid = IconD2CeilingReader::read_grid_from_file(&fc_step)?;

                let weather_layer = DwdWeatherLayer::new(ww_grid, ceiling_grid)?;
                let weather_bin = DwdWeatherMeteoBin::new(weather_layer);
                let data = weather_bin.create_bin_values();
                let filename = format!(
                    "{}WW_D2.meteobin",
                    IconD2ForecastRendererHelper::get_output_path(&fc_step, WEATHER_LAYER),
                );
                let mut file = BufWriter::new(File::create(&filename).expect("Unable to create file"));
                let _ = file.write_all(&data);

                Ok(())
            })
    }
}
