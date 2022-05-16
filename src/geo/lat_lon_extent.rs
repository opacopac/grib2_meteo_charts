use crate::geo::lat_lon::LatLon;

#[derive(Debug)]
pub struct LatLonExtent {
    pub min_coord: LatLon,
    pub max_coord: LatLon
}


impl LatLonExtent {
    pub fn new(
        min_coord: LatLon,
        max_coord: LatLon
    ) -> LatLonExtent {
        if min_coord.lat > max_coord.lat || min_coord.lon > max_coord.lon {
            panic!("min coord '{:?}' must be smaller than max coord '{:?}'", min_coord, max_coord);
        }

        return LatLonExtent { min_coord, max_coord };
    }


    pub fn clone(&self) -> LatLonExtent {
        let clone = LatLonExtent::new(
            self.min_coord.clone(),
            self.max_coord.clone()
        );

        return clone;
    }
}


impl PartialEq for LatLonExtent {
    fn eq(&self, other: &Self) -> bool {
        return self.min_coord == other.min_coord && self.max_coord == other.max_coord;
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_extent::LatLonExtent;

    #[test]
    fn it_creates_a_new_extent() {
        let extent = LatLonExtent::new(
            LatLon::new(-90.0, -180.0),
            LatLon::new(90.0, 180.0)
        );

        assert_eq!(extent.min_coord, LatLon::new(-90.0, -180.0));
        assert_eq!(extent.max_coord, LatLon::new(90.0, 180.0));
    }


    #[test]
    #[should_panic]
    fn it_panics_if_min_coord_larger_than_max_coord() {
        LatLonExtent::new(
            LatLon::new(90.0, 180.0),
            LatLon::new(-90.0, -180.0)
        );
    }


    #[test]
    #[should_panic]
    fn it_panics_if_min_coord_larger_than_max_coord_2() {
        LatLonExtent::new(
            LatLon::new(90.0, -180.0),
            LatLon::new(-90.0, 180.0)
        );
    }


    #[test]
    #[should_panic]
    fn it_panics_if_min_coord_larger_than_max_coord_3() {
        LatLonExtent::new(
            LatLon::new(-90.0, 180.0),
            LatLon::new(90.0, -180.0)
        );
    }
}
