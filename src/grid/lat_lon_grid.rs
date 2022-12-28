use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;

pub struct LatLonGrid {
    dimensions: (usize, usize),
    lat_lon_extent: LatLonExtent,
    lat_inc: f32,
    lon_inc: f32,
}


impl LatLonGrid {
    pub fn new(
        dimensions: (usize, usize),
        lat_lon_extent: LatLonExtent
    ) -> LatLonGrid {
        let lat_inc = (lat_lon_extent.max_coord.lat - lat_lon_extent.min_coord.lat) / dimensions.1 as f32;
        let lon_inc = (lat_lon_extent.max_coord.lon - lat_lon_extent.min_coord.lon) / dimensions.0 as f32;

        return LatLonGrid { dimensions, lat_lon_extent, lat_inc, lon_inc };
    }


    pub fn get_dimensions(&self) -> (usize, usize) {
        return self.dimensions.clone();
    }


    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        return &self.lat_lon_extent;
    }


    pub fn get_index_by_x_y(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.dimensions.0 || y >= self.dimensions.1 {
            return None
        }

        let idx = x + y * self.dimensions.0;

        return Some(idx);
    }


    pub fn get_x_y_by_lat_lon(&self, pos: &LatLon) -> Option<(f32, f32)> {
        if !self.lat_lon_extent.is_inside(pos) {
            return None;
        }

        let x = (pos.lon - &self.lat_lon_extent.min_coord.lon) / &self.lon_inc;
        let y = (pos.lat - &self.lat_lon_extent.min_coord.lat) / &self.lat_inc;

        return Some((x, y));
    }
}


#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_extent::LatLonExtent;
    use crate::grid::lat_lon_grid::LatLonGrid;

    fn create_test_grid() -> LatLonGrid {
        let dimensions = (2, 3);
        let lat_lon_extent = LatLonExtent::new(
            LatLon::new(40.0, 7.0),
            LatLon::new(46.0, 9.0)
        );

        return LatLonGrid::new(dimensions, lat_lon_extent);
    }


    #[test]
    fn it_creates_a_new_instance_with_the_correct_lat_lon_incs() {
        let grid = create_test_grid();

        assert_eq!(2.0, grid.lat_inc);
        assert_eq!(1.0, grid.lon_inc);
    }


    #[test]
    fn it_gets_the_correct_dimensions() {
        let grid = create_test_grid();

        let result = grid.get_dimensions();

        assert_eq!((2, 3), result);
    }


    #[test]
    fn it_gets_the_correct_lat_lon_extent() {
        let grid = create_test_grid();

        let result = grid.get_lat_lon_extent();

        assert_eq!([40.0, 7.0], result.min_coord.as_array());
        assert_eq!([46.0, 9.0], result.max_coord.as_array());
    }


    #[test]
    fn it_gets_the_correct_index_by_x_y() {
        let grid = create_test_grid();

        let result1 = grid.get_index_by_x_y(0, 0);
        let result2 = grid.get_index_by_x_y(1, 0);
        let result3 = grid.get_index_by_x_y(0, 1);
        let result4 = grid.get_index_by_x_y(1, 2);

        assert!(result1.is_some());
        assert!(result2.is_some());
        assert!(result3.is_some());
        assert!(result4.is_some());

        assert_eq!(0, result1.unwrap());
        assert_eq!(1, result2.unwrap());
        assert_eq!(2, result3.unwrap());
        assert_eq!(5, result4.unwrap());
    }


    #[test]
    fn it_gets_none_index_if_x_y_are_out_of_bounds() {
        let grid = create_test_grid();

        let result1 = grid.get_index_by_x_y(2, 0);
        let result2 = grid.get_index_by_x_y(0, 3);

        assert!(result1.is_none());
        assert!(result2.is_none());
    }


    #[test]
    fn it_gets_the_correct_x_y_by_lat_lon() {
        let grid = create_test_grid();
        let pos1 = LatLon::new(40.0, 7.0);
        let pos2 = LatLon::new(42.0, 7.0);
        let pos3 = LatLon::new(40.0, 8.0);
        let pos4 = LatLon::new(44.0, 8.0);
        let pos4b = LatLon::new(45.9, 8.9);

        let result1 = grid.get_x_y_by_lat_lon(&pos1);
        let result2 = grid.get_x_y_by_lat_lon(&pos2);
        let result3 = grid.get_x_y_by_lat_lon(&pos3);
        let result4 = grid.get_x_y_by_lat_lon(&pos4);
        let result4b = grid.get_x_y_by_lat_lon(&pos4b);

        assert!(result1.is_some());
        assert!(result2.is_some());
        assert!(result3.is_some());
        assert!(result4.is_some());
        assert!(result4b.is_some());

        assert_eq!((0.0, 0.0), result1.unwrap());
        assert_eq!((0.0, 1.0), result2.unwrap());
        assert_eq!((1.0, 0.0), result3.unwrap());
        assert_eq!((1.0, 2.0), result4.unwrap());
        assert_approx_eq!(1.90, result4b.unwrap().0, 0.01);
        assert_approx_eq!(2.95, result4b.unwrap().1, 0.01);
    }


    #[test]
    fn it_gets_none_if_lat_or_lon_are_out_of_bounds() {
        let grid = create_test_grid();
        let pos1 = LatLon::new(40.0, 6.9);
        let pos2 = LatLon::new(39.9, 7.0);
        let pos3 = LatLon::new(43.0, 9.1);
        let pos4 = LatLon::new(46.1, 8.0);
        let pos5 = LatLon::new(46.0, 8.0);

        let result1 = grid.get_x_y_by_lat_lon(&pos1);
        let result2 = grid.get_x_y_by_lat_lon(&pos2);
        let result3 = grid.get_x_y_by_lat_lon(&pos3);
        let result4 = grid.get_x_y_by_lat_lon(&pos4);
        let result5 = grid.get_x_y_by_lat_lon(&pos5);

        assert!(result1.is_none());
        assert!(result2.is_none());
        assert!(result3.is_none());
        assert!(result4.is_none());
        assert!(result5.is_none());
    }
}
