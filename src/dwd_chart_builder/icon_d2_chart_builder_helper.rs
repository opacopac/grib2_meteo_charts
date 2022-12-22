use std::fs;
use std::io::Read;

use bzip2::read::BzDecoder;
use log::info;

use crate::dwd_files::icon_d2_file::DWD_DATE_FORMAT;
use crate::dwd_forecast_runs::dwd_forecast_step::DwdForecastStep;
use crate::imaging::drawable::Drawable;

pub struct IconD2ChartBuilderHelper;


const FORECAST_BASE_DIR: &str = "./output/icon-d2/";


impl IconD2ChartBuilderHelper {
    pub fn get_file_reader(filename: &str) -> impl Read {
        info!("reading file {}", filename);
        let response_result = ureq::get(filename).call().unwrap(); // TODO
        let reader = response_result.into_reader();
        let bz_decoder = BzDecoder::new(reader);

        return bz_decoder;
    }


    pub fn save_tile_step(
        tile: &Drawable,
        zoom: u32,
        x: u32,
        y: u32,
        layer: &str,
        fc_step: &DwdForecastStep
    ) {
        let path = format!(
            "{}{}/{}",
            Self::get_output_path(fc_step, layer),
            zoom,
            x
        );
        fs::create_dir_all(&path).unwrap();

        let filename = format!("{}/{}.png", &path, y);
        let _result = tile.safe_image(&filename);
    }


    pub fn get_output_path(
        fc_step: &DwdForecastStep,
        layer: &str
    ) -> String {
        return format!(
            "{}{}{}/{:03}/{}/",
            FORECAST_BASE_DIR,
            fc_step.run.start_date.format(DWD_DATE_FORMAT),
            fc_step.run.run_name.get_name(),
            fc_step.step,
            layer,
        );
    }
}
