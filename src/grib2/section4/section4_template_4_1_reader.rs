use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section4::meteo_parameter_category_reader::MeteoParameterCategoryReader;
use crate::grib2::section4::product_definition_template_4_1::ProductDefinitionTemplate4_1;
use byteorder::ReadBytesExt;
use std::io::Read;


pub struct Section4Template4_1Reader;


impl Section4Template4_1Reader {
    pub fn read(reader: &mut impl Read) -> Result<ProductDefinitionTemplate4_1, Grib2Error> {
        let parameter_category = MeteoParameterCategoryReader::read(reader)?;
        let parameter_number = reader.read_u8()?;
        let type_of_generating_process = reader.read_u8()?;
        let background_generating_process_identifier = reader.read_u8()?;
        let forecast_generating_process_identifier = reader.read_u8()?;
        let hours_after_reference_time = reader.read_u16::<byteorder::BigEndian>()?;
        let minutes_after_reference_time = reader.read_u8()?;
        let time_range_unit = reader.read_u8()?;
        let forecast_time_in_units_defined_by_octet_18 = reader.read_u32::<byteorder::BigEndian>()?;
        let type_of_first_fixed_surface = reader.read_u8()?;
        let scale_factor_of_first_fixed_surface = reader.read_i8()?;
        let scaled_value_of_first_fixed_surface = reader.read_u32::<byteorder::BigEndian>()?;
        let type_of_second_fixed_surface = reader.read_u8()?;
        let scale_factor_of_second_fixed_surface = reader.read_i8()?;
        let scaled_value_of_second_fixed_surface = reader.read_u32::<byteorder::BigEndian>()?;
        let type_of_ensemble_forecast = reader.read_u8()?;
        let perturbation_number = reader.read_u8()?;
        let number_of_forecasts_in_ensemble = reader.read_u8()?;

        let tpl_4_1 = ProductDefinitionTemplate4_1::new(
            parameter_category,
            parameter_number,
            type_of_generating_process,
            background_generating_process_identifier,
            forecast_generating_process_identifier,
            hours_after_reference_time,
            minutes_after_reference_time,
            time_range_unit,
            forecast_time_in_units_defined_by_octet_18,
            type_of_first_fixed_surface,
            scale_factor_of_first_fixed_surface,
            scaled_value_of_first_fixed_surface,
            type_of_second_fixed_surface,
            scale_factor_of_second_fixed_surface,
            scaled_value_of_second_fixed_surface,
            type_of_ensemble_forecast,
            perturbation_number,
            number_of_forecasts_in_ensemble,
        );

        Ok(tpl_4_1)
    }
}
