use crate::dwd::dwd_file_reader::icon_d2_clc_reader::IconD2ClcReader;
use crate::dwd::dwd_file_reader::icon_d2_hhl_reader::IconD2HhlReader;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_renderer_helper::IconD2ForecastRendererHelper;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::meteo_chart::meteo_layer::meteo_vertical_cloud_layer::MeteoVerticalCloudLayer;
use crate::metobin::vertical_cloud_metobin::VerticalCloudMeteobin;
use log::info;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::RangeInclusive;


pub struct IconD2VerticalCloudForecastRenderer;


const VERTICAL_CLOUDS_SUB_DIR: &str = "vertical_clouds";
const VERTICAL_LEVEL_RANGE: RangeInclusive<u8> = 25..=65; //25..=65;


impl IconD2VerticalCloudForecastRenderer {
    pub fn render(
        forecast_run: &DwdForecastRun,
        step_filter: &Vec<usize>,
    ) -> Result<(), ForecastRendererError> {
        let hhl_grids = IconD2HhlReader::read_hhl_grids(forecast_run, VERTICAL_LEVEL_RANGE)?;

        DwdForecastStep::get_step_range()
            .try_for_each(|step| {
                if !step_filter.is_empty() && !step_filter.contains(&step) {
                    return Ok(());
                }

                info!("creating vertical cloud charts, time step {}", step);
                let fc_step = DwdForecastStep::new_from_run(forecast_run, step);
                let clc_grids = IconD2ClcReader::read_clc_grids(&fc_step, VERTICAL_LEVEL_RANGE)?;
                let vertical_cloud_layer = MeteoVerticalCloudLayer::new(&hhl_grids, clc_grids);

                // meteobin
                let bin_data = VerticalCloudMeteobin::create_bin_values(&vertical_cloud_layer);
                let path = IconD2ForecastRendererHelper::get_output_path(&fc_step, VERTICAL_CLOUDS_SUB_DIR);
                let filename = format!("{}VERTICAL_CLOUDS.meteobin", path);

                info!("writing vertical clouds meteobin file {}", &filename);
                fs::create_dir_all(&path)?;
                let mut file = BufWriter::new(File::create(&filename).expect(&*format!("Unable to create vertical clouds meteobin file {}", &filename)));
                let _ = file.write_all(&bin_data);

                Ok(())
            })
    }
}
