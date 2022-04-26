use crate::dwd::discipline_checker::DisciplineChecker;
use crate::dwd::value_grid::ValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section0::discipline::Discipline;
use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;

pub struct DwdPrecipLayer {
    pub value_grid: ValueGrid
}


impl DwdPrecipLayer {
    pub fn from_grib2(
        document: Grib2Document
    ) -> Result<DwdPrecipLayer, Grib2Error> {
        DisciplineChecker::check(
            &document,
            Discipline::Meteorological,
            MeteoParameterCategory::Moisture
        )?;

        let value_grid = ValueGrid::from_grib2(document)?;
        let layer = DwdPrecipLayer { value_grid };

        return Ok(layer);
    }
}
