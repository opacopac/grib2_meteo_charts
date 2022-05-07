use crate::geo::lat_lon::LatLon;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section0::discipline::Discipline;
use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
use crate::meteo::precip_layer::PrecipLayer;
use crate::meteo_dwd::discipline_checker::DisciplineChecker;
use crate::meteo_dwd::value_grid::ValueGrid;

pub struct DwdPrecipLayer {
    pub value_grid: ValueGrid
}


impl DwdPrecipLayer {
    pub fn from_grib2(document: Grib2Document) -> Result<DwdPrecipLayer, Grib2Error> {
        DisciplineChecker::check(
            &document,
            Discipline::Meteorological,
            MeteoParameterCategory::Moisture,
            52
        )?;

        let value_grid = ValueGrid::from_grib2(document)?;
        let layer = DwdPrecipLayer { value_grid };

        return Ok(layer);
    }
}


impl PrecipLayer for DwdPrecipLayer {
    fn get_latlon_grid_points(&self) -> (u32, u32) {
        let lat_points = self.value_grid.grid.lat_grid_points;
        let lon_points = self.value_grid.grid.lon_grid_points;

        return (lat_points, lon_points);
    }


    fn get_precip_rate_kg_per_m2_per_s_by_latlon(&self, pos: &LatLon) -> f32 {
        let value = self.value_grid.get_value_by_lat_lon(pos);

        return value;
    }


    fn get_precip_rate_kg_per_m2_per_s_by_index(&self, index: usize) -> f32 {
        let value = self.value_grid.get_value_by_index(index);

        return value;
    }
}
