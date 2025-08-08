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

    pub fn new_with_mbr(dimensions: (usize, usize), coordinates: Vec<LatLon>) -> UnstructuredGrid {
        let lat_lon_extent = UnstructuredGrid::calc_min_bounding_extent(&coordinates);
        UnstructuredGrid {
            dimensions,
            coordinates,
            lat_lon_extent,
        }
    }

    fn calc_min_bounding_extent(coordinates: &Vec<LatLon>) -> LatLonExtent {
        let mut min_lat = LatLon::MAX_LAT;
        let mut max_lat = LatLon::MIN_LAT;
        let mut min_lon = LatLon::MAX_LON;
        let mut max_lon = LatLon::MIN_LON;

        for coord in coordinates {
            if coord.lat < min_lat {
                min_lat = coord.lat;
            }
            if coord.lat > max_lat {
                max_lat = coord.lat;
            }
            if coord.lon < min_lon {
                min_lon = coord.lon;
            }
            if coord.lon > max_lon {
                max_lon = coord.lon;
            }
        }

        LatLonExtent::new(LatLon::new(min_lat, min_lon), LatLon::new(max_lat, max_lon))
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
        assert_eq!((2,3), grid.dimensions);
        assert_eq!(3, grid.coordinates.len());
        assert_eq!(LatLonExtent::MAX_EXTENT, grid.lat_lon_extent);
    }


    #[test]
    fn it_creates_a_new_instance_with_min_bounding_extent() {
        // given
        let dimensions = (3, 3);
        let coordinates = vec![
            LatLon::new(40.0, 7.0),
            LatLon::new(50.0, 6.0),
            LatLon::new(45.0, 9.0),
        ];

        // when
        let grid = super::UnstructuredGrid::new_with_mbr(dimensions, coordinates);

        // then
        assert_eq!((3,3), grid.dimensions);
        assert_eq!(3, grid.coordinates.len());
        assert_eq!(LatLonExtent::new(LatLon::new(40.0, 6.0), LatLon::new(50.0, 9.0)), grid.lat_lon_extent);
    }


    #[test]
    fn it_calculates_min_bounding_extent() {
        // given
        let coordinates = vec![
            LatLon::new(40.0, 7.0),
            LatLon::new(50.0, 6.0),
            LatLon::new(45.0, 9.0),
        ];

        // when
        let extent = super::UnstructuredGrid::calc_min_bounding_extent(&coordinates);

        // then
        assert_eq!(LatLon::new(40.0, 6.0), extent.min_coord);
        assert_eq!(LatLon::new(50.0, 9.0), extent.max_coord);
    }
}
