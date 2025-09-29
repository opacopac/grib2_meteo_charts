use crate::dwd::dwd_file_reader::icon_d2_t_2m_reader::IconD2T2mReader;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_renderer_helper::IconD2ForecastRendererHelper;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::imaging::drawable::Drawable;
use crate::meteo_chart::forecast_renderer::temp_chart_renderer::TempChartRenderer;
use crate::meteo_chart::meteo_layer::meteo_temp_layer::MeteoTempLayer;
use crate::metobin::temp_metobin::TempMeteoBin;
use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};


pub struct IconD2TempForecastRenderer;


impl IconD2TempForecastRenderer {
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

                info!("creating temperature charts, time step {}", step);

                let fc_step = DwdForecastStep::new_from_run(forecast_run, step);
                let temp_grid = IconD2T2mReader::read_grid_from_file(&fc_step)?;
                let layer = MeteoTempLayer::new(temp_grid)?;

                // map tiles
                let _ = TempChartRenderer::render_map_tiles(
                    &layer,
                    (0, 7),
                    |tile: &Drawable, zoom: u32, x: u32, y: u32| IconD2ForecastRendererHelper::save_tile_step(tile, zoom, x, y, &layer.get_type().get_output_subdir(), &fc_step),
                );

                // meteobin
                let _ = TempMeteoBin::create_meteobin_file(&layer, forecast_run, step);

                Ok(())
            })
    }
}
