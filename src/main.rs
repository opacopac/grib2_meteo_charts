#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::Instant;
use rand::Rng;

use meteo_grib2_renderer::chart::cloud_chart_renderer::CloudChartRenderer;
use meteo_grib2_renderer::chart::map_tile_renderer::MapTileRenderer;
use meteo_grib2_renderer::chart::precip_chart_renderer::PrecipChartRenderer;
use meteo_grib2_renderer::chart::wind_chart_renderer::WindChartRenderer;
use meteo_grib2_renderer::geo::lat_lon::LatLon;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::meteo_dwd::dwd_cloud_layer::DwdCloudLayer;
use meteo_grib2_renderer::meteo_dwd::dwd_icon_d2_tot_cloud_cover_layer::DwdIconD2TotalCloudCoverLayer;
use meteo_grib2_renderer::meteo_dwd::dwd_icon_global_grid_reader::DwdIconGlobalGridReader;
use meteo_grib2_renderer::meteo_dwd::dwd_icon_global_tot_cloud_cover_layer::DwdIconGlobalTotalCloudCoverLayer;
use meteo_grib2_renderer::meteo_dwd::dwd_precip_layer::DwdPrecipLayer;
use meteo_grib2_renderer::meteo_dwd::dwd_wind_layer::DwdWindLayer;

const CLCT_TEST_FILE_D2: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_clct_mod.grib2";
const CLCT_TEST_FILE_EU: &str = "icon-eu_europe_regular-lat-lon_single-level_2022042700_047_CLCT_MOD.grib2";
const CLCT_TEST_FILE_GLOBAL: &str = "icon_global_icosahedral_single-level_2022051300_000_CLCT_MOD.grib2";
const PRECIP_TEST_FILE: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042700_048_2d_tot_prec.grib2";
//const WIND_U_TEST_FILE: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_u_10m.grib2";
//const WIND_V_TEST_FILE: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_v_10m.grib2";
const WIND_U_TEST_FILE: &str = "icon-eu_europe_regular-lat-lon_single-level_2022051015_000_U_10M.grib2";
const WIND_V_TEST_FILE: &str = "icon-eu_europe_regular-lat-lon_single-level_2022051015_000_V_10M.grib2";
const NETCDF_ICON_GRID_TEST_FILE: &str = "icon_grid_0026_R03B07_G.nc";

fn main() {
    //create_icon_d2_precip_img();
    //create_icon_d2_clct_img();
    //create_icon_eu_clct_img();
    //create_icon_d2_wind_img();
    //create_icon_d2_wind_map_tile();

    //create_icon_d2_map_tile();
    //create_icon_d2_map_tile_series();
    perf_icon_global();
}


fn perf_icon_global() {
    let grib2_doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_D2).unwrap();
    let grid = DwdIconGlobalGridReader::create(NETCDF_ICON_GRID_TEST_FILE).unwrap();
    let layer = DwdIconGlobalTotalCloudCoverLayer::create(grib2_doc, grid).unwrap();

    let mut rng = rand::thread_rng();
    let start = Instant::now();
    for _ in 0..1000000 {
        let pos = &LatLon::new(rng.gen::<f32>() * 180.0 - 90.0, rng.gen::<f32>() * 360.0 - 180.0);
        let (point, value) = layer.grid.find_closest_point_value(pos);
        //println!("CH point {}: {:?}", value, point);
    }
    println!("reading from grid: {}", start.elapsed().as_millis());
}


fn create_icon_d2_map_tile_series() {
    let file_prefix: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042415_";
    let file_suffix: &str = "_2d_clct_mod.grib2";

    for i in 0..=7 {
        let nr = format!("{:03}", i);
        let file = format!("{}{}{}", file_prefix, &nr, file_suffix);
        //println!("{}", file);

        let doc = Grib2DocumentReader::read_file(&file).unwrap();
        let ccl = DwdIconD2TotalCloudCoverLayer::from_grib2(doc).unwrap();
        let dir = &format!("./{}/", &nr);
        let _result = MapTileRenderer::create_all_tiles(
            &ccl.value_grid,
            (0, 7),
            dir,
            DwdIconD2TotalCloudCoverLayer::color_by_value
        );
    }
}


fn create_icon_d2_clct_img() {
    let start = Instant::now();

    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_D2).unwrap();
    let elapsed = start.elapsed();
    println!("read doc {}", elapsed.as_millis());

    let layer = DwdCloudLayer::from_grib2(doc).unwrap();
    let elapsed = start.elapsed();
    println!("create ccl {}", elapsed.as_millis());

    let img = CloudChartRenderer::render(&layer).unwrap();
    let elapsed = start.elapsed();
    println!("create img {}", elapsed.as_millis());

    img.safe_image("CLCT.png").unwrap();
    let elapsed = start.elapsed();
    println!("save img {}", elapsed.as_millis());
}


fn create_icon_eu_clct_img() {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_EU).unwrap();
    let layer = DwdCloudLayer::from_grib2(doc).unwrap();
    let img = CloudChartRenderer::render(&layer).unwrap();
    img.safe_image("CLCT_EU.png").unwrap();
}


fn create_icon_d2_precip_img() {
    let doc = Grib2DocumentReader::read_file(PRECIP_TEST_FILE).unwrap();
    let layer = DwdPrecipLayer::from_grib2(doc).unwrap();
    let img = PrecipChartRenderer::render(&layer).unwrap();
    img.safe_image("PRECIP.png").unwrap();
}


fn create_icon_d2_wind_img() {
    let doc_u = Grib2DocumentReader::read_file(WIND_U_TEST_FILE).unwrap();
    let doc_v = Grib2DocumentReader::read_file(WIND_V_TEST_FILE).unwrap();
    let layer = DwdWindLayer::from_grib2(doc_u, doc_v).unwrap();
    let img = WindChartRenderer::render_full_chart(&layer).unwrap();
    img.safe_image("WIND.png").unwrap();
}


fn create_icon_d2_map_tile() {
    let start = Instant::now();

    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_D2).unwrap();
    let elapsed = start.elapsed();
    println!("read doc {}", elapsed.as_millis());

    let ccl = DwdIconD2TotalCloudCoverLayer::from_grib2(doc).unwrap();
    let elapsed = start.elapsed();
    println!("create ccl {}", elapsed.as_millis());

    //let map_tile_coord = MapTileCoord::new(1070, 718, 11);
    //let map_tile_coord = MapTileCoord::new(535, 359, 10);
    //let map_tile_coord = MapTileCoord::new(33, 22, 6);
    //let map_tile_coord = MapTileCoord::new(0, 0, 0);
    //let img = CloudCoverChartRenderer::create_single_tile(&ccl, &map_tile_coord).unwrap();
    //img.safe_image("CCL_TILE.png").unwrap();

    let _result = MapTileRenderer::create_all_tiles(
        &ccl.value_grid,
        (0, 7),
        "./007/",
        DwdIconD2TotalCloudCoverLayer::color_by_value
    );
    let elapsed = start.elapsed();
    println!("create img {}", elapsed.as_millis());

    let elapsed = start.elapsed();
    println!("save img {}", elapsed.as_millis());
}


fn create_icon_d2_wind_map_tile() {
    let doc_u = Grib2DocumentReader::read_file(WIND_U_TEST_FILE).unwrap();
    let doc_v = Grib2DocumentReader::read_file(WIND_V_TEST_FILE).unwrap();
    let layer = DwdWindLayer::from_grib2(doc_u, doc_v).unwrap();
    let _result = WindChartRenderer::create_all_tiles(
        &layer,
        (0, 11),
        "./007/"
    );
}

