use crate::dwd::common::icon_d2_model_config::IconD2ModelConfig;
use crate::dwd::dwd_file_reader::dwd_icon_u_reader::DwdIconUReader;
use crate::dwd::dwd_file_reader::dwd_icon_v_reader::DwdIconVReader;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::meteo_vertical_wind_layer::MeteoVerticalWindLayer;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct IconD2VerticalWindReader;


impl IconD2VerticalWindReader {
    pub fn read_layer_from_file(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
    ) -> Result<MeteoVerticalWindLayer, Grib2Error> {
        let vertical_levels = IconD2ModelConfig::get_vertical_level_range();
        let u_grids = DwdIconUReader::read_u_grids(fc_run, fc_step, &vertical_levels)?;
        let v_grids = DwdIconVReader::read_v_grids(fc_run, fc_step, &vertical_levels)?;

        let layer = MeteoVerticalWindLayer::new(hhl_grids.clone(), u_grids, v_grids);

        Ok(layer)
    }
}
