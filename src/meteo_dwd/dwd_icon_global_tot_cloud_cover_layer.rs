use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section0::discipline::Discipline;
use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
use crate::meteo_dwd::discipline_checker::DisciplineChecker;

pub struct DwdIconGlobalTotalCloudCoverLayer {
    pub parameter_category: MeteoParameterCategory,
    pub parameter_number: u8,
}


impl DwdIconGlobalTotalCloudCoverLayer {
    pub fn from_grib2(document: Grib2Document) -> Result<DwdIconGlobalTotalCloudCoverLayer, Grib2Error> {
        DisciplineChecker::check(
            &document,
            Discipline::Meteorological,
            MeteoParameterCategory::Cloud,
            199
        )?;

        let parameter_cat_num = DisciplineChecker::get_parameter_category_number(&document)?;
        /*let value_grid = ValueGrid::from_grib2(document)?;*/
        let layer = DwdIconGlobalTotalCloudCoverLayer {
            parameter_category: parameter_cat_num.0,
            parameter_number: parameter_cat_num.1,
        };

        return Ok(layer);
    }
}
