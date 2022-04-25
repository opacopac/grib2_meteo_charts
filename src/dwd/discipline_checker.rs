use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section0::discipline::Discipline;
use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
use crate::grib2::section4::product_definition_template::ProductDefinitionTemplate;

pub struct DisciplineChecker;


impl DisciplineChecker {
    pub fn check(
        document: &Grib2Document,
        expected_discipline: Discipline,
        expected_parameter_category: MeteoParameterCategory
    ) -> Result<(), Grib2Error> {
        if document.section0.discipline != expected_discipline {
            return Err(Grib2Error::InvalidData(
                format!("invalid discipline '{:?}'", document.section0.discipline)
            ));
        }

        match &document.section4.product_definition_template {
            ProductDefinitionTemplate::Template4_0(tpl) => {
                if let MeteoParameterCategory::Cloud = expected_parameter_category {
                } else {
                    return Err(Grib2Error::InvalidData(
                        format!("invalid parameter category '{:?}'", tpl.parameter_category)
                    ));
                }
            },
            _ => return Err(Grib2Error::InvalidData(
                format!("invalid product definition template '{:?}'", document.section4.product_definition_template)
            ))
        }

        return Ok(());
    }
}
