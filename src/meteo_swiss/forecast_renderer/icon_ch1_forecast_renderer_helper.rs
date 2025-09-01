use crate::imaging::drawable::Drawable;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use std::fs;

pub struct IconCh1ForecastRendererHelper;


const FORECAST_BASE_DIR: &str = "./output/icon-ch1/";
const DATE_FORMAT: &str = "%Y%m%d";


impl IconCh1ForecastRendererHelper {
    pub fn get_zoom_range() -> (u32, u32) {
        (0, 8)
    }


    pub fn save_tile_step(
        tile: &Drawable,
        zoom: u32,
        x: u32,
        y: u32,
        layer: &str,
        fc_run: &IconChForecastRun,
        fc_step_idx: usize,
    ) {
        let path = format!(
            "{}{}/{}",
            Self::get_output_path(fc_run, fc_step_idx, layer),
            zoom,
            x
        );
        fs::create_dir_all(&path).unwrap();

        let filename = format!("{}/{}.png", &path, y);
        let _result = tile.safe_image(&filename);
    }


    pub fn get_output_path(
        fc_run: &IconChForecastRun,
        fc_step_idx: usize,
        layer: &str,
    ) -> String {
        format!(
            "{}{}{}/{:03}/{}/",
            FORECAST_BASE_DIR,
            fc_run.start_date.format(DATE_FORMAT),
            fc_run.run_name.get_name(),
            fc_step_idx,
            layer,
        )
    }
}
