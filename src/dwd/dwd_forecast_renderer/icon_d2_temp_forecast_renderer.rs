use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::dwd::dwd_file_reader::icon_d2_t_2m_reader::IconD2T2mReader;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_renderer_helper::IconD2ForecastRendererHelper;
use crate::meteo_layer::temp_layer::DwdTempLayer;
use crate::metobin::dwd_temp_metobin::DwdTempMeteoBin;

pub struct IconD2TempForecastRenderer;

const TEMP_LAYER: &str = "temp";


impl IconD2TempForecastRenderer {
    pub fn create(forecast_run: &DwdForecastRun) -> Result<(), ForecastRendererError> {
        DwdForecastStep::get_step_range()
            .into_par_iter()
            .try_for_each(|step| {
                info!("creating temperature charts, time step {}", step);

                let fc_step = DwdForecastStep::new_from_run(forecast_run, step);
                let temp_grid = IconD2T2mReader::read_grid_from_file(&fc_step)?;
                let layer = DwdTempLayer::new(temp_grid)?;

                // map tiles
                /*let _ = WindChartRenderer::render_map_tiles(
                    &layer,
                    (0, 7),
                    |tile: &Drawable, zoom: u32, x: u32, y: u32| IconD2ForecastRendererHelper::save_tile_step(tile, zoom, x, y, WIND_LAYER, &fc_step)
                );*/

                // meteobin
                let temp_bin = DwdTempMeteoBin::new(layer);
                let data = temp_bin.create_bin_values();

                let path = IconD2ForecastRendererHelper::get_output_path(&fc_step, TEMP_LAYER);
                fs::create_dir_all(&path).unwrap();

                let filename = format!(
                    "{}TEMP_D2.meteobin",
                    &path,
                );
                let mut file = BufWriter::new(File::create(&filename).expect("Unable to create temperature meteobin file"));
                let _ = file.write_all(&data);

                Ok(())
            })
    }
}
