use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::meteo_chart::meteo_layer::meteo_vertical_wind_layer::MeteoVerticalWindLayer;
use crate::meteo_swiss::common::icon_ch1_model_config::IconCh1ModelConfig;
use crate::meteo_swiss::common::meteo_swiss_error::MeteoSwissError;
use crate::meteo_swiss::file_reader::icon_ch_u_reader::IconChUReader;
use crate::meteo_swiss::file_reader::icon_ch_v_reader::IconChVReader;
use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_renderer_helper::IconCh1ForecastRendererHelper;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::metobin::meteobin_type::MeteobinType;
use crate::metobin::vertical_wind_metobin::VerticalWindMeteobin;
use log::info;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

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

                let layer = MeteoVerticalWindLayer::new(&hhl_grids, u_grids, v_grids);

                // meteobin
                let bin_data = VerticalWindMeteobin::create_bin_values(&layer);
                let path = IconCh1ForecastRendererHelper::get_output_path(&fc_run_u, step_idx, &layer.get_type().get_output_subdir());
                let filename = format!("{}{}", path, MeteobinType::VerticalWind.get_output_file());

                info!("writing vertical wind meteobin file {}", &filename);
                fs::create_dir_all(&path)?;
                let mut file = BufWriter::new(File::create(&filename).expect(&*format!("Unable to create vertical wind meteobin file {}", &filename)));
                let _ = file.write_all(&bin_data);

                Ok(())
            })
    }
}
