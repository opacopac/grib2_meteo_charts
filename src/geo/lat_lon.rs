#[derive(Debug)]
pub struct LatLon {
    pub lat: f32,
    pub lon: f32
}


impl LatLon {
    pub fn new(
        mut lat: f32,
        mut lon: f32
    ) -> LatLon {
        if lat >= 360.0 || lat < -180.0 {
            lat = lat.rem_euclid(360.0);
        }
        if lon >= 360.0 || lon < -180.0 {
            lon = lon.rem_euclid(360.0);
        }
        if lat >= 180.0 {
            lat = lat - 360.0;
        }
        if lon >= 180.0 {
            lon = lon - 360.0;
        }

        return LatLon {
            lat,
            lon
        }
    }


    pub fn clone(&self) -> LatLon {
        return LatLon::new(self.lat, self.lon);
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
}
