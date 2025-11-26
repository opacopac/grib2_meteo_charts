use crate::geo::common::lat_lon::LatLon;

#[derive(Debug, Clone)]
pub struct LatLonExtent {
    pub min_coord: LatLon,
    pub max_coord: LatLon,
}

impl LatLonExtent {
    pub const MAX_EXTENT: LatLonExtent = LatLonExtent {
        min_coord: LatLon::MIN_COORD,
        max_coord: LatLon::MAX_COORD,
    };
    pub const MERCATOR_EXTENT: LatLonExtent = LatLonExtent {
        min_coord: LatLon::MIN_MERCATOR_COORD,
        max_coord: LatLon::MAX_MERCATOR_COORD,
    };

    pub fn new(min_coord: LatLon, max_coord: LatLon) -> LatLonExtent {
        if min_coord.lat > max_coord.lat || min_coord.lon > max_coord.lon {
            panic!(
                "min coord '{:?}' must be smaller than max coord '{:?}'",
                min_coord, max_coord
            );
        }

        LatLonExtent {
            min_coord,
            max_coord,
        }
    }

    pub fn clone(&self) -> LatLonExtent {
        LatLonExtent {
            min_coord: self.min_coord.clone(),
            max_coord: self.max_coord.clone(),
        }
    }

    pub fn calc_midpoint(&self) -> LatLon {
        LatLon {
            lat: (self.min_coord.lat + self.max_coord.lat) / 2.0,
            lon: (self.min_coord.lon + self.max_coord.lon) / 2.0,
        }
    }

    pub fn is_inside(&self, point: &LatLon) -> bool {
        point.lat >= self.min_coord.lat
            && point.lat < self.max_coord.lat
            && point.lon >= self.min_coord.lon
            && point.lon < self.max_coord.lon
    }

    pub fn calc_min_bounding_extent(coordinates: &[LatLon]) -> LatLonExtent {
        let mut min_lat = LatLon::MAX_LAT;
        let mut max_lat = LatLon::MIN_LAT;
        let mut min_lon = LatLon::MAX_LON;
        let mut max_lon = LatLon::MIN_LON;

        for coord in coordinates {
            if coord.lat < min_lat {
                min_lat = coord.lat;
            }
            if coord.lat > max_lat {
                max_lat = coord.lat;
            }
            if coord.lon < min_lon {
                min_lon = coord.lon;
            }
            if coord.lon > max_lon {
                max_lon = coord.lon;
            }
        }

        LatLonExtent::new(LatLon::new(min_lat, min_lon), LatLon::new(max_lat, max_lon))
    }
}

impl PartialEq for LatLonExtent {
    fn eq(&self, other: &Self) -> bool {
        self.min_coord == other.min_coord && self.max_coord == other.max_coord
    }
}

#[cfg(test)]
mod tests {
    use crate::geo::common::lat_lon::LatLon;
    use crate::geo::common::lat_lon_extent::LatLonExtent;

    #[test]
    fn it_creates_a_new_extent() {
        let extent = LatLonExtent::new(LatLon::new(0.0, 0.0), LatLon::new(45.0, 90.0));

        assert_eq!(extent.min_coord, LatLon::new(0.0, 0.0));
        assert_eq!(extent.max_coord, LatLon::new(45.0, 90.0));
    }

    #[test]
    #[should_panic]
    fn it_panics_if_min_coord_is_larger_than_max_coord() {
        LatLonExtent::new(LatLon::new(45.0, 90.0), LatLon::new(0.0, 0.0));
    }

    #[test]
    #[should_panic]
    fn it_panics_if_min_coord_is_larger_than_max_coord_2() {
        LatLonExtent::new(LatLon::new(45.0, 0.0), LatLon::new(0.0, 90.0));
    }

    #[test]
    #[should_panic]
    fn it_panics_if_min_coord_is_larger_than_max_coord_3() {
        LatLonExtent::new(LatLon::new(0.0, 90.0), LatLon::new(45.0, 0.0));
    }

    #[test]
    fn it_clones_an_extent() {
        let extent = LatLonExtent::new(LatLon::new(-10.0, 20.0), LatLon::new(45.0, 90.0));

        let clone = extent.clone();

        assert_eq!(extent, clone);
    }

    #[test]
    fn it_clones_a_max_extent() {
        let extent = LatLonExtent::MAX_EXTENT;

        let clone = extent.clone();

        assert_eq!(extent, clone);
    }

    #[test]
    fn it_calculates_the_midpoint() {
        let extent = LatLonExtent::MAX_EXTENT;

        let mid_point = extent.calc_midpoint();

        assert_eq!(LatLon::new(0.0, 0.0), mid_point);
    }

    #[test]
    fn it_checks_if_a_point_is_inside() {
        let extent = LatLonExtent::new(LatLon::new(-10.0, -20.0), LatLon::new(30.0, 40.0));
        let point1 = LatLon::new(0.0, 0.0);
        let point2 = LatLon::new(-20.0, 0.0);

        let is_inside1 = extent.is_inside(&point1);
        let is_inside2 = extent.is_inside(&point2);

        assert!(is_inside1);
        assert!(!is_inside2);
    }

    #[test]
    fn it_calculates_min_bounding_extent() {
        // given
        let coordinates = vec![
            LatLon::new(40.0, 7.0),
            LatLon::new(50.0, 6.0),
            LatLon::new(45.0, 9.0),
        ];

        // when
        let extent = LatLonExtent::calc_min_bounding_extent(&coordinates);

        // then
        assert_eq!(LatLon::new(40.0, 6.0), extent.min_coord);
        assert_eq!(LatLon::new(50.0, 9.0), extent.max_coord);
    }
}
