use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::meteo_vertical_wind_layer::MeteoVerticalWindLayer;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use crate::meteo_swiss::common::icon_ch1_model_config::IconCh1ModelConfig;
use crate::meteo_swiss::file_reader::icon_ch_u_reader::IconChUReader;
use crate::meteo_swiss::file_reader::icon_ch_v_reader::IconChVReader;


pub struct IconChVerticalWindReader;


impl IconChVerticalWindReader {
    pub fn read_layer_from_file(
        u_step: &MeteoForecastRun2Step,
        v_step: &MeteoForecastRun2Step,
        unstructured_grid: &UnstructuredGrid,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
    ) -> Result<MeteoVerticalWindLayer, Grib2Error> {
        let vertical_levels = IconCh1ModelConfig::get_vertical_level_range();
        let u_grids = IconChUReader::read_grids(&u_step.get_file_url(), &unstructured_grid, Some(&vertical_levels))?;
        let v_grids = IconChVReader::read_grids(&v_step.get_file_url(), &unstructured_grid, Some(&vertical_levels))?;

        let layer = MeteoVerticalWindLayer::new(hhl_grids.clone(), u_grids, v_grids);


        Ok(layer)
    }
}
