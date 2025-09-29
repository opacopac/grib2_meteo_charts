use crate::dwd::dwd_file_reader::icon_d2_ceiling_reader::IconD2CeilingReader;
use crate::dwd::dwd_file_reader::icon_d2_clct_mod_reader::IconD2ClctModReader;
use crate::dwd::dwd_file_reader::icon_d2_tot_prec_reader::IconD2TotPrecReader;
use crate::dwd::dwd_file_reader::icon_d2_ww_reader::IconD2WwReader;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_renderer_helper::IconD2ForecastRendererHelper;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::imaging::drawable::Drawable;
use crate::meteo_chart::forecast_renderer::cloud_precip_chart_renderer::CloudPrecipChartRenderer;
use crate::meteo_chart::meteo_layer::meteo_cloud_precip_layer::MeteoCloudPrecipLayer;
use crate::meteo_chart::meteo_layer::weather_layer::WeatherLayer;
use crate::metobin::precip_metobin::PrecipMeteoBin;
use crate::metobin::weather_metobin::WeatherMeteoBin;
use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};


pub struct IconD2CloudPrecipRenderer;


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

                let clct_grid = IconD2ClctModReader::read_grid_from_file(&fc_step)
                    .map_err(|err| ForecastRendererError::ReadGridFromClctFileError(err))?;
                let precip_grid0 = IconD2TotPrecReader::read_grid_from_file(&fc_previous_step)
                    .map_err(|err| ForecastRendererError::ReadGridFromPrecipFileError(err))?;
                let precip_grid1 = IconD2TotPrecReader::read_grid_from_file(&fc_step)
                    .map_err(|err| ForecastRendererError::ReadGridFromPrecipFileError(err))?;
                let ww_grid = IconD2WwReader::read_grid_from_file(&fc_step)?;
                let ceiling_grid = IconD2CeilingReader::read_grid_from_file(&fc_step)?;

                let clct_precip_layer = MeteoCloudPrecipLayer::new(clct_grid.clone(), precip_grid0, precip_grid1)?;
                let weather_layer = WeatherLayer::new(clct_grid, ceiling_grid, Some(ww_grid))?;

                // map tiles
                let _ = CloudPrecipChartRenderer::render_map_tiles(
                    &clct_precip_layer,
                    (0, 7),
                    |tile: &Drawable, zoom: u32, x: u32, y: u32| IconD2ForecastRendererHelper::save_tile_step(
                        tile, zoom, x, y, &clct_precip_layer.get_type().get_output_subdir(), &fc_step),
                );

                // precip meteobin
                let _ = PrecipMeteoBin::create_meteobin_file(&clct_precip_layer, forecast_run, step);

                // ww meteobin
                let _ = WeatherMeteoBin::create_meteobin_file(&weather_layer, forecast_run, step);

                Ok(())
            })
    }
}
