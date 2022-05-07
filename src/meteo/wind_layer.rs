use crate::geo::lat_lon::LatLon;

pub trait WindLayer {
    fn get_latlon_grid_points(&self) -> (u32, u32);

    fn get_wind_speed_m_per_s_by_latlon(&self, pos: &LatLon) -> (f32, f32);

    fn get_wind_speed_m_per_s_by_index(&self, index: usize) -> (f32, f32);
}
