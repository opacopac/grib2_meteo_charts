use crate::geo::lat_lon::LatLon;

pub trait CloudLayer {
    fn get_latlon_grid_points(&self) -> (u32, u32);

    fn get_tot_cloud_cover_percent_by_latlon(&self, pos: &LatLon) -> f32;

    fn get_tot_cloud_cover_percent_by_index(&self, index: usize) -> f32;
}
