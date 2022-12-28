use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::RangeInclusive;

use log::info;

use crate::dwd_chart_builder::icon_d2_chart_builder_helper::IconD2ChartBuilderHelper;
use crate::dwd_files::icon_d2_file_clc::IconD2FileClc;
use crate::dwd_files::icon_d2_file_hhl::IconD2FileHhl;
use crate::dwd_forecast_runs::dwd_forecast_run::DwdForecastRun;
use crate::dwd_forecast_runs::dwd_forecast_step::DwdForecastStep;
use crate::dwd_layer::dwd_vertical_cloud_layer::DwdVerticalCloudLayer;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::metobin::dwd_vertical_cloud_metobin::DwdVerticalCloudMeteobin;

pub struct IconD2VerticalCloudChartBuilder;

const VERTICAL_CLOUDS_SUB_DIR: &str = "vertical_clouds";
const VERTICAL_LEVEL_RANGE: RangeInclusive<u8> = 25..=65; //25..=65;
const FEET_PER_M: f32 = 3.28084;


impl IconD2VerticalCloudChartBuilder {
    pub fn create_wind_charts(forecast_run: &DwdForecastRun) {
        info!("reading hhl grids...");
        let hhl_grids = Self::read_hhl_grids(forecast_run);
        info!("{} hhl grids found", hhl_grids.len());

        DwdForecastStep::get_step_range().for_each(|step| {
            info!("creating vertical cloud charts, time step {}", step);
            let fc_step = DwdForecastStep::new_from_run(forecast_run, step);
            info!("reading clc grids...");
            let clc_grids = Self::read_clc_grids(&fc_step);
            info!("{} clc grids found", clc_grids.len());
            let vertical_cloud_layer = DwdVerticalCloudLayer::new(&hhl_grids, clc_grids);

            // meteobin
            let vert_cloud_bin = DwdVerticalCloudMeteobin::new(vertical_cloud_layer);
            let data = vert_cloud_bin.create_bin_values();
            let path = IconD2ChartBuilderHelper::get_output_path(&fc_step, VERTICAL_CLOUDS_SUB_DIR);
            let filename = format!("{}VERTICAL_CLOUDS_D2.meteobin", path);

            fs::create_dir_all(&path).unwrap();
            let mut file = BufWriter::new(File::create(&filename).expect(&*format!("Unable to create vertical clouds meteobin file {}", &filename)));
            let _ = file.write_all(&data);
        });
    }


    fn read_hhl_grids(forecast_run: &DwdForecastRun) -> Vec<LatLonValueGrid<u8>> {
        let mut hhl_grids = vec![];
        for level in VERTICAL_LEVEL_RANGE {
            info!("reading hhl layers for level {}", level);
            let hhl_grid = IconD2FileHhl::read_grid_from_file_and_convert(
                forecast_run,
                level as usize,
                0,
                |x| (x * FEET_PER_M / 100.0) as u8
            ).unwrap();

            hhl_grids.push(hhl_grid);
        };

        return hhl_grids;
    }


    fn read_clc_grids(fc_step: &DwdForecastStep) -> Vec<LatLonValueGrid<u8>> {
        let mut clc_grids = vec![];
        for level in VERTICAL_LEVEL_RANGE {
            info!("reading clc layers for level {}", level);
            let clc_grid = IconD2FileClc::read_grid_from_file_and_convert(
                &fc_step,
                level as usize,
                0,
                |x| x as u8
            ).unwrap();

            clc_grids.push(clc_grid);
        };

        return clc_grids;
    }
}
