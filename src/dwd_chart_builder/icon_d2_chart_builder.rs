use std::fs;
use std::io::{BufReader, Read, Seek};
use std::ops::RangeInclusive;
use bzip2::read::BzDecoder;
use image::io::Reader;
use crate::chart::cloud_precip_chart_renderer::CloudPrecipChartRenderer;

use crate::dwd_chart_builder::icon_d2_forecast_run_finder::IconD2ForecastRunFinder;
use crate::dwd_forecast_runs::icon_d2_forecast_run::IconD2ForecastRun;
use crate::dwd_forecast_runs::icon_d2_forecast_step::IconD2ForecastStep;
use crate::dwd_layer::dwd_cloud_precip_layer::DwdCloudPrecipLayer;
use crate::grib2::document::grib2_document_reader::Grib2DocumentReader;
use crate::grid::regular_grid_converter::RegularGridConverter;
use crate::imaging::drawable::Drawable;

pub struct IconD2ChartBuilder;


impl IconD2ChartBuilder {
    pub fn create_latest_dwd_forecasts() {
        let latest_run = IconD2ForecastRunFinder::find_latest_forecast_run().unwrap(); // TODO

        Self::create_weather_map_tiles(&latest_run);
        Self::create_wind_map_tiles(&latest_run);
    }


    fn create_weather_map_tiles(forecast_run: &IconD2ForecastRun) {
        let base_dir: &str = "./set02/";
        let filename = "TODO";
        let mut reader = Self::get_file_reader(filename);
        // let clct_doc = Grib2DocumentReader::read_stream(&mut reader).unwrap();

        /*for i in IconD2ForecastStep::get_step_range() {
            println!("time step {}", i);

            let nr0 = format!("{:03}", i);
            let nr_m1 = format!("{:03}", i - 1);
            let clct_file = format!("{}{}{}{}", base_dir, clct_file_prefix, &nr0, clct_file_suffix);
            let precip_file0 = format!("{}{}{}{}", base_dir, precip_file_prefix, &nr_m1, precip_file_suffix);
            let precip_file1 = format!("{}{}{}{}", base_dir, precip_file_prefix, &nr0, precip_file_suffix);
            let clct_doc = Grib2DocumentReader::read_file(&clct_file).unwrap();
            let clct_grid = RegularGridConverter::create(&clct_doc, -1.0).unwrap();
            let precip_doc0 = Grib2DocumentReader::read_file(&precip_file0).unwrap();
            let precip_grid0 = RegularGridConverter::create(&precip_doc0, -1.0).unwrap();
            let precip_doc1 = Grib2DocumentReader::read_file(&precip_file1).unwrap();
            let precip_grid1 = RegularGridConverter::create(&precip_doc1, -1.0).unwrap();

            let layer = DwdCloudPrecipLayer::new(clct_grid, precip_grid0, precip_grid1).unwrap();
            let save_dir = format!("{}clct_precip/{}/", &base_dir, &nr0);
            let _ = CloudPrecipChartRenderer::render_map_tiles(
                &layer,
                (0, 7),
                |tile: &Drawable, zoom: u32, x: u32, y: u32| save_tile_step(tile, zoom, x, y, &save_dir)
            );
        }*/
    }


    fn create_wind_map_tiles(forecast_run: &IconD2ForecastRun) {
    }


    fn get_file_reader(filename: &str) -> impl Read {
        let response_result = ureq::get(filename).call().unwrap(); // TODO
        let reader = response_result.into_reader();
        let bz_decoder = BzDecoder::new(reader);

        return bz_decoder;
    }


    fn save_tile_step(
        tile: &Drawable,
        zoom: u32,
        x: u32,
        y: u32,
        step: &str
    ) {
        let base_path = format!("./{}/", step);
        let path = format!("{}/{}/{}", base_path, zoom, x);
        fs::create_dir_all(&path).unwrap();

        let filename = format!("{}/{}.png", &path, y);
        let _result = tile.safe_image(&filename);
    }
}
