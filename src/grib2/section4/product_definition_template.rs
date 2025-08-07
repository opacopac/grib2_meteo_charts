use crate::grib2::section4::product_definition_template_4_0::ProductDefinitionTemplate4_0;
use crate::grib2::section4::product_definition_template_4_1::ProductDefinitionTemplate4_1;
use crate::grib2::section4::product_definition_template_4_8::ProductDefinitionTemplate4_8;

#[derive(Debug)]
pub enum ProductDefinitionTemplate {
    Template4_0(ProductDefinitionTemplate4_0),
    Template4_1(ProductDefinitionTemplate4_1),
    Template4_8(ProductDefinitionTemplate4_8),
    Missing,
    Unknown(u16),
}
