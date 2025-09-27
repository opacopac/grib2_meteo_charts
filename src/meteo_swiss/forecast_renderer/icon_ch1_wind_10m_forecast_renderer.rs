use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::imaging::drawable::Drawable;
use crate::meteo_chart::forecast_renderer::wind_chart_renderer::WindChartRenderer;
use crate::meteo_chart::meteo_layer::meteo_wind_layer::MeteoWindLayer;
use crate::meteo_swiss::common::meteo_swiss_error::MeteoSwissError;
use crate::meteo_swiss::file_reader::icon_ch_wind_u_10m_reader::IconChWindU10mReader;
use crate::meteo_swiss::file_reader::icon_ch_wind_v_10m_reader::IconChWindV10mReader;
use crate::meteo_swiss::file_reader::icon_ch_wind_vmax_10m_reader::IconChWindVmax10mReader;
use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_renderer_helper::IconCh1ForecastRendererHelper;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::metobin::wind_metobin::WindMeteobin;
use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::fs::File;
use std::io::{BufWriter, Write};


pub struct IconCh1Wind10mForecastRenderer;


impl IconCh1Wind10mForecastRenderer {
    pub fn render(
        fc_run_u10m: &IconChForecastRun,
        fc_run_v10m: &IconChForecastRun,
        fc_run_vmax10m: &IconChForecastRun,
        unstructured_grid: &UnstructuredGrid,
        step_filter: &Vec<usize>,
    ) -> Result<(), MeteoSwissError> {
        fc_run_u10m.get_step_range()
            .into_par_iter()
            .try_for_each(|step_idx| {
                if !step_filter.is_empty() && !step_filter.contains(&step_idx) {
                    return Ok(());
                }

                info!("creating wind charts, time step {}", step_idx);

                let fc_step_u10m = &fc_run_u10m.steps[step_idx];
                let fc_step_v10m = &fc_run_v10m.steps[step_idx];
                let fc_step_vmax10m = &fc_run_vmax10m.steps[step_idx];

                let wind_u10m_grid = IconChWindU10mReader::read_grid_from_file(&fc_step_u10m.href, &unstructured_grid)?;
                let wind_v10m_grid = IconChWindV10mReader::read_grid_from_file(&fc_step_v10m.href, &unstructured_grid)?;
                let wind_vmax10m_grid = IconChWindVmax10mReader::read_grid_from_file(&fc_step_vmax10m.href, &unstructured_grid)?;

                let layer = MeteoWindLayer::new(wind_u10m_grid, wind_v10m_grid, Some(wind_vmax10m_grid))?;

                // map tiles
                let _ = WindChartRenderer::render_map_tiles(
                    &layer,
                    IconCh1ForecastRendererHelper::get_zoom_range(),
                    |tile: &Drawable, zoom: u32, x: u32, y: u32| IconCh1ForecastRendererHelper::save_tile_step(
                        tile, zoom, x, y, &layer.get_type().get_output_subdir(), &fc_run_u10m, step_idx,
                    ),
                );

                // meteobin
                let bin_data = WindMeteobin::create_bin_values(&layer);
                let filename = format!(
                    "{}WIND.meteobin",
                    IconCh1ForecastRendererHelper::get_output_path(&fc_run_u10m, step_idx, &layer.get_type().get_output_subdir()),
                );
                let mut file = BufWriter::new(File::create(&filename).expect("Unable to create wind meteobin file"));
                let _ = file.write_all(&bin_data);

                Ok(())
            })
    }
}
