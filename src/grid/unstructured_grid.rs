use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;

pub struct UnstructuredGrid {
    dimensions: (usize, usize),
    coordinates: Vec<LatLon>,
    lat_lon_extent: LatLonExtent,
}

impl UnstructuredGrid {
    pub fn new(
        dimensions: (usize, usize),
        coordinates: Vec<LatLon>,
        lat_lon_extent: LatLonExtent,
    ) -> UnstructuredGrid {
        UnstructuredGrid {
            dimensions,
            coordinates,
            lat_lon_extent,
        }
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
        assert_eq!((2, 3), grid.dimensions);
        assert_eq!(3, grid.coordinates.len());
        assert_eq!(LatLonExtent::MAX_EXTENT, grid.lat_lon_extent);
    }
}
