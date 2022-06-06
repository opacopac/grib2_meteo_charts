use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_dwd::lat_lon_value_grid::LatLonValueGrid;

pub struct DwdWindLayer {
    zonal_value_grid: LatLonValueGrid<f32>,
    meridional_value_grid: LatLonValueGrid<f32>
}


impl DwdWindLayer {
    pub fn new(
        zonal_value_grid: LatLonValueGrid<f32>,
        meridional_value_grid: LatLonValueGrid<f32>
    ) -> Result<DwdWindLayer, Grib2Error> {
        if zonal_value_grid.get_grid_dimensions() != meridional_value_grid.get_grid_dimensions() {
            return Err(Grib2Error::InvalidData("grids have different dimensions".to_string()));
        }

        if zonal_value_grid.get_grid_lat_lon_extent() != meridional_value_grid.get_grid_lat_lon_extent() {
            return Err(Grib2Error::InvalidData("grids have different lat lon extents".to_string()));
        }

        let layer = DwdWindLayer { zonal_value_grid, meridional_value_grid };

        return Ok(layer);
    }


    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        return self.zonal_value_grid.get_grid_dimensions();
    }


    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        return self.zonal_value_grid.get_grid_lat_lon_extent();
    }


    pub fn get_wind_speed_e_n_by_xy(&self, x: usize, y: usize) -> Option<(f32, f32)> {
        let value_e = self.zonal_value_grid.get_value_by_xy(x, y);
        let value_n = self.meridional_value_grid.get_value_by_xy(x, y);

        return match (value_e, value_n) {
            (Some(val_e), Some(val_n)) => Some((val_e, val_n)),
            _ => None
        };
    }


    pub fn get_wind_speed_tot_xy(&self, x: usize, y: usize) -> Option<f32> {
        return self.get_wind_speed_e_n_by_xy(x, y)
            .map(|(value_e, value_n)| (value_e * value_e + value_n * value_n).sqrt());
    }


    pub fn get_wind_speed_dir_by_xy(&self, x: usize, y: usize) -> Option<(f32, f32)> {
        return self.get_wind_speed_e_n_by_xy(x, y)
            .map(|(value_e, value_n)| (
                (value_e * value_e + value_n * value_n).sqrt(),
                value_n.atan2(value_e).to_degrees()
            ));
    }


    pub fn get_wind_speed_e_n_by_lat_lon(&self, pos: &LatLon) -> Option<(f32, f32)> {
        let value_e = self.zonal_value_grid.get_value_by_lat_lon(pos);
        let value_n = self.meridional_value_grid.get_value_by_lat_lon(pos);

        return match (value_e, value_n) {
            (Some(val_e), Some(val_n)) => Some((val_e, val_n)),
            _ => None
        }
    }


    pub fn get_wind_speed_tot_by_lat_lon(&self, pos: &LatLon) -> Option<f32> {
        return self.get_wind_speed_e_n_by_lat_lon(pos)
            .map(|(value_e, value_n)| (value_e * value_e + value_n * value_n).sqrt());
    }
}
