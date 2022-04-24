use std::thread::sleep;
use std::time::{Duration, Instant};

use meteo_grib2_renderer::dwd::cloud_cover::cloud_cover_layer::CloudCoverLayer;
use meteo_grib2_renderer::geo::map_tile_coord::MapTileCoord;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::meteo_chart::cloud_cover_chart_renderer::CloudCoverChartRenderer;

const CLCT_TEST_FILE: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042312_000_2d_clct_mod.grib2";
//const CLCT_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";

fn main() {
    create_img();
    create_map_tile();
}


fn create_img() {
    let start = Instant::now();

    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();
    let elapsed = start.elapsed();
    println!("read doc {}", elapsed.as_millis());

    let ccl = CloudCoverLayer::new(doc).unwrap();
    let elapsed = start.elapsed();
    println!("create ccl {}", elapsed.as_millis());

    let img = CloudCoverChartRenderer::create_single_chart(&ccl).unwrap();
    let elapsed = start.elapsed();
    println!("create img {}", elapsed.as_millis());

    img.safe_image("CCL.png").unwrap();
    let elapsed = start.elapsed();
    println!("save img {}", elapsed.as_millis());
}


fn create_map_tile() {
    let start = Instant::now();

    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();
    let elapsed = start.elapsed();
    println!("read doc {}", elapsed.as_millis());

    let ccl = CloudCoverLayer::new(doc).unwrap();
    let elapsed = start.elapsed();
    println!("create ccl {}", elapsed.as_millis());

    //let map_tile_coord = MapTileCoord::new(1070, 718, 11);
    let map_tile_coord = MapTileCoord::new(535, 359, 10);
    //let map_tile_coord = MapTileCoord::new(33, 22, 6);
    //let map_tile_coord = MapTileCoord::new(0, 0, 0);
    //let img = CloudCoverChartRenderer::create_single_tile(&ccl, &map_tile_coord).unwrap();
    //img.safe_image("CCL_TILE.png").unwrap();

    CloudCoverChartRenderer::create_all_tiles(&ccl, (0, 12), "./");
    let elapsed = start.elapsed();
    println!("create img {}", elapsed.as_millis());

    let elapsed = start.elapsed();
    println!("save img {}", elapsed.as_millis());
}

