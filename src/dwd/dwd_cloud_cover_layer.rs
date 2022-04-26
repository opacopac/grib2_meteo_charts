use crate::dwd::discipline_checker::DisciplineChecker;
use crate::dwd::value_grid::ValueGrid;
use crate::geo::lat_lon::LatLon;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section0::discipline::Discipline;
use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;

pub struct DwdCloudCoverLayer {
    pub value_grid: ValueGrid
}


impl DwdCloudCoverLayer {
    pub fn from_grib2(document: Grib2Document) -> Result<DwdCloudCoverLayer, Grib2Error> {
        DisciplineChecker::check(
            &document,
            Discipline::Meteorological,
            MeteoParameterCategory::Cloud
        )?;

        let value_grid = ValueGrid::from_grib2(document)?;
        let layer = DwdCloudCoverLayer { value_grid };

        return Ok(layer);
    }


    pub fn get_value_by_index(&self, index: usize) -> f32 {
        return self.value_grid.get_value_by_index(index);
    }


    pub fn get_index_by_lat_lon(&self, pos: &LatLon) -> usize {
        return self.value_grid.get_index_by_lat_lon(pos);
    }


    pub fn get_value_by_lat_lon(&self, pos: &LatLon) -> f32 {
        return self.value_grid.get_value_by_lat_lon(pos);
    }


}
