use std::f32::consts::PI;
use crate::geo::lat_lon::LatLon;

const POW2: usize = 8;
const CELLS: usize = 1 << POW2;


pub struct MapTileGrid {
    value: [[u8; CELLS]; CELLS]
}


impl MapTileGrid {
    pub fn new() -> MapTileGrid {
        return MapTileGrid {
            value: [[0; CELLS]; CELLS]
        }
    }


    pub fn get_value(&self, x: usize, y: usize) -> u8 {
        return self.value[x][y];
    }


    pub fn set_value(&mut self, pos: &LatLon, value: u8) {
        let (x, y) = Self::calc_xy_from_latlon(pos);
        self.value[x][y] = value;
    }


    pub fn calc_xy_from_latlon(lat_lon: &LatLon) -> (usize, usize) {
        let pow = CELLS as f32;
        let x = ((lat_lon.lon + 180.0) / 360.0 * pow).floor() as usize;
        let y = ((1.0 - (lat_lon.lat.to_radians().tan() + 1.0 / lat_lon.lat.to_radians().cos()).ln() / PI) / 2.0 * pow).floor() as usize;

        return (x, y);
    }
}


#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use crate::geo::lat_lon::LatLon;
    use crate::geo::map_tile_grid::{CELLS, MapTileGrid, POW2};

    #[test]
    fn it_has_the_correct_number_of_cells() {
        let expected = 2_u32.pow(POW2 as u32);

        assert_eq!(expected, CELLS as u32);
    }


    #[test]
    fn it_reads_the_default_value_from_a_cell() {
        let grid = MapTileGrid::new();

        let result = grid.get_value(0, 0);

        assert_eq!(0, result);
    }


    #[test]
    fn it_calculates_the_correct_cell_xy_from_latlon() {
        let lat_limit = PI.sinh().atan().to_degrees();
        let pos1 = LatLon { lat: 0.0, lon: 0.0 };
        let pos2 = LatLon { lat: lat_limit, lon: -180.0 };
        let pos3 = LatLon { lat: -lat_limit, lon: 180.0 };

        let xy1 = MapTileGrid::calc_xy_from_latlon(&pos1);
        let xy2 = MapTileGrid::calc_xy_from_latlon(&pos2);
        let xy3 = MapTileGrid::calc_xy_from_latlon(&pos3);

        assert_eq!((CELLS / 2, CELLS / 2), xy1);
        assert_eq!((0, 0), xy2);
        assert_eq!((CELLS, CELLS), xy3);
    }


    #[test]
    fn it_adds_a_value_to_the_correct_target_cell() {
        let pos1 = LatLon { lat: 0.0, lon: 0.0 };
        let mut grid = MapTileGrid::new();

        grid.set_value(&pos1, 99);

        let x = CELLS / 2;
        let y = x;
        let result = grid.get_value(x, y);

        assert_eq!(99, result);
    }
}
