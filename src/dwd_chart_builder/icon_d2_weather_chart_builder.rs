use std::fs::File;
use std::io::{BufWriter, Write};

use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::chart::cloud_precip_chart_renderer::CloudPrecipChartRenderer;
use crate::dwd_chart_builder::icon_d2_chart_builder_helper::IconD2ChartBuilderHelper;
use crate::dwd_files::icon_d2_file_ceiling::IconD2FileCeiling;
use crate::dwd_files::icon_d2_file_clct_mod::IconD2FileClctMod;
use crate::dwd_files::icon_d2_file_tot_prec::IconD2FileTotPrec;
use crate::dwd_files::icon_d2_file_ww::IconD2FileWw;
use crate::dwd_forecast_runs::dwd_forecast_run::DwdForecastRun;
use crate::dwd_forecast_runs::dwd_forecast_step::DwdForecastStep;
use crate::dwd_layer::dwd_cloud_precip_layer::DwdCloudPrecipLayer;
use crate::dwd_layer::dwd_weather_layer::DwdWeatherLayer;
use crate::grib2::document::grib2_document_reader::Grib2DocumentReader;
use crate::grid::regular_grid_converter::RegularGridConverter;
use crate::imaging::drawable::Drawable;
use crate::metobin::dwd_weather_metobin::DwdWeatherMeteoBin;

pub struct IconD2WeatherChartBuilder;


const WEATHER_LAYER: &str = "clct_precip";


impl IconD2WeatherChartBuilder {
    pub fn create_weather_map_tiles(forecast_run: &DwdForecastRun) {
        DwdForecastStep::get_step_range().into_par_iter().for_each(|step| {
            info!("creating weather charts, time step {}", step);

            let fc_step = DwdForecastStep::new_from_run(forecast_run, step);
            let fc_previous_step = DwdForecastStep::new_from_run(forecast_run, step - 1);

            // map tiles
            let clct_file = IconD2FileClctMod::get_file_url(&fc_step);
            let mut clct_reader = IconD2ChartBuilderHelper::get_file_reader(&clct_file);
            let clct_doc = Grib2DocumentReader::read_stream(&mut clct_reader).unwrap();
            let clct_grid = RegularGridConverter::create(&clct_doc, -1.0).unwrap();

            let precip_file0 = IconD2FileTotPrec::get_file_url(&fc_previous_step);
            let mut precip_reader0 = IconD2ChartBuilderHelper::get_file_reader(&precip_file0);
            let precip_doc0 = Grib2DocumentReader::read_stream(&mut precip_reader0).unwrap();
            let precip_grid0 = RegularGridConverter::create(&precip_doc0, -1.0).unwrap();

            let precip_file1 = IconD2FileTotPrec::get_file_url(&fc_step);
            let mut precip_reader1 = IconD2ChartBuilderHelper::get_file_reader(&precip_file1);
            let precip_doc1 = Grib2DocumentReader::read_stream(&mut precip_reader1).unwrap();
            let precip_grid1 = RegularGridConverter::create(&precip_doc1, -1.0).unwrap();

            let layer = DwdCloudPrecipLayer::new(clct_grid, precip_grid0, precip_grid1).unwrap();

            let _ = CloudPrecipChartRenderer::render_map_tiles(
                &layer,
                (0, 7),
                |tile: &Drawable, zoom: u32, x: u32, y: u32| IconD2ChartBuilderHelper::save_tile_step(tile, zoom, x, y, WEATHER_LAYER, &fc_step)
            );


            // meteobin
            let ww_file = IconD2FileWw::get_file_url(&fc_step);
            let mut ww_reader = IconD2ChartBuilderHelper::get_file_reader(&ww_file);
            let ww_doc = Grib2DocumentReader::read_stream(&mut ww_reader).unwrap();
            let ww_grid = RegularGridConverter::create(&ww_doc, -1.0).unwrap();

            let ceiling_file = IconD2FileCeiling::get_file_url(&fc_step);
            let mut ceiling_reader = IconD2ChartBuilderHelper::get_file_reader(&ceiling_file);
            let ceiling_doc = Grib2DocumentReader::read_stream(&mut ceiling_reader).unwrap();
            let ceiling_grid = RegularGridConverter::create(&ceiling_doc, -1.0).unwrap();

            let weather_layer = DwdWeatherLayer::new(ww_grid, ceiling_grid).unwrap();

            let weather_bin = DwdWeatherMeteoBin::new(weather_layer);
            let data = weather_bin.create_bin_values();
            let filename = format!(
                "{}WW_D2.meteobin",
                IconD2ChartBuilderHelper::get_output_path(&fc_step, WEATHER_LAYER),
            );
            let mut file = BufWriter::new(File::create(&filename).expect("Unable to create file"));
            let _ = file.write_all(&data);
        });
    }
}
