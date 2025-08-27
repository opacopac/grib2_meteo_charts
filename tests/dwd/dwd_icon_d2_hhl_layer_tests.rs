use assert_approx_eq::assert_approx_eq;

use meteo_grib2_renderer::meteo_layer::meteo_hhl_layer::MeteoHhlLayer;
use meteo_grib2_renderer::geo::lat_lon::LatLon;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grib2::converter::regular_grid_converter::RegularGridConverter;

pub const HHL_TEST_FILE: &str = "./tests/resources/icon-d2_germany_regular-lat-lon_time-invariant_2022122100_000_66_hhl.grib2";


#[test]
fn it_successfully_reads_a_hhl_test_file() {
    let doc = Grib2DocumentReader::read_single_doc_from_file(HHL_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = MeteoHhlLayer::new(grid);

    let pos1 = LatLon { lat: 47.0, lon: 7.0 }; // lac de neuchatel
    let height1 = layer.get_height_by_lat_lon(&pos1).unwrap();
    assert_approx_eq!(429.28, height1, 5.0);

    let pos2 = LatLon { lat: 56.84, lon: 19.18 }; // ostsee
    let height2 = layer.get_height_by_lat_lon(&pos2).unwrap();
    assert_approx_eq!(0.0, height2, 5.0);
}
