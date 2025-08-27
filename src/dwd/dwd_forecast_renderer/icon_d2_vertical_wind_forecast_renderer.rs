use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::RangeInclusive;

use log::info;

use crate::dwd::dwd_file_reader::icon_d2_hhl_reader::IconD2HhlReader;
use crate::dwd::dwd_file_reader::icon_d2_u_reader::IconD2UReader;
use crate::dwd::dwd_file_reader::icon_d2_v_reader::IconD2VReader;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_renderer_helper::IconD2ForecastRendererHelper;
use crate::meteo_layer::meteo_vertical_wind_layer::MeteoVerticalWindLayer;
use crate::metobin::vertical_wind_metobin::VerticalWindMeteobin;

pub struct IconD2VerticalWindForecastRenderer;

const VERTICAL_WIND_SUB_DIR: &str = "vertical_wind";
const VERTICAL_LEVEL_RANGE: RangeInclusive<u8> = 25..=65; //25..=65;


impl IconD2VerticalWindForecastRenderer {
    pub fn create(forecast_run: &DwdForecastRun) -> Result<(), ForecastRendererError> {
        let hhl_grids = IconD2HhlReader::read_hhl_grids(forecast_run, VERTICAL_LEVEL_RANGE)?;

        DwdForecastStep::get_step_range()
            .try_for_each(|step| {
                info!("creating vertical cloud charts, time step {}", step);
                let fc_step = DwdForecastStep::new_from_run(forecast_run, step);
                let u_grids = IconD2UReader::read_u_grids(&fc_step, VERTICAL_LEVEL_RANGE)?;
                let v_grids = IconD2VReader::read_v_grids(&fc_step, VERTICAL_LEVEL_RANGE)?;
                let vertical_wind_layer = MeteoVerticalWindLayer::new(&hhl_grids, u_grids, v_grids);

                // meteobin
                let vert_wind_bin = VerticalWindMeteobin::new(vertical_wind_layer);
                let data = vert_wind_bin.create_bin_values();
                let path = IconD2ForecastRendererHelper::get_output_path(&fc_step, VERTICAL_WIND_SUB_DIR);
                let filename = format!("{}VERTICAL_WIND_D2.meteobin", path);

                info!("writing vertical wind meteobin file {}", &filename);
                fs::create_dir_all(&path)?;
                let mut file = BufWriter::new(File::create(&filename).expect(&*format!("Unable to create vertical wind meteobin file {}", &filename)));
                let _ = file.write_all(&data);

                Ok(())
            })
    }
}
