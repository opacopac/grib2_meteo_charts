use meteo_grib2_renderer::grib2::converter::file_to_grid_converter::FileToGridConverter;
use meteo_grib2_renderer::grib2::converter::regular_grid_converter::RegularGridConverter;
use meteo_grib2_renderer::grib2::section3::grid_definition_template::GridDefinitionTemplate::LatitudeLongitude;
use meteo_grib2_renderer::meteo_chart::meteo_layer::meteo_cloud_layer::MeteoCloudLayer;

pub const CLCT_TEST_FILE: &str = "./tests/resources/icon-eu_europe_regular-lat-lon_single-level_2022042700_047_CLCT_MOD.grib2";


#[test]
fn it_successfully_reads_an_icon_eu_clct_test_file() {
    let doc = FileToGridConverter::read_single_doc_from_file_or_url(CLCT_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();

    match doc.section3.grid_definition_template {
        LatitudeLongitude(grid) => {
            println!("{} {}", grid.first_grid_point_lat, grid.first_grid_point_lon);
            println!("{} {}", grid.last_grid_point_lat, grid.last_grid_point_lon);
            println!("{} {}", grid.number_of_points_along_parallel, grid.number_of_points_along_meridian);
            println!("{} {}", grid.i_direction_increment, grid.j_direction_increment);
        }
        _ => {}
    }


    let _layer = MeteoCloudLayer::new(grid);

    assert!(true);
}
