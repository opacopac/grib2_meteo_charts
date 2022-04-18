pub struct LatLon {
    pub lat: f32,
    pub lon: f32
}


impl LatLon {
    pub fn new(
        lat: f32,
        lon: f32
    ) -> LatLon {
        return LatLon {
            lat,
            lon
        }
    }
}
