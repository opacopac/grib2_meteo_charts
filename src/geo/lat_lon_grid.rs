use derive_more::Constructor;

use crate::geo::lat_lon::LatLon;

#[derive(Debug, Constructor)]
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
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_grid::LatLonGrid;


    #[test]
    fn it_correctly_detects_if_a_pos_is_inside() {
        let extent = LatLonGrid::new(
            LatLon::new(47.0, 7.0),
            LatLon::new(48.0, 8.0),
            0.1,
            0.1,
            10,
            10
        );

        let result = extent.is_pos_inside(&LatLon::new(47.5, 7.5));
        assert_eq!(true, result);

        let result = extent.is_pos_inside(&LatLon::new(48.1, 7.5));
        assert_eq!(false, result);

        let result = extent.is_pos_inside(&LatLon::new(47.5, 6.9));
        assert_eq!(false, result);
    }


    #[test]
    fn it_correctly_detects_if_a_pos_is_inside_across_0_deg_line() {
        let extent = LatLonGrid::new(
            LatLon::new(-20.0, 330.0),
            LatLon::new(33.0, 44.0),
            0.1,
            0.1,
            10,
            10
        );

        let result = extent.is_pos_inside(&LatLon::new(-10.0, 340.0));
        assert_eq!(true, result);

        let result = extent.is_pos_inside(&LatLon::new(-21.0, 0.0));
        assert_eq!(false, result);

        let result = extent.is_pos_inside(&LatLon::new(0.0, 320.0));
        assert_eq!(false, result);
    }
}
