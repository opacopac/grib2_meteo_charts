use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;

pub struct DwdTempLayer {
    temp_value_grid: LatLonValueGrid<f32>,
}


impl DwdTempLayer {
    pub fn new(
        temp_value_grid: LatLonValueGrid<f32>
    ) -> Result<DwdTempLayer, Grib2Error> {
        let layer = DwdTempLayer { temp_value_grid };

        return Ok(layer);
    }


    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        return self.temp_value_grid.get_grid_dimensions();
    }


    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        return self.temp_value_grid.get_grid_lat_lon_extent();
    }


    pub fn get_temp_by_xy(&self, x: usize, y: usize) -> Option<f32> {
        return self.temp_value_grid.get_value_by_xy(x, y);
    }


    pub fn get_temp_by_lat_lon(&self, pos: &LatLon) -> Option<f32> {
        return self.temp_value_grid.get_value_by_lat_lon(pos);
    }
}
