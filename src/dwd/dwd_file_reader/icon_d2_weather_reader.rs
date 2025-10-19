use crate::dwd::dwd_file_reader::icon_d2_ceiling_reader::IconD2CeilingReader;
use crate::dwd::dwd_file_reader::icon_d2_clct_mod_reader::IconD2ClctModReader;
use crate::dwd::dwd_file_reader::icon_d2_ww_reader::IconD2WwReader;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::weather_layer::WeatherLayer;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;


pub struct IconD2WeatherReader;


impl IconD2WeatherReader {
    pub fn read_layer_from_files(
        fc_run: &MeteoForecastRun2,
        fc_step_clct: &MeteoForecastRun2Step,
        fc_step_ceiling: &MeteoForecastRun2Step,
        fc_step_ww: &MeteoForecastRun2Step,
    ) -> Result<WeatherLayer, Grib2Error> {
        let grid_clct = IconD2ClctModReader::read_grid_from_file(fc_run, fc_step_clct)?;
        let grid_ceiling = IconD2CeilingReader::read_grid_from_file(fc_run, fc_step_ceiling)?;
        let grid_ww = IconD2WwReader::read_grid_from_file(fc_run, fc_step_ww)?;

        let layer = WeatherLayer::new(grid_clct, grid_ceiling, Some(grid_ww))?;

        Ok(layer)
    }
}
