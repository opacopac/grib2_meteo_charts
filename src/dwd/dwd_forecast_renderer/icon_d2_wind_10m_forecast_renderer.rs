use crate::dwd::dwd_file_reader::icon_d2_u_10m_reader::IconD2U10mReader;
use crate::dwd::dwd_file_reader::icon_d2_v_10m_reader::IconD2V10mReader;
use crate::dwd::dwd_file_reader::icon_d2_vmax_10m_reader::IconD2Vmax10mReader;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_renderer_helper::IconD2ForecastRendererHelper;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::imaging::drawable::Drawable;
use crate::meteo_chart::wind_chart_renderer::WindChartRenderer;
use crate::meteo_layer::meteo_wind_layer::MeteoWindLayer;
use crate::metobin::wind_metobin::WindMeteobin;
use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::fs::File;
use std::io::{BufWriter, Write};


pub struct IconD2Wind10mForecastRenderer;


const WIND_LAYER: &str = "wind";


impl IconD2Wind10mForecastRenderer {
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

                info!("creating wind charts, time step {}", step);

                let fc_step = DwdForecastStep::new_from_run(forecast_run, step);

                let wind_u_grid = IconD2U10mReader::read_grid_from_file(&fc_step)?;
                let wind_v_grid = IconD2V10mReader::read_grid_from_file(&fc_step)?;
                let wind_v_max_grid = IconD2Vmax10mReader::read_grid_from_file(&fc_step)?;

                let layer = MeteoWindLayer::new(wind_u_grid, wind_v_grid, Some(wind_v_max_grid))?;

                // map tiles
                let _ = WindChartRenderer::render_map_tiles(
                    &layer,
                    (0, 7),
                    |tile: &Drawable, zoom: u32, x: u32, y: u32| IconD2ForecastRendererHelper::save_tile_step(tile, zoom, x, y, WIND_LAYER, &fc_step),
                );

                // meteobin
                let bin_data = WindMeteobin::create_bin_values(&layer);
                let filename = format!(
                    "{}WIND.meteobin",
                    IconD2ForecastRendererHelper::get_output_path(&fc_step, WIND_LAYER),
                );
                let mut file = BufWriter::new(File::create(&filename).expect("Unable to create wind meteobin file"));
                let _ = file.write_all(&bin_data);

                Ok(())
            })
    }
}
