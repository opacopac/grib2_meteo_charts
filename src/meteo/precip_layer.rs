use crate::geo::lat_lon::LatLon;

pub trait PrecipLayer {
    fn get_latlon_grid_points(&self) -> (u32, u32);

    fn get_precip_rate_kg_per_m2_per_s_by_latlon(&self, pos: &LatLon) -> f32;

    fn get_precip_rate_kg_per_m2_per_s_by_index(&self, index: usize) -> f32;
}
