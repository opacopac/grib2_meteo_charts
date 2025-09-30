use crate::meteo_chart::meteo_layer::meteo_layer_type::MeteoLayerType;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;


pub struct MeteoForecastFileHelper;


const FORECAST_BASE_DIR: &str = "./output/";
const FC_STARTDATE_FORMAT: &str = "%Y%m%d";


impl MeteoForecastFileHelper {
    pub fn get_output_path(
        fc_run: &dyn MeteoForecastRun,
        fc_step: usize,
        layer_type: &MeteoLayerType,
    ) -> String {
        format!(
            "{}{}/{}{}/{:03}/{}/",
            FORECAST_BASE_DIR,
            fc_run.get_model_name(),
            fc_run.get_start_date().format(FC_STARTDATE_FORMAT),
            fc_run.get_name(),
            fc_step,
            &layer_type.get_output_subdir(),
        )
    }
}
