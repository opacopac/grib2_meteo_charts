use std::fs;

use crate::dwd::dwd_files::icon_d2_file::DWD_DATE_FORMAT;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::imaging::drawable::Drawable;

pub struct IconD2ForecastRendererHelper;


const FORECAST_BASE_DIR: &str = "./output/icon-d2/";


impl IconD2ForecastRendererHelper {
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
