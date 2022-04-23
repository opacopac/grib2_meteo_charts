use crate::geo::lat_lon::LatLon;

#[derive(Debug)]
pub struct LatLonExtent {
    pub min_pos: LatLon,
    pub max_pos: LatLon
}


impl LatLonExtent {
    pub fn calc_mid_pos(&self) -> LatLon {
        return LatLon {
            lon: (self.min_pos.lon + self.max_pos.lon) / 2.0,
            lat: (self.min_pos.lat + self.max_pos.lat) / 2.0
        };
    }
}
