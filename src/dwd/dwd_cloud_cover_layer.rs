use crate::dwd::discipline_checker::DisciplineChecker;
use crate::dwd::value_grid::ValueGrid;
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


    pub fn color_by_value(value: f32) -> [u8; 4] {
        let u8_value = (value  * 255.0).floor() as u8;

        return [255, 255, 255, u8_value]; // TODO
    }
}
