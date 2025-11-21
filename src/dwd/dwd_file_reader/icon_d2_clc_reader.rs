use crate::dwd::dwd_file_reader::icon_d2_file::IconD2File;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use log::info;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::ops::RangeInclusive;


pub struct IconD2ClcReader;


const DWD_ICON_D2_CLC_FILE_PREFIX: &str = "/clc/icon-d2_germany_regular-lat-lon_model-level_";
const DWD_ICON_D2_CLC_FILE_SUFFIX: &str = "_clc.grib2.bz2";
const MISSING_VALUE: u8 = 0;


impl IconD2ClcReader {
    pub fn read_clc_grids2(
        fc_run: &MeteoForecastRun2,
        fc_step: &MeteoForecastRun2Step,
        vertical_level_range: &RangeInclusive<u8>,
    ) -> Result<Vec<LatLonValueGrid<u8>>, Grib2Error> {
        let transform_fn = |x| x as u8;

        info!("reading clc grids...");

        let clc_grids = vertical_level_range.clone()
            .into_par_iter()
            .map(|level| {
                info!("reading clc layers for level {}", level);
                let url = Self::get_file_url2(fc_run, fc_step, level as usize);
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


    fn get_file_url2(
        fc_run: &MeteoForecastRun2,
        fc_step: &MeteoForecastRun2Step,
        level: usize,
    ) -> String {
        IconD2File::get_multi_level_file_url2(
            DWD_ICON_D2_CLC_FILE_PREFIX,
            DWD_ICON_D2_CLC_FILE_SUFFIX,
            level,
            fc_run,
            fc_step,
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::icon_d2_clc_reader::IconD2ClcReader;
    use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
    use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
    use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;
    use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
    use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
    use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
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


    #[test]
    fn it_creates_the_correct_file_url2() {
        // given
        let fc_run = MeteoForecastRun2::new(
            MeteoForecastModel::IconD2,
            NaiveDate::from_ymd_opt(2022, 12, 22).unwrap(),
            "00".to_string(),
        );
        let fc_step = MeteoForecastRun2Step::new(0, "".to_string()); // TODO: get rid of this...

        // when
        let result = IconD2ClcReader::get_file_url2(&fc_run, &fc_step, 65);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/clc/icon-d2_germany_regular-lat-lon_model-level_2022122200_000_65_clc.grib2.bz2";
        assert_eq!(expected, result);
    }
}
