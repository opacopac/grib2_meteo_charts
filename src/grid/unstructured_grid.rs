use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grid::lat_lon_grid::LatLonGrid;

pub struct UnstructuredGrid {
    lat_lon_grid: LatLonGrid,
    coordinates: Vec<LatLon>,
}

impl UnstructuredGrid {
    pub fn new(
        dimensions: (usize, usize),
        coordinates: Vec<LatLon>,
        lat_lon_extent: LatLonExtent,
    ) -> UnstructuredGrid {
        UnstructuredGrid {
            lat_lon_grid: LatLonGrid::new(dimensions, lat_lon_extent),
            coordinates,
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        self.lat_lon_grid.get_dimensions()
    }

    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        &self.lat_lon_grid.get_lat_lon_extent()
    }
}

#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_extent::LatLonExtent;

    #[test]
    fn it_creates_a_new_instance() {
        // given
        let dimensions = (2, 3);
        let coordinates = vec![
            LatLon::new(40.0, 7.0),
            LatLon::new(41.0, 7.0),
            LatLon::new(42.0, 7.0),
        ];
        let lat_lon_extent = LatLonExtent::MAX_EXTENT;

        // when
        let grid = super::UnstructuredGrid::new(dimensions, coordinates, lat_lon_extent);

        // then
        assert_eq!((2, 3), grid.get_dimensions());
        assert_eq!(3, grid.coordinates.len());
        assert_eq!(&LatLonExtent::MAX_EXTENT, grid.get_lat_lon_extent());
    }
}
