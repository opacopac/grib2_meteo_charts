use crate::imaging::drawable::Drawable;
use crate::meteo_chart::meteo_layer::meteo_layer_type::MeteoLayerType;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use std::fs;


pub struct MeteoForecastRendererHelper;


const FORECAST_BASE_DIR: &str = "./output/";
const FC_STARTDATE_FORMAT: &str = "%Y%m%d";


impl MeteoForecastRendererHelper {
    pub fn save_tile_step(
        tile: &Drawable,
        zoom: u32,
        x: u32,
        y: u32,
        layer_type: &MeteoLayerType,
        fc_run: &dyn MeteoForecastRun,
        fc_step: usize,
    ) {
        let path = format!(
            "{}{}/{}",
            Self::get_output_path(fc_run, fc_step, layer_type),
            zoom,
            x
        );
        fs::create_dir_all(&path).unwrap();

        let filename = format!("{}/{}.png", &path, y);
        let _result = tile.safe_image(&filename);
    }


    pub fn get_output_path(
        fc_run: &dyn MeteoForecastRun,
        fc_step: usize,
        layer_type: &MeteoLayerType,
    ) -> String {
        format!(
            "{}{}/{}{}/{:03}/{}/",
            FORECAST_BASE_DIR,
            fc_run.get_model_name(),
            fc_run.get_start_date().format(FC_STARTDATE_FORMAT),
            fc_run.get_name(),
            fc_step,
            &layer_type.get_output_subdir(),
        )
    }
}
