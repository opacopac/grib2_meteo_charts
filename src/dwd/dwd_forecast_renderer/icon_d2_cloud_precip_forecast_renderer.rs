use crate::dwd::dwd_file_reader::icon_d2_ceiling_reader::IconD2CeilingReader;
use crate::dwd::dwd_file_reader::icon_d2_clct_mod_reader::IconD2ClctModReader;
use crate::dwd::dwd_file_reader::icon_d2_tot_prec_reader::IconD2TotPrecReader;
use crate::dwd::dwd_file_reader::icon_d2_ww_reader::IconD2WwReader;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_renderer_helper::IconD2ForecastRendererHelper;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::imaging::drawable::Drawable;
use crate::meteo_chart::cloud_precip_chart_renderer::CloudPrecipChartRenderer;
use crate::meteo_layer::meteo_cloud_precip_layer::MeteoCloudPrecipLayer;
use crate::meteo_layer::weather_layer::WeatherLayer;
use crate::metobin::precip_metobin::PrecipMeteoBin;
use crate::metobin::weather_metobin::WeatherMeteoBin;
use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::fs::File;
use std::io::{BufWriter, Write};


pub struct IconD2CloudPrecipRenderer;


const WEATHER_LAYER: &str = "clct_precip";


impl IconD2CloudPrecipRenderer {
    pub fn render(
        forecast_run: &DwdForecastRun,
        step_filter: &Vec<usize>,
    ) -> Result<(), ForecastRendererError> {
        DwdForecastStep::get_step_range()
            .into_par_iter()
            .try_for_each(|step| {
                if !step_filter.is_empty() && !step_filter.contains(&step) {
                    return Ok(());
                }

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

                let layer = MeteoCloudPrecipLayer::new(clct_grid.clone(), precip_grid0, precip_grid1)?;

                let _ = CloudPrecipChartRenderer::render_map_tiles(
                    &layer,
                    (0, 7),
                    |tile: &Drawable, zoom: u32, x: u32, y: u32| IconD2ForecastRendererHelper::save_tile_step(tile, zoom, x, y, WEATHER_LAYER, &fc_step),
                );

                // precip meteobin
                let precip_bin_data = PrecipMeteoBin::create_bin_values(&layer);
                let precip_filename = format!(
                    "{}PRECIP.meteobin",
                    IconD2ForecastRendererHelper::get_output_path(&fc_step, WEATHER_LAYER),
                );
                let mut precip_file = BufWriter::new(File::create(&precip_filename).expect("Unable to create file"));
                let _ = precip_file.write_all(&precip_bin_data);


                // ww meteobin
                let ww_grid = IconD2WwReader::read_grid_from_file(&fc_step)?;
                let ceiling_grid = IconD2CeilingReader::read_grid_from_file(&fc_step)?;

                let weather_layer = WeatherLayer::new(clct_grid, ceiling_grid, Some(ww_grid))?;
                let ww_bin_data = WeatherMeteoBin::create_bin_values(&weather_layer);
                let ww_filename = format!(
                    "{}WW.meteobin",
                    IconD2ForecastRendererHelper::get_output_path(&fc_step, WEATHER_LAYER),
                );
                let mut ww_file = BufWriter::new(File::create(&ww_filename).expect("Unable to create file"));
                let _ = ww_file.write_all(&ww_bin_data);

                Ok(())
            })
    }
}
