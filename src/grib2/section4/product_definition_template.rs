use crate::grib2::section4::product_definition_template_4_0::ProductDefinitionTemplate4_0;

pub enum ProductDefinitionTemplate {
    Template4_0(ProductDefinitionTemplate4_0),
    Missing,
    Unknown(u16),
}
