use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grib2::section3::grid_definition_template::GridDefinitionTemplate::LatitudeLongitude;
use meteo_grib2_renderer::meteo_layer::wind_layer::DwdWindLayer;
use meteo_grib2_renderer::grib2::converter::regular_grid_converter::RegularGridConverter;

pub const WIND_U_TEST_FILE: &str = "./tests/resources/icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_u_10m.grib2";
pub const WIND_V_TEST_FILE: &str = "./tests/resources/icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_v_10m.grib2";
pub const WIND_V_EU_TEST_FILE: &str = "./tests/resources/icon-eu_europe_regular-lat-lon_single-level_2022050700_000_V_10M.grib2";

#[test]
fn it_successfully_creates_a_wind_test_file_from_wind_u_and_v_grib_docs() {
    let doc_u = Grib2DocumentReader::read_single_doc_from_file(WIND_U_TEST_FILE).unwrap();
    let doc_v = Grib2DocumentReader::read_single_doc_from_file(WIND_V_TEST_FILE).unwrap();
    let grid_u = RegularGridConverter::create(&doc_u, -1.0).unwrap();
    let grid_v = RegularGridConverter::create(&doc_v, -1.0).unwrap();

    match doc_u.section3.grid_definition_template {
        LatitudeLongitude(grid) => {
            println!("{} {}", grid.first_grid_point_lat, grid.first_grid_point_lon);
            println!("{} {}", grid.last_grid_point_lat, grid.last_grid_point_lon);
            println!("{} {}", grid.number_of_points_along_parallel, grid.number_of_points_along_meridian);
        }
        _ => {}
    }

    let layer = DwdWindLayer::new(grid_u, grid_v, None).unwrap();
    println!("{:?}", layer.get_grid_dimensions());
    println!("{:?}", layer.get_lat_lon_extent());

    assert!(true);
}


#[test]
fn it_returns_an_error_when_the_grid_sizes_dont_match() {
    let doc_u = Grib2DocumentReader::read_single_doc_from_file(WIND_U_TEST_FILE).unwrap();
    let doc_v = Grib2DocumentReader::read_single_doc_from_file(WIND_V_EU_TEST_FILE).unwrap();
    let grid_u = RegularGridConverter::create(&doc_u, -1.0).unwrap();
    let grid_v = RegularGridConverter::create(&doc_v, -1.0).unwrap();

    let result = DwdWindLayer::new(grid_u, grid_v, None);

    assert!(result.is_err());
}
