use crate::geo::lat_lon::LatLon;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section0::discipline::Discipline;
use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
use crate::meteo::wind_layer::WindLayer;
use crate::meteo_dwd::discipline_checker::DisciplineChecker;
use crate::meteo_dwd::value_grid::ValueGrid;

pub struct DwdWindLayer {
    pub zonal_value_grid: ValueGrid,
    pub meridional_value_grid: ValueGrid
}


impl DwdWindLayer {
    pub fn from_grib2(
        east_wind_doc: Grib2Document,
        north_wind_doc: Grib2Document,
    ) -> Result<DwdWindLayer, Grib2Error> {
        DisciplineChecker::check(
            &east_wind_doc,
            Discipline::Meteorological,
            MeteoParameterCategory::Momentum,
            2
        )?;
        DisciplineChecker::check(
            &north_wind_doc,
            Discipline::Meteorological,
            MeteoParameterCategory::Momentum,
            3
        )?;

        let east_value_grid = ValueGrid::from_grib2(east_wind_doc)?;
        let north_value_grid = ValueGrid::from_grib2(north_wind_doc)?;
        if east_value_grid.grid.lat_grid_points != north_value_grid.grid.lat_grid_points
            || east_value_grid.grid.lon_grid_points != north_value_grid.grid.lon_grid_points {
            return Err(Grib2Error::InvalidData("Grid sizes don't match".to_string()));
        }

        let layer = DwdWindLayer {
            zonal_value_grid: east_value_grid,
            meridional_value_grid: north_value_grid
        };

        return Ok(layer);
    }
}


impl WindLayer for DwdWindLayer {
    fn get_latlon_grid_points(&self) -> (u32, u32) {
        let lat_points = self.meridional_value_grid.grid.lat_grid_points;
        let lon_points = self.meridional_value_grid.grid.lon_grid_points;

        return (lat_points, lon_points);
    }


    fn get_wind_speed_east_north_m_per_s_by_latlon(&self, pos: &LatLon) -> (f32, f32) {
        let east_value = self.zonal_value_grid.get_value_by_lat_lon(pos);
        let north_value = self.meridional_value_grid.get_value_by_lat_lon(pos);

        return (east_value, north_value);
    }


    fn get_wind_speed_east_north_m_per_s_by_index(&self, index: usize) -> (f32, f32) {
        let east_value = self.zonal_value_grid.get_value_by_index(index);
        let north_value = self.meridional_value_grid.get_value_by_index(index);

        return (east_value, north_value);
    }
}
