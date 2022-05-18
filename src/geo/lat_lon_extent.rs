use crate::geo::lat_lon::LatLon;

#[derive(Debug)]
pub struct LatLonExtent {
    pub min_coord: LatLon,
    pub max_coord: LatLon
}


impl LatLonExtent {
    pub const MAX_EXTENT: LatLonExtent = LatLonExtent { min_coord: LatLon::MIN_COORD, max_coord: LatLon::MAX_COORD };

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
        let clone = LatLonExtent {
            min_coord: self.min_coord.clone(),
            max_coord: self.max_coord.clone()
        };

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
            LatLon::new(0.0, 0.0),
            LatLon::new(45.0, 90.0)
        );

        assert_eq!(extent.min_coord, LatLon::new(0.0, 0.0));
        assert_eq!(extent.max_coord, LatLon::new(45.0, 90.0));
    }


    #[test]
    #[should_panic]
    fn it_panics_if_min_coord_is_larger_than_max_coord() {
        LatLonExtent::new(
            LatLon::new(45.0, 90.0),
            LatLon::new(0.0, 0.0)
        );
    }


    #[test]
    #[should_panic]
    fn it_panics_if_min_coord_is_larger_than_max_coord_2() {
        LatLonExtent::new(
            LatLon::new(45.0, 0.0),
            LatLon::new(0.0, 90.0)
        );
    }


    #[test]
    #[should_panic]
    fn it_panics_if_min_coord_is_larger_than_max_coord_3() {
        LatLonExtent::new(
            LatLon::new(0.0, 90.0),
            LatLon::new(45.0, 0.0)
        );
    }


    #[test]
    fn it_clones_an_extent() {
        let extent = LatLonExtent::new(
            LatLon::new(-10.0, 20.0),
            LatLon::new(45.0, 90.0)
        );

        let clone = extent.clone();

        assert_eq!(extent, clone);
    }


    #[test]
    fn it_clones_a_max_extent() {
        let extent = LatLonExtent::MAX_EXTENT;

        let clone = extent.clone();

        assert_eq!(extent, clone);
    }
}
