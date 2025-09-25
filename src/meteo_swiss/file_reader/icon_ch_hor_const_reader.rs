use crate::geo::common::lat_lon::LatLon;
use crate::geo::common::lat_lon_extent::LatLonExtent;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::grib2::converter::grib2_to_grid_converter::Grib2ToGridConverter;
use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;


pub struct IconHorConstReader;


impl IconHorConstReader {
    const IMG_DIMENSIONS: (usize, usize) = (1851, 847); // (1024, 1024);
    const MAX_COORD_DIST_DEG: f32 = 0.01;
    const MIN_LAT: f32 = 42.03; // 42.0279274;
    const MAX_LAT: f32 = 50.50; // 50.5005836;
    const MIN_LON: f32 = -0.81; // -0.817148566;
    const MAX_LON: f32 = 17.70; // 17.7106838;


    pub fn read_grid_from_file(file_url: &str) -> Result<UnstructuredGrid, MeteoSwissError> {
        let hor_docs = FileToGridConverter::read_multi_doc_from_file_or_url(file_url)?;
        let clat_doc = &hor_docs[4];
        let clon_doc = &hor_docs[3];
        let lat_lon_extent = LatLonExtent::new(
            LatLon::new(Self::MIN_LAT, Self::MIN_LON),
            LatLon::new(Self::MAX_LAT, Self::MAX_LON),
        );
        let coordinates = Grib2ToGridConverter::get_lat_lon_values_from_grib_doc(clat_doc, clon_doc)?;
        let mut grid = UnstructuredGrid::new(Self::IMG_DIMENSIONS, lat_lon_extent, coordinates);
        grid.calc_coord_dist_lookup_map(Self::MAX_COORD_DIST_DEG);

        Ok(grid)
    }
}
