use std::f32::consts::PI;
use std::ops::AddAssign;

use crate::geo::lat_lon::LatLon;


pub struct MapTileGrid<T: Copy, const CELLS: usize> {
    pub value: Vec<Vec<T>>
}


impl <T: Copy + AddAssign, const CELLS: usize> MapTileGrid<T, CELLS> {
    pub fn new(default_value: T) -> MapTileGrid<T, CELLS> {
        if !Self::is_power_of_2(CELLS) {
            panic!("grid size must be power of 2!");
        }

        return MapTileGrid {
            value: vec![vec![default_value; CELLS]; CELLS]
        }
    }


    pub fn get_value(&self, x: usize, y: usize) -> T {
        return self.value[x][y];
    }


    pub fn set_value(&mut self, pos: &LatLon, value: T) {
        let (x, y) = Self::calc_xy_from_latlon(pos);
        self.value[x][y] = value;
    }


    pub fn add_value(&mut self, pos: &LatLon, value: T) {
        let (x, y) = Self::calc_xy_from_latlon(pos);
        self.value[x][y] += value;
    }


    fn is_power_of_2(num: usize) -> bool {
        return (num as f32).log2().floor() == (num as f32).log2().ceil();
    }


    fn calc_xy_from_latlon(lat_lon: &LatLon) -> (usize, usize) {
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
    use crate::geo::map_tile_grid::MapTileGrid;

    #[test]
    #[should_panic]
    fn it_panics_if_cells_is_not_a_power_of_two() {
        let _: MapTileGrid<u8, 127> = MapTileGrid::new(0);
    }


    #[test]
    fn it_reads_the_default_value_from_a_cell() {
        let grid: MapTileGrid<u8, 256> = MapTileGrid::new(0);

        let result = grid.get_value(0, 0);

        assert_eq!(0, result);
    }


    #[test]
    fn it_calculates_the_correct_cell_xy_from_latlon() {
        let lat_limit = PI.sinh().atan().to_degrees();
        let pos1 = LatLon { lat: 0.0, lon: 0.0 };
        let pos2 = LatLon { lat: lat_limit, lon: -180.0 };
        let pos3 = LatLon { lat: -lat_limit, lon: 180.0 };

        let xy1 = MapTileGrid::<u8, 256>::calc_xy_from_latlon(&pos1);
        let xy2 = MapTileGrid::<u8, 256>::calc_xy_from_latlon(&pos2);
        let xy3 = MapTileGrid::<u8, 256>::calc_xy_from_latlon(&pos3);

        assert_eq!((128, 128), xy1);
        assert_eq!((0, 0), xy2);
        assert_eq!((256, 256), xy3);
    }


    #[test]
    fn it_adds_a_value_to_the_correct_target_cell() {
        let pos1 = LatLon { lat: 0.0, lon: 0.0 };
        let mut grid: MapTileGrid<u8, 256> = MapTileGrid::new(0);

        grid.set_value(&pos1, 99);

        let x = 128;
        let y = x;
        let result = grid.get_value(x, y);

        assert_eq!(99, result);
    }
}
