use crate::meteo_common::meteo_forecast_renderer_helper::MeteoForecastFileHelper;
use crate::meteo_chart::meteo_layer::meteo_layer_type::MeteoLayerType;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::metobin::meteobin_type::MeteobinType;
use std::fs::File;
use std::io::{BufWriter, Write};


pub struct MeteobinFileHelper {}


impl MeteobinFileHelper {
    pub fn create_meteobin_file(
        bin_data: &Vec<u8>,
        fc_run: &dyn MeteoForecastRun,
        fc_step: usize,
        layer_type: &MeteoLayerType,
        meteobin_type: &MeteobinType,
    ) {
        let filename = format!(
            "{}{}",
            MeteoForecastFileHelper::get_output_path(fc_run, fc_step, layer_type),
            meteobin_type.get_output_file()
        );
        let mut file = BufWriter::new(File::create(&filename).expect("Unable to create wind meteobin file"));
        let _ = file.write_all(&bin_data);
    }
}
