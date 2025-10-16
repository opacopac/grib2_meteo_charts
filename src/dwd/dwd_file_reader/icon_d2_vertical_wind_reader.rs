use crate::dwd::common::icon_d2_model_config::IconD2ModelConfig;
use crate::dwd::dwd_file_reader::icon_d2_u_reader::IconD2UReader;
use crate::dwd::dwd_file_reader::icon_d2_v_reader::IconD2VReader;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::meteo_vertical_wind_layer::MeteoVerticalWindLayer;


pub struct IconD2VerticalWindReader;


impl IconD2VerticalWindReader {
    pub fn read_layer_from_file(
        fc_step: &DwdForecastStep,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
    ) -> Result<MeteoVerticalWindLayer, Grib2Error> {
        let vertical_levels = IconD2ModelConfig::get_vertical_level_range();
        let u_grids = IconD2UReader::read_u_grids(fc_step, &vertical_levels)?;
        let v_grids = IconD2VReader::read_v_grids(fc_step, &vertical_levels)?;

        let layer = MeteoVerticalWindLayer::new(hhl_grids.clone(), u_grids, v_grids);

        Ok(layer)
    }
}
