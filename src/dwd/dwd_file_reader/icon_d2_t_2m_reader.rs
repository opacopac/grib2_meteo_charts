use crate::dwd::dwd_file_reader::icon_d2_file::IconD2File;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_chart::meteo_layer::meteo_temp_2m_layer::MeteoTemp2mLayer;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct IconD2T2mReader;


const DWD_ICON_D2_T_2M_FILE_PREFIX: &str = "/t_2m/icon-d2_germany_regular-lat-lon_single-level_";
const DWD_ICON_D2_T_2M_FILE_SUFFIX: &str = "_2d_t_2m.grib2.bz2";
const MISSING_VALUE: f32 = -1.0;


impl IconD2T2mReader {
    pub fn read_layer(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> Result<MeteoTemp2mLayer, Grib2Error> {
        let grid = FileToGridConverter::read_rectangular_grid_from_file(
            Self::get_file_url(fc_run, fc_step).as_str(),
            MISSING_VALUE,
        )?;
        let layer = MeteoTemp2mLayer::new(grid);

        Ok(layer)
    }
    
    
    pub fn get_file_url(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> String {
        IconD2File::get_single_level_file_url(
            DWD_ICON_D2_T_2M_FILE_PREFIX,
            DWD_ICON_D2_T_2M_FILE_SUFFIX,
            fc_run,
            fc_step,
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::icon_d2_t_2m_reader::IconD2T2mReader;
    use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
    use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
    use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
    use chrono::NaiveDate;

    #[test]
    fn it_creates_the_correct_file_url() {
        // given
        let fc_run = MeteoForecastRun::new(
            MeteoForecastModel::IconD2,
            NaiveDate::from_ymd_opt(2023, 8, 6).unwrap(),
            "00".to_string(),
        );
        let fc_step = MeteoForecastRunStep::new(0, "".to_string()); // TODO: get rid of this...

        // when
        let result = IconD2T2mReader::get_file_url(&fc_run, &fc_step);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/t_2m/icon-d2_germany_regular-lat-lon_single-level_2023080600_000_2d_t_2m.grib2.bz2";
        assert_eq!(expected, result);
    }
}
