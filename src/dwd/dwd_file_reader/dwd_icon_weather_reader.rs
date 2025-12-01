use crate::dwd::dwd_file_reader::dwd_icon_ceiling_reader::DwdIconCeilingReader;
use crate::dwd::dwd_file_reader::dwd_icon_clct_mod_reader::DwdIconClctModReader;
use crate::dwd::dwd_file_reader::dwd_icon_ww_reader::DwdIconWwReader;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::weather_layer::WeatherLayer;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct DwdIconWeatherReader;


impl DwdIconWeatherReader {
    pub fn read_layer(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> Result<WeatherLayer, Grib2Error> {
        let grid_clct = DwdIconClctModReader::read_grid_from_file(fc_run, fc_step)?;
        let grid_ceiling = DwdIconCeilingReader::read_grid_from_file(fc_run, fc_step)?;
        let grid_ww = DwdIconWwReader::read_grid_from_file(fc_run, fc_step)?;

        let layer = WeatherLayer::new(grid_clct, grid_ceiling, Some(grid_ww))?;

        Ok(layer)
    }
}
