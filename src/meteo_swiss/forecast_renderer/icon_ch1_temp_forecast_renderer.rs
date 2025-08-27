use crate::chart::temp_chart_renderer::TempChartRenderer;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::imaging::drawable::Drawable;
use crate::meteo_layer::meteo_temp_layer::MeteoTempLayer;
use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_renderer_helper::IconCh1ForecastRendererHelper;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use crate::metobin::temp_metobin::TempMeteoBin;
use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

const TEMP_LAYER: &str = "temp";


pub struct IconCh1TempForecastRenderer;


impl IconCh1TempForecastRenderer {
    pub fn create(forecast_run: &IconChForecastRun) -> Result<(), MeteoSwissError> {
        forecast_run.get_step_range()
            .into_par_iter()
            .try_for_each(|step_idx| {
                info!("creating temperature charts, time step {}", step_idx);

                let fc_step = &forecast_run.steps[step_idx];
                let missing_value = 999.0;
                let temp_grid = FileToGridConverter::read_grid_from_file(&fc_step.href, missing_value)?;

                let layer = MeteoTempLayer::new(temp_grid)?;

                // map tiles
                let zoom_range = (0, 7);
                let save_fn = |tile: &Drawable, zoom: u32, x: u32, y: u32| IconCh1ForecastRendererHelper::save_tile_step(
                    tile, zoom, x, y, TEMP_LAYER, forecast_run, step_idx,
                );
                let _ = TempChartRenderer::render_map_tiles(
                    &layer,
                    zoom_range,
                    save_fn,
                );

                // meteobin
                let temp_bin = TempMeteoBin::new(layer);
                let data = temp_bin.create_bin_values();

                let path = IconCh1ForecastRendererHelper::get_output_path(forecast_run, step_idx, TEMP_LAYER);
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
