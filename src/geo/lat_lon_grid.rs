use derive_new::new;
use min_max::{max, min};

use crate::geo::lat_lon::LatLon;

#[derive(Debug, new)]
pub struct LatLonGrid {
    pub start_pos: LatLon,
    pub end_pos: LatLon,
    pub lat_inc_deg: f32,
    pub lon_inc_deg: f32,
    pub lat_grid_points: u32,
    pub lon_grid_points: u32
}


impl LatLonGrid {
    pub fn is_pos_inside(&self, pos: &LatLon) -> bool {
        if self.start_pos.lat < self.end_pos.lat {
            if pos.lat < self.start_pos.lat || pos.lat > self.end_pos.lat {
                return false;
            }
        } else {
            if pos.lat < self.end_pos.lat || pos.lat > self.start_pos.lat {
                return false;
            }
        }

        if self.start_pos.lon < self.end_pos.lon {
            if pos.lon < self.start_pos.lon || pos.lon > self.end_pos.lon {
                return false;
            }
        } else {
            if pos.lon < self.end_pos.lon || pos.lon > self.start_pos.lon {
                return false;
            }
        }

        return true;
    }


    pub fn get_min_pos(&self) -> LatLon {
        let lat = min(self.start_pos.lat, self.end_pos.lat);
        let lon = min(self.start_pos.lon, self.end_pos.lon);

        return LatLon::new(lat, lon);
    }


    pub fn get_max_pos(&self) -> LatLon {
        let lat = max(self.start_pos.lat, self.end_pos.lat);
        let lon = max(self.start_pos.lon, self.end_pos.lon);

        return LatLon::new(lat, lon);
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_grid::LatLonGrid;

    #[test]
    fn it_correctly_detects_if_a_pos_is_inside() {
        let grid = LatLonGrid::new(
            LatLon::new(47.0, 7.0),
            LatLon::new(48.0, 8.0),
            0.1, 0.1, 10, 10
        );

        let result = grid.is_pos_inside(&LatLon::new(47.5, 7.5));
        assert_eq!(true, result);

        let result = grid.is_pos_inside(&LatLon::new(48.1, 7.5));
        assert_eq!(false, result);

        let result = grid.is_pos_inside(&LatLon::new(47.5, 6.9));
        assert_eq!(false, result);
    }


    #[test]
    fn it_correctly_detects_if_a_pos_is_inside_across_0_deg_line() {
        let grid = LatLonGrid::new(
            LatLon::new(-20.0, 330.0),
            LatLon::new(33.0, 44.0),
            0.1, 0.1, 10, 10
        );

        let result = grid.is_pos_inside(&LatLon::new(-10.0, 340.0));
        assert_eq!(true, result);

        let result = grid.is_pos_inside(&LatLon::new(-21.0, 0.0));
        assert_eq!(false, result);

        let result = grid.is_pos_inside(&LatLon::new(0.0, 320.0));
        assert_eq!(false, result);
    }


    #[test]
    fn it_calculates_the_correct_min_max_pos() {
        let grid = LatLonGrid::new(
            LatLon::new(22.0, 33.0),
            LatLon::new(44.0, 55.0),
            0.1, 0.1, 10, 10
        );

        let result = grid.get_min_pos();
        assert_eq!(22.0, result.lat);
        assert_eq!(33.0, result.lon);

        let result = grid.get_max_pos();
        assert_eq!(44.0, result.lat);
        assert_eq!(55.0, result.lon);
    }


    #[test]
    fn it_calculates_the_correct_min_max_pos_for_reversed_grids() {
        let grid = LatLonGrid::new(
            LatLon::new(-20.0, 44.0),
            LatLon::new(33.0, 330.0),
            -0.1, -0.1, 10, 10
        );

        let result = grid.get_min_pos();
        assert_eq!(-20.0, result.lat);
        assert_eq!(-30.0, result.lon);

        let result = grid.get_max_pos();
        assert_eq!(33.0, result.lat);
        assert_eq!(44.0, result.lon);
    }
}
