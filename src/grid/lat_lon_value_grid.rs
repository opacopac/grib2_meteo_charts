use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grid::lat_lon_grid::LatLonGrid;

pub struct LatLonValueGrid<T> {
    grid: LatLonGrid,
    values: Vec<T>,
    missing_value: T
}


impl <T: Copy + PartialEq> LatLonValueGrid<T> {
    pub fn new(
        values: Vec<T>,
        missing_value: T,
        dimensions: (usize, usize),
        lat_lon_extent: LatLonExtent
    ) -> LatLonValueGrid<T> {
        let grid = LatLonGrid::new(dimensions, lat_lon_extent);

        return LatLonValueGrid { grid, values, missing_value };
    }


    pub fn get_grid(&self) -> &LatLonGrid {
        return &self.grid;
    }


    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        return self.grid.get_dimensions();
    }


    pub fn get_grid_lat_lon_extent(&self) -> &LatLonExtent {
        return self.grid.get_lat_lon_extent();
    }


    pub fn get_missing_value(&self) -> T {
        return self.missing_value;
    }


    pub fn get_value_by_xy(&self, x: usize, y: usize) -> Option<T> {
        let idx = self.grid.get_index_by_x_y(x, y)?;
        let value = self.values[idx];

        return if value != self.missing_value {
            Some(value)
        } else {
            None
        }
    }


    pub fn get_value_by_lat_lon(&self, pos: &LatLon) -> Option<T> {
        let (x0, y0) = self.grid.get_x_y_by_lat_lon(pos)?;

        let x = x0.round() as usize;
        let y = y0.round() as usize;

        return self.get_value_by_xy(x, y);
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_extent::LatLonExtent;
    use crate::grid::lat_lon_value_grid::LatLonValueGrid;

    fn create_test_grid() -> LatLonValueGrid<f32> {
        let values = vec![00.0, 01.0, 10.0, 11.0, -1.0, 21.0];
        let missing_value = -1.0;
        let dimensions = (2, 3);
        let lat_lon_extent = LatLonExtent::new(
            LatLon::new(40.0, 7.0),
            LatLon::new(46.0, 9.0)
        );

        return LatLonValueGrid::new(values, missing_value, dimensions, lat_lon_extent);
    }


    #[test]
    fn it_gets_the_correct_x_y_value() {
        let grid = create_test_grid();

        let result00 = grid.get_value_by_xy(0, 0).unwrap();
        let result01 = grid.get_value_by_xy(1, 0).unwrap();
        let result21 = grid.get_value_by_xy(1, 2).unwrap();

        assert_eq!(00.0, result00);
        assert_eq!(01.0, result01);
        assert_eq!(21.0, result21);
    }


    #[test]
    fn it_gets_none_if_x_or_y_are_out_of_bounds() {
        let grid = create_test_grid();

        let result1 = grid.get_value_by_xy(2, 0);
        let result2 = grid.get_value_by_xy(0, 3);

        assert!(result1.is_none());
        assert!(result2.is_none());
    }


    #[test]
    fn it_gets_the_correct_lat_lon_value() {
        let grid = create_test_grid();
        let pos1 = LatLon::new(40.0, 7.0);
        let pos2 = LatLon::new(42.0, 7.0);
        let pos3 = LatLon::new(44.0, 8.0);
        let pos4 = LatLon::new(44.9, 8.4);

        let result1 = grid.get_value_by_lat_lon(&pos1).unwrap();
        let result2 = grid.get_value_by_lat_lon(&pos2).unwrap();
        let result3 = grid.get_value_by_lat_lon(&pos3).unwrap();
        let result4 = grid.get_value_by_lat_lon(&pos4).unwrap();

        assert_eq!(00.0, result1);
        assert_eq!(10.0, result2);
        assert_eq!(21.0, result3);
        assert_eq!(21.0, result4);
    }


    #[test]
    fn it_gets_none_if_lat_or_lon_are_out_of_bounds() {
        let grid = create_test_grid();
        let pos1 = LatLon::new(40.0, 6.9);
        let pos2 = LatLon::new(39.9, 7.0);
        let pos3 = LatLon::new(43.0, 9.1);
        let pos4 = LatLon::new(46.1, 8.0);
        let pos5 = LatLon::new(45.5, 8.5);

        let result1 = grid.get_value_by_lat_lon(&pos1);
        let result2 = grid.get_value_by_lat_lon(&pos2);
        let result3 = grid.get_value_by_lat_lon(&pos3);
        let result4 = grid.get_value_by_lat_lon(&pos4);
        let result5 = grid.get_value_by_lat_lon(&pos5);

        assert!(result1.is_none());
        assert!(result2.is_none());
        assert!(result3.is_none());
        assert!(result4.is_none());
        assert!(result5.is_none());
    }


    #[test]
    fn it_gets_none_for_missing_values() {
        let grid = create_test_grid();

        let result1 = grid.get_value_by_xy(0, 2);

        assert!(result1.is_none());
    }
}
