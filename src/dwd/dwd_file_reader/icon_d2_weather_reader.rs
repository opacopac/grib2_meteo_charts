use crate::dwd::dwd_file_reader::dwd_icon_ceiling_reader::DwdIconCeilingReader;
use crate::dwd::dwd_file_reader::icon_d2_clct_mod_reader::IconD2ClctModReader;
use crate::dwd::dwd_file_reader::icon_d2_ww_reader::IconD2WwReader;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::weather_layer::WeatherLayer;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct IconD2WeatherReader;


impl IconD2WeatherReader {
    pub fn read_layer(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> Result<WeatherLayer, Grib2Error> {
        let grid_clct = IconD2ClctModReader::read_grid_from_file(fc_run, fc_step)?;
        let grid_ceiling = DwdIconCeilingReader::read_grid_from_file(fc_run, fc_step)?;
        let grid_ww = IconD2WwReader::read_grid_from_file(fc_run, fc_step)?;

        let layer = WeatherLayer::new(grid_clct, grid_ceiling, Some(grid_ww))?;

        Ok(layer)
    }
}
