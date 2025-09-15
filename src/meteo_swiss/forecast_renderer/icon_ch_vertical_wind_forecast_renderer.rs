use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grid::unstructured_grid::UnstructuredGrid;
use crate::meteo_layer::meteo_vertical_wind_layer::MeteoVerticalWindLayer;
use crate::meteo_swiss::file_reader::icon_ch_u_reader::IconChUReader;
use crate::meteo_swiss::file_reader::icon_ch_v_reader::IconChVReader;
use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_renderer_helper::IconCh1ForecastRendererHelper;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use crate::metobin::vertical_wind_metobin::VerticalWindMeteobin;
use log::info;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::RangeInclusive;

pub struct IconCh1VerticalWindForecastRenderer;


const VERTICAL_WIND_SUB_DIR: &str = "vertical_wind";
const VERTICAL_LEVEL_RANGE: RangeInclusive<usize> = 30..=79;
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

                let u_grids = IconChUReader::read_grids(&fc_step_u.href, &unstructured_grid, Some(VERTICAL_LEVEL_RANGE))?;
                let v_grids = IconChVReader::read_grids(&fc_step_v.href, &unstructured_grid, Some(VERTICAL_LEVEL_RANGE))?;

                let vertical_wind_layer = MeteoVerticalWindLayer::new(&hhl_grids, u_grids, v_grids);

                // meteobin
                let vert_wind_bin = VerticalWindMeteobin::new(vertical_wind_layer);
                let data = vert_wind_bin.create_bin_values();
                let path = IconCh1ForecastRendererHelper::get_output_path(&fc_run_u, step_idx, VERTICAL_WIND_SUB_DIR);
                let filename = format!("{}VERTICAL_WIND.meteobin", path);

                info!("writing vertical wind meteobin file {}", &filename);
                fs::create_dir_all(&path)?;
                let mut file = BufWriter::new(File::create(&filename).expect(&*format!("Unable to create vertical wind meteobin file {}", &filename)));
                let _ = file.write_all(&data);

                Ok(())
            })
    }
}
