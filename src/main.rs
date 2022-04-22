use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;

const CLCT_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";

fn main() {
    println!("Hello, world!");
    let _layer = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();
}
