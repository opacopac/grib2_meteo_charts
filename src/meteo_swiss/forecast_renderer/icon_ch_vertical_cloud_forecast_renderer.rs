use crate::grid::unstructured_grid::UnstructuredGrid;
use crate::meteo_layer::meteo_vertical_cloud_layer::MeteoVerticalCloudLayer;
use crate::meteo_swiss::file_reader::icon_ch_clc_reader::IconChClcReader;
use crate::meteo_swiss::file_reader::icon_ch_hhl_reader::IconChHhlReader;
use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_renderer_helper::IconCh1ForecastRendererHelper;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use crate::metobin::vertical_cloud_metobin::VerticalCloudMeteobin;
use log::info;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::RangeInclusive;

pub struct IconCh1VerticalCloudForecastRenderer;


const VERTICAL_CLOUDS_SUB_DIR: &str = "vertical_clouds";
const VERTICAL_LEVEL_RANGE: RangeInclusive<usize> = 25..=65;


impl IconCh1VerticalCloudForecastRenderer {
    pub fn render(
        fc_run_hhl: &IconChForecastRun,
        fc_run_clc: &IconChForecastRun,
        unstructured_grid: &UnstructuredGrid,
        step_filter: &Vec<usize>,
    ) -> Result<(), MeteoSwissError> {
        fc_run_clc.get_step_range()
            .try_for_each(|step_idx| {
                if !step_filter.is_empty() && !step_filter.contains(&step_idx) {
                    return Ok(());
                }

                info!("creating vertical cloud charts, time step {}", step_idx);
                let fc_step_hhl = &fc_run_hhl.steps[step_idx];
                let fc_step_clc = &fc_run_clc.steps[step_idx];

                let hhl_grids = IconChHhlReader::read_grids(&fc_step_hhl.href, &unstructured_grid, Some(VERTICAL_LEVEL_RANGE))?;
                let clc_grids = IconChClcReader::read_grids(&fc_step_clc.href, &unstructured_grid, Some(VERTICAL_LEVEL_RANGE))?;

                let vertical_cloud_layer = MeteoVerticalCloudLayer::new(&hhl_grids, clc_grids);

                // meteobin
                let vert_cloud_bin = VerticalCloudMeteobin::new(vertical_cloud_layer);
                let data = vert_cloud_bin.create_bin_values();
                let path = IconCh1ForecastRendererHelper::get_output_path(&fc_run_clc, step_idx, VERTICAL_CLOUDS_SUB_DIR);
                let filename = format!("{}VERTICAL_CLOUDS_D2.meteobin", path);

                info!("writing vertical clouds meteobin file {}", &filename);
                fs::create_dir_all(&path)?;
                let mut file = BufWriter::new(File::create(&filename).expect(&*format!("Unable to create vertical clouds meteobin file {}", &filename)));
                let _ = file.write_all(&data);

                Ok(())
            })
    }
}
