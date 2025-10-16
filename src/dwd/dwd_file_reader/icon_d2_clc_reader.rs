use crate::dwd::dwd_file_reader::icon_d2_file::IconD2File;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_chart::meteo_layer::meteo_vertical_cloud_layer::MeteoVerticalCloudLayer;
use log::info;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::ops::RangeInclusive;


pub struct IconD2ClcReader;


const DWD_ICON_D2_CLC_FILE_PREFIX: &str = "/clc/icon-d2_germany_regular-lat-lon_model-level_";
const DWD_ICON_D2_CLC_FILE_SUFFIX: &str = "_clc.grib2.bz2";
const MISSING_VALUE: u8 = 0;


impl IconD2ClcReader {
    pub fn read_layer_from_file(
        fc_step: &DwdForecastStep,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
        vertical_level_range: &RangeInclusive<u8>,
    ) -> Result<MeteoVerticalCloudLayer, Grib2Error> {
        let clc_grids = Self::read_clc_grids(fc_step, vertical_level_range)?;
        let layer = MeteoVerticalCloudLayer::new(hhl_grids.clone(), clc_grids);

        Ok(layer)
    }


    pub fn read_clc_grids(
        fc_step: &DwdForecastStep,
        vertical_level_range: &RangeInclusive<u8>,
    ) -> Result<Vec<LatLonValueGrid<u8>>, Grib2Error> {
        let transform_fn = |x| x as u8;

        info!("reading clc grids...");

        let clc_grids = vertical_level_range.clone()
            .into_par_iter()
            .map(|level| {
                info!("reading clc layers for level {}", level);
                let url = Self::get_file_url(&fc_step, level as usize);
                let grid = FileToGridConverter::read_rectangular_grid_from_file_and_transform(&url, MISSING_VALUE, transform_fn)?;

                Ok(grid)
            }).collect();

        info!("reading clc grids done");

        clc_grids
    }


    pub fn get_file_url(forecast_step: &DwdForecastStep, level: usize) -> String {
        IconD2File::get_multi_level_file_url(
            DWD_ICON_D2_CLC_FILE_PREFIX,
            DWD_ICON_D2_CLC_FILE_SUFFIX,
            level,
            forecast_step,
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::icon_d2_clc_reader::IconD2ClcReader;
    use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
    use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
    use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;
    use chrono::NaiveDate;


    #[test]
    fn it_creates_the_correct_file_url() {
        let forecast_step = DwdForecastStep::new(
            DwdModelType::IconD2,
            NaiveDate::from_ymd_opt(2022, 12, 22).unwrap(),
            IconD2ForecastRunName::Run00,
            0,
        );
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/clc/icon-d2_germany_regular-lat-lon_model-level_2022122200_000_65_clc.grib2.bz2";

        let result = IconD2ClcReader::get_file_url(&forecast_step, 65);

        assert_eq!(expected, result);
    }
}
