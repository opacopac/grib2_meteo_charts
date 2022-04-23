use meteo_grib2_renderer::dwd::cloud_cover::cloud_cover_layer::CloudCoverLayer;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::meteo_chart::cloud_cover_chart_renderer::CloudCoverChartRenderer;

const CLCT_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";

fn main() {
    println!("Hello, world!");
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();
    let ccl = CloudCoverLayer::new(doc).unwrap();
    let img = CloudCoverChartRenderer::create(ccl).unwrap();
    img.safe_image("CCL.png");

    /*let mut i = 0;
    let mut j = 0;
    loop {
        let value = ccl.get_value_by_index(i);
        if value > 0.0 {
            println!("{} {}", i, value);
            j += 1;

            if j > 100 {
                break;
            }
        }
        i += 1;
    }*/
}
