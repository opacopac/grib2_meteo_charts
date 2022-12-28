use std::ops::{Add, Mul};

use crate::geo::lat_lon::LatLon;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;

pub struct LatLonValueGridInterpolator;


impl LatLonValueGridInterpolator {
    pub fn interpolate<T: Copy + PartialEq + Mul<f32, Output = T> + Add<Output = T>>(
        value_grid: &LatLonValueGrid<T>,
        pos: &LatLon
    ) -> Option<T> {
        let grid = value_grid.get_grid();
        let dimensions = grid.get_dimensions();
        let (x, y) = grid.get_x_y_by_lat_lon(pos)?;
        let x_floor_f32 = x.floor();
        let y_floor_f32 = y.floor();
        let x_ceil_f32 = x.ceil();
        let y_ceil_f32 = y.ceil();

        if x_ceil_f32 < 0.0 || y_ceil_f32 < 0.0 || x_floor_f32 >= dimensions.0 as f32 || y_floor_f32 >= dimensions.1 as f32 {
            return None;
        }

        let x_floor = x_floor_f32 as usize;
        let y_floor = y_floor_f32 as usize;
        let x_ceil = x_ceil_f32 as usize;
        let y_ceil = y_ceil_f32 as usize;

        if x_floor == x_ceil || y_floor == y_ceil {
            return value_grid.get_value_by_xy(x_floor, y_floor);
        }

        let val_tl = value_grid.get_value_by_xy(x_floor, y_floor);
        let val_tr = value_grid.get_value_by_xy(x_ceil, y_floor);
        let val_bl = value_grid.get_value_by_xy(x_floor, y_ceil);
        let val_br = value_grid.get_value_by_xy(x_ceil, y_ceil);
        let val_t = Self::interpolate_value(val_tl, x_ceil_f32 - x, val_tr, x - x_floor_f32);
        let val_b = Self::interpolate_value(val_bl, x_ceil_f32 - x, val_br, x - x_floor_f32);
        let value = Self::interpolate_value(val_t, y_ceil_f32 - y, val_b, y - y_floor_f32);

        return value;
    }


    fn interpolate_value<T: Copy + PartialEq + Mul<f32, Output = T> + Add<Output = T>>(
        value1: Option<T>,
        weight1: f32,
        value2: Option<T>,
        weight2: f32
    ) -> Option<T> {
        if value1.is_none() || value2.is_none() {
            return None;
        }

        let val1 = value1.unwrap(); // TODO
        let val2 = value2.unwrap(); // TODO
        let value = val1 * weight1 + val2 * weight2;

        return Some(value);
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
}
