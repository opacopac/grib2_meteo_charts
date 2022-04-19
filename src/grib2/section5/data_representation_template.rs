use crate::grib2::section5::data_representation_template_5_0::DataRepresentationTemplate5_0;

pub enum DataRepresentationTemplate {
    GridPointDataSimplePacking(DataRepresentationTemplate5_0),
    Missing,
    Unknown(u16),
}
