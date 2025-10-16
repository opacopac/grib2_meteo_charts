use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::meteo_chart::meteo_layer::meteo_vertical_wind_layer::MeteoVerticalWindLayer;
use crate::meteo_swiss::common::icon_ch1_model_config::IconCh1ModelConfig;
use crate::meteo_swiss::common::meteo_swiss_error::MeteoSwissError;
use crate::meteo_swiss::file_reader::icon_ch_u_reader::IconChUReader;
use crate::meteo_swiss::file_reader::icon_ch_v_reader::IconChVReader;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::metobin::vertical_wind_metobin::VerticalWindMeteobin;
use log::info;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};


pub struct IconCh1VerticalWindForecastRenderer;


const MAX_PARALLEL_STEPS: usize = 3;


impl IconCh1VerticalWindForecastRenderer {
    pub fn render(
        fc_run_u: &IconChForecastRun,
        fc_run_v: &IconChForecastRun,
        unstructured_grid: &UnstructuredGrid,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
        step_filter: &Vec<usize>,
    ) -> Result<(), MeteoSwissError> {
        let steps = fc_run_u.get_step_range().collect::<Vec<usize>>();

        steps
            .into_par_iter()
            .with_max_len(MAX_PARALLEL_STEPS)
            .try_for_each(|step_idx| {
                if !step_filter.is_empty() && !step_filter.contains(&step_idx) {
                    return Ok(());
                }

                info!("creating vertical wind charts, time step {}", step_idx);
                let fc_step_u = &fc_run_u.steps[step_idx];
                let fc_step_v = &fc_run_v.steps[step_idx];

                let vertical_levels = IconCh1ModelConfig::get_vertical_level_range();
                let u_grids = IconChUReader::read_grids(&fc_step_u.href, &unstructured_grid, Some(&vertical_levels))?;
                let v_grids = IconChVReader::read_grids(&fc_step_v.href, &unstructured_grid, Some(&vertical_levels))?;

                let layer = MeteoVerticalWindLayer::new(hhl_grids.clone(), u_grids, v_grids);

                // meteobin
                let _ = VerticalWindMeteobin::create_meteobin_file(&layer, fc_run_u, step_idx)?;

                Ok(())
            })
    }
}
