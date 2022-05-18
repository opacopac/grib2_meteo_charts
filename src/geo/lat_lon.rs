#[derive(Debug)]
pub struct LatLon {
    pub lat: f32,
    pub lon: f32
}


impl LatLon {
    pub const MIN_LAT: f32 = -90.0;
    pub const MAX_LAT: f32 = 90.0;
    pub const MIN_LON: f32 = -180.0;
    pub const MAX_LON: f32 = 180.0;

    pub const MIN_COORD: LatLon = LatLon { lat: Self::MIN_LAT, lon: Self::MIN_LON };
    pub const MAX_COORD: LatLon = LatLon { lat: Self::MAX_LAT, lon: Self::MAX_LON };


    // TODO
    pub fn new(
        mut lat: f32,
        mut lon: f32
    ) -> LatLon {
        if lat >= 360.0 || lat < -180.0 {
            lat = lat.rem_euclid(360.0);
        }
        if lon >= 360.0 || lon < Self::MIN_LON {
            lon = lon.rem_euclid(360.0);
        }
        if lat >= 180.0 {
            lat = lat - 360.0;
        }
        if lon >= Self::MAX_LON {
            lon = lon - 360.0;
        }

        return LatLon {
            lat,
            lon
        }
    }


    pub fn clone(&self) -> LatLon {
        return LatLon { lat: self.lat, lon: self.lon };
    }
}


impl PartialEq for LatLon {
    fn eq(&self, other: &Self) -> bool {
        return self.lat == other.lat && self.lon == other.lon;
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;


    // TODO: lat: -90 - 90
    #[test]
    fn it_always_creates_angle_values_between_m180_and_p180() {
        let result1 = LatLon::new(60.5, -40.1);
        assert_eq!(result1.lat, 60.5);
        assert_eq!(result1.lon, -40.1);

        let result2 = LatLon::new(360.5, 270.0);
        assert_eq!(result2.lat, 0.5);
        assert_eq!(result2.lon, -90.0);

        let result3 = LatLon::new(360.0, -720.0);
        assert_eq!(result3.lat, 0.0);
        assert_eq!(result3.lon, 0.0);
    }


    #[test]
    fn it_clones_a_value() {
        let pos = LatLon::new(60.5, -40.1);

        let clone = pos.clone();

        assert_eq!(pos, clone);
    }


    #[test]
    fn it_clones_the_min_max_coords() {
        let pos1 = LatLon::MIN_COORD;
        let pos2 = LatLon::MAX_COORD;

        let clone1 = pos1.clone();
        let clone2 = pos2.clone();

        assert_eq!(pos1, clone1);
        assert_eq!(pos2, clone2);
    }
}
