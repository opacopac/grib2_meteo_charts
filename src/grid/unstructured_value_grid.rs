use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grid::grid_value_type::GridValueType;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grid::unstructured_grid::UnstructuredGrid;
use std::ops::{Add, Mul};

pub struct UnstructuredValueGrid<T> {
    grid: UnstructuredGrid,
    values: Vec<T>,
    missing_value: T,
}

impl<T: GridValueType + Mul<f32, Output = T> + Add<Output = T> + std::iter::Sum>
    UnstructuredValueGrid<T>
{
    pub fn new(
        values: Vec<T>,
        missing_value: T,
        grid: UnstructuredGrid,
    ) -> UnstructuredValueGrid<T> {
        UnstructuredValueGrid {
            grid,
            values,
            missing_value,
        }
    }

    pub fn get_grid(&self) -> &UnstructuredGrid {
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
        let coord_dist_triple = self.grid.get_coord_dist_triple(x, y)?;
        let coord_dists = coord_dist_triple.get_coord_dists();

        if coord_dists.is_empty() {
            return None;
        }

        let coord_dist_sum: f32 = coord_dists
            .iter()
            .map(|cd| cd.get_coord_dist_squared().sqrt())
            .sum();

        let value: T = coord_dists
            .iter()
            .filter_map(|cd| {
                let value_index = cd.get_coord_index();
                return if let Some(value) = self.values.get(value_index) {
                    if *value == self.missing_value {
                        return None;
                    }
                    Some(*value * (cd.get_coord_dist_squared().sqrt() / coord_dist_sum))
                } else {
                    None
                }
            })
            .sum();

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
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_extent::LatLonExtent;
    use crate::grid::unstructured_grid::UnstructuredGrid;
    use crate::grid::unstructured_value_grid::UnstructuredValueGrid;

    /*
     * ----------------
     * |    | 30 |    |
     * ----------------
     * | 20 |    |    |
     * ----------------
     * |    | 10 |    |
     * ----------------
     */
    fn create_test_value_grid() -> UnstructuredValueGrid<f32> {
        let dimensions = (3, 3);
        let lat_lon_extent = LatLonExtent::new(
            LatLon::new(0.0, 0.0),
            LatLon::new(3.0, 3.0)
        );
        let values = vec![10.0, 20.0, 30.0];
        let missing_value = 255.0;
        let coordinates = vec![
            LatLon::new(1.5, 0.5),
            LatLon::new(0.5, 1.5),
            LatLon::new(1.5, 2.5),
        ];
        let mut grid = UnstructuredGrid::new(dimensions, lat_lon_extent, coordinates);
        grid.calc_coord_dist_lookup_map(1.1);

        UnstructuredValueGrid::new(values, missing_value, grid)
    }

    #[test]
    fn it_gets_the_correct_x_y_value() {
        // given
        let uv_grid = create_test_value_grid();

        // when
        let grid = uv_grid.create_regular_grid();

        let result00 = grid.get_value_by_xy(0, 0).unwrap();
        let result11 = grid.get_value_by_xy(1, 1).unwrap();
        let result22 = grid.get_value_by_xy(2, 2).unwrap();
        let result21 = grid.get_value_by_xy(2, 1).unwrap();

        assert_eq!(15.0, result00);
        assert_eq!(20.0, result11);
        assert_eq!(30.0, result22);
        assert_eq!(255.0, result21);
    }
}
