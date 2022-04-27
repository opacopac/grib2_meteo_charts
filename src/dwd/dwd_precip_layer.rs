use crate::dwd::discipline_checker::DisciplineChecker;
use crate::dwd::value_grid::ValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section0::discipline::Discipline;
use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;

pub struct DwdPrecipLayer {
    pub parameter_category: MeteoParameterCategory,
    pub parameter_number: u8,
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

        println!("{}", document.section1.length);
        println!("{}", document.section2.length);
        println!("{}", document.section3.length);
        println!("{}", document.section4.length);
        println!("{}", document.section5.length);
        println!("{}", document.section6.length);

        println!("{}", document.section3.section_number);
        println!("{}", document.section3.number_of_datapoints);
        println!("{}", document.section5.section_number);
        println!("{}", document.section5.data_points);
        println!("{}", document.section6.section_number);
        println!("{}", document.section6.bitmap_indicator);
        println!("{}", document.section6.length);
        println!("{}", document.section6.bitmap.len());
        println!("{}", document.section7.section_number);
        println!("{}", document.section7.data_points.len());
        println!("{}", document.section7.length);

        let parameter_cat_num = DisciplineChecker::get_parameter_category_number(&document)?;
        let value_grid = ValueGrid::from_grib2(document)?;
        let layer = DwdPrecipLayer {
            parameter_category: parameter_cat_num.0,
            parameter_number: parameter_cat_num.1,
            value_grid
        };

        return Ok(layer);
    }


    pub fn color_by_value(value: f32) -> [u8; 4] {
        let u8_value = (value  * 255.0).floor() as u8;

        return [0, 127, 255, u8_value]; // TODO
    }
}
