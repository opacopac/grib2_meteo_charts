use crate::imaging::drawable::Drawable;
use crate::meteo_chart::meteo_layer::meteo_layer_type::MeteoLayerType;
use crate::meteo_common::meteo_forecast_renderer_helper::MeteoForecastFileHelper;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use std::fs;

pub struct MapTileFileHelper;


impl MapTileFileHelper {
    pub fn save_tile_step(
        tile: &Drawable,
        zoom: u32,
        x: u32,
        y: u32,
        layer_type: &MeteoLayerType,
        fc_run: &MeteoForecastRun,
        step_nr: usize,
    ) {
        let path = format!(
            "{}{}/{}",
            MeteoForecastFileHelper::get_output_path(fc_run, step_nr, layer_type),
            zoom,
            x
        );
        fs::create_dir_all(&path).unwrap();

        let filename = format!("{}/{}.png", &path, y);
        let _result = tile.safe_image(&filename);
    }
}
