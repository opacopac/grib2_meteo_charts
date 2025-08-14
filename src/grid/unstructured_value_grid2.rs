use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grid::grid_value_type::GridValueType;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grid::unstructured_grid2::UnstructuredGrid2;
use std::ops::{Add, Mul};

pub struct UnstructuredValueGrid2<T> {
    grid: UnstructuredGrid2,
    values: Vec<T>,
    missing_value: T,
}

impl<T: GridValueType + Mul<f32, Output = T> + Add<Output = T> + std::iter::Sum>
    UnstructuredValueGrid2<T>
{
    pub fn new(
        values: Vec<T>,
        missing_value: T,
        grid: UnstructuredGrid2,
    ) -> UnstructuredValueGrid2<T> {
        UnstructuredValueGrid2 {
            grid,
            values,
            missing_value,
        }
    }

    pub fn get_grid(&self) -> &UnstructuredGrid2 {
        &self.grid
    }

    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        self.grid.get_dimensions()
    }

    pub fn get_grid_lat_lon_extent(&self) -> &LatLonExtent {
        self.grid.get_lat_lon_extent()
    }

    pub fn get_missing_value(&self) -> T {
        self.missing_value
    }

    pub fn get_value_by_xy(&self, x: usize, y: usize) -> Option<T> {
        let coord_dist = match self.grid.get_coord_dist(x, y) {
            Some(coord_dist) => coord_dist,
            None => return None,
        };

        let value_index = coord_dist.get_coord_index();
        let value = match self.values.get(value_index) {
            Some(val) => *val,
            None => return None,
        };

        Some(value)
    }

    pub fn get_value_by_lat_lon(&self, pos: &LatLon) -> Option<T> {
        let (x0, y0) = self.grid.get_x_y_by_lat_lon(pos)?;

        let x = x0.round() as usize;
        let y = y0.round() as usize;

        self.get_value_by_xy(x, y)
    }

    pub fn create_regular_grid(&self) -> LatLonValueGrid<T> {
        let lat_lon_extent = self.grid.get_lat_lon_extent().clone();
        let dimensions = self.grid.get_dimensions();
        let values: Vec<T> = (0..dimensions.0 * dimensions.1)
            .map(|i| {
                let x = i % dimensions.0;
                let y = i / dimensions.0;
                self.get_value_by_xy(x, y).unwrap_or(self.missing_value)
            })
            .collect();

        LatLonValueGrid::new(values, self.missing_value, dimensions, lat_lon_extent)
    }
}

#[cfg(test)]
mod tests {
    /*use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_extent::LatLonExtent;
    use crate::grid::lat_lon_value_grid::LatLonValueGrid;

    fn create_test_grid() -> LatLonValueGrid<f32> {
        let values = vec![00.0, 01.0, 10.0, 11.0, -1.0, 21.0];
        let missing_value = -1.0;
        let dimensions = (2, 3);
        let lat_lon_extent = LatLonExtent::new(LatLon::new(40.0, 7.0), LatLon::new(46.0, 9.0));

        LatLonValueGrid::new(values, missing_value, dimensions, lat_lon_extent)
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
    }*/
}
