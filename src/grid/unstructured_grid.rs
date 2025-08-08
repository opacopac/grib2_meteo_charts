use crate::geo::lat_lon::LatLon;

pub struct UnstructuredGrid {
    dimensions: (usize, usize),
    coordinates: Vec<LatLon>,
}

impl UnstructuredGrid {
    pub fn new(dimensions: (usize, usize), coordinates: Vec<LatLon>) -> UnstructuredGrid {
        UnstructuredGrid {
            dimensions,
            coordinates,
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_extent::LatLonExtent;
    use crate::grid::lat_lon_grid::LatLonGrid;
    use assert_approx_eq::assert_approx_eq;

    fn create_test_grid() -> LatLonGrid {
        let dimensions = (2, 3);
        let lat_lon_extent = LatLonExtent::new(LatLon::new(40.0, 7.0), LatLon::new(46.0, 9.0));

        return LatLonGrid::new(dimensions, lat_lon_extent);
    }

    #[test]
    fn it_creates_a_new_instance_with_the_correct_lat_lon_incs() {
        let grid = create_test_grid();

        assert_eq!(2.0, grid.lat_inc);
        assert_eq!(1.0, grid.lon_inc);
    }
}
*/
