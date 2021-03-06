use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::dwd_layer::dwd_weather_interpretation::DwdWeatherInterpretation;

pub struct DwdWeatherLayer {
    ww_grid: LatLonValueGrid<f32>,
    ceiling_grid: LatLonValueGrid<f32>
}


impl DwdWeatherLayer {
    pub fn new(
        ww_grid: LatLonValueGrid<f32>,
        ceiling_grid: LatLonValueGrid<f32>
    ) -> Result<DwdWeatherLayer, Grib2Error> {
        if ww_grid.get_grid_dimensions() != ceiling_grid.get_grid_dimensions() {
            return Err(Grib2Error::InvalidData("grids have different dimensions".to_string()));
        }

        if ww_grid.get_grid_lat_lon_extent() != ceiling_grid.get_grid_lat_lon_extent() {
            return Err(Grib2Error::InvalidData("grids have different lat lon extents".to_string()));
        }

        let layer = DwdWeatherLayer { ww_grid, ceiling_grid };

        return Ok(layer);
    }


    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        return self.ww_grid.get_grid_dimensions();
    }


    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        return self.ww_grid.get_grid_lat_lon_extent();
    }


    pub fn get_ww_by_xy(&self, x: usize, y: usize) -> Option<DwdWeatherInterpretation> {
        return self.ww_grid
            .get_value_by_xy(x, y)
            .map(|v| DwdWeatherInterpretation::from_value(v as u8));
    }


    pub fn get_ww_by_lat_lon(&self, pos: &LatLon) -> Option<DwdWeatherInterpretation> {
        return self.ww_grid
            .get_value_by_lat_lon(pos)
            .map(|v| DwdWeatherInterpretation::from_value(v as u8));
    }


    pub fn get_ceiling_by_xy(&self, x: usize, y: usize) -> Option<f32> {
        return self.ceiling_grid.get_value_by_xy(x, y);
    }


    pub fn get_ceiling_by_lat_lon(&self, pos: &LatLon) -> Option<f32> {
        return self.ww_grid.get_value_by_lat_lon(pos);
    }
}
