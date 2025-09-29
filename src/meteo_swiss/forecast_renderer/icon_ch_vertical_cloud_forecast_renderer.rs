use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::meteo_chart::meteo_layer::meteo_vertical_cloud_layer::MeteoVerticalCloudLayer;
use crate::meteo_swiss::common::icon_ch1_model_config::IconCh1ModelConfig;
use crate::meteo_swiss::common::meteo_swiss_error::MeteoSwissError;
use crate::meteo_swiss::file_reader::icon_ch_clc_reader::IconChClcReader;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::metobin::vertical_cloud_metobin::VerticalCloudMeteobin;
use log::info;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};


pub struct IconCh1VerticalCloudForecastRenderer;


const MAX_PARALLEL_STEPS: usize = 3;


impl IconCh1VerticalCloudForecastRenderer {
    pub fn render(
        fc_run_clc: &IconChForecastRun,
        unstructured_grid: &UnstructuredGrid,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
        step_filter: &Vec<usize>,
    ) -> Result<(), MeteoSwissError> {
        let steps = fc_run_clc.get_step_range().collect::<Vec<usize>>();

        steps
            .into_par_iter()
            .with_max_len(MAX_PARALLEL_STEPS)
            .try_for_each(|step_idx| {
                if !step_filter.is_empty() && !step_filter.contains(&step_idx) {
                    return Ok(());
                }

                info!("creating vertical cloud charts, time step {}", step_idx);
                let fc_step_clc = &fc_run_clc.steps[step_idx];

                let vertical_levels = IconCh1ModelConfig::get_vertical_level_range();
                let clc_grids = IconChClcReader::read_grids(&fc_step_clc.href, &unstructured_grid, Some(&vertical_levels))?;

                let layer = MeteoVerticalCloudLayer::new(&hhl_grids, clc_grids);

                // meteobin
                let _ = VerticalCloudMeteobin::create_meteobin_file(&layer, fc_run_clc, step_idx)?;

                Ok(())
            })
    }
}
