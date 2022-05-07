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
        expected_parameter_category: MeteoParameterCategory,
        expected_parameter_number: u8
    ) -> Result<(), Grib2Error> {
        if document.section0.discipline != expected_discipline {
            return Err(Grib2Error::InvalidData(
                format!("invalid discipline '{:?}'", document.section0.discipline)
            ));
        }

        let parameter_cat_num = Self::get_parameter_category_number(document)?;
        if parameter_cat_num.0 != expected_parameter_category {
            return Err(Grib2Error::InvalidData(
                format!("invalid parameter category '{:?}'", parameter_cat_num.0)
            ));
        }
        if parameter_cat_num.1 != expected_parameter_number {
            return Err(Grib2Error::InvalidData(
                format!("invalid parameter number '{:?}'", parameter_cat_num.1)
            ));
        }

        return Ok(());
    }


    pub fn get_parameter_category_number(document: &Grib2Document) -> Result<(MeteoParameterCategory, u8), Grib2Error> {
        match &document.section4.product_definition_template {
            ProductDefinitionTemplate::Template4_0(tpl) => {
                return Ok((tpl.parameter_category.clone(), tpl.parameter_number));
            },
            ProductDefinitionTemplate::Template4_8(tpl) => {
                return Ok((tpl.parameter_category.clone(), tpl.parameter_number));
            },
            _ => return Err(Grib2Error::InvalidData(
                format!("invalid product definition template '{:?}'", document.section4.product_definition_template)
            ))
        }
    }
}
