#[derive(Debug)]
pub struct LatLon {
    pub lat: f32,
    pub lon: f32,
}

impl LatLon {
    pub const MIN_LAT: f32 = -90.0;
    pub const MAX_LAT: f32 = 90.0;
    pub const MIN_LON: f32 = -180.0;
    pub const MAX_LON: f32 = 180.0;

    pub const MIN_COORD: LatLon = LatLon {
        lat: Self::MIN_LAT,
        lon: Self::MIN_LON,
    };
    pub const MAX_COORD: LatLon = LatLon {
        lat: Self::MAX_LAT,
        lon: Self::MAX_LON,
    };

    pub fn new(lat: f32, lon: f32) -> LatLon {
        if lat < Self::MIN_LAT
            || lat > Self::MAX_LAT
            || lon < Self::MIN_LON
            || lon > Self::MAX_LON
        {
            panic!("lat/lon values out of bounds: lat: {}, lon: {}", lat, lon);
        }

        LatLon { lat, lon }
    }

    pub fn clone(&self) -> LatLon {
        LatLon {
            lat: self.lat,
            lon: self.lon,
        }
    }

    pub fn as_array(&self) -> [f32; 2] {
        [self.lat, self.lon]
    }

    pub fn calc_euclidean_dist_squared(&self, other: &LatLon) -> f32 {
        let lat_diff = self.lat - other.lat;
        let lon_diff = self.lon - other.lon;
        let sq_dist = lat_diff * lat_diff + lon_diff * lon_diff;

        sq_dist
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

    #[test]
    fn it_always_creates_an_instance() {
        let result1 = LatLon::new(60.5, -40.1);
        assert_eq!(result1.lat, 60.5);
        assert_eq!(result1.lon, -40.1);
    }

    #[test]
    #[should_panic]
    fn it_panics_on_invalid_lat() {
        LatLon::new(-91.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn it_panics_on_invalid_lon() {
        LatLon::new(0.0, 180.5);
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
