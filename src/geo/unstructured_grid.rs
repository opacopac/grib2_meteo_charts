use crate::geo::lat_lon::LatLon;

pub struct UnstructuredGrid {
    grid_points: Vec<LatLon>
}


impl UnstructuredGrid {
    pub fn new() -> UnstructuredGrid {
        let grid = UnstructuredGrid { grid_points: vec![] };

        return grid;
    }


    pub fn add_point(&mut self, lat_lon: LatLon) {
        self.grid_points.push(lat_lon);
    }


    pub fn get_point_count(&self) -> usize {
        return self.grid_points.len();
    }


    pub fn get_point_by_idx(&self, index: usize) -> &LatLon {
        let point = &self.grid_points[index];

        return point;
    }


    pub fn get_idx_by_lat_lon(&self, lat_lon: &LatLon) -> usize {
        // TODO: temp => use spacial index

        let mut best_dist = 999999.0;
        let mut best_idx = 0;
        for i in 0..self.grid_points.len() {
            let lat_dist = lat_lon.lat - self.grid_points[i].lat;
            let lon_dist = lat_lon.lon - self.grid_points[i].lon;
            let dist = lat_dist * lat_dist + lon_dist * lon_dist;

            if dist < best_dist {
                best_dist = dist;
                best_idx = i;
            }
        }

        return best_idx;
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::unstructured_grid::UnstructuredGrid;

    #[test]
    fn it_adds_a_grid_point() {
        let mut grid = UnstructuredGrid::new();
        let lat_lon = LatLon::new(47.0, 7.0);

        grid.add_point(lat_lon);

        assert_eq!(1, grid.grid_points.len());
    }


    #[test]
    fn it_gets_the_number_of_grid_point() {
        let mut grid = UnstructuredGrid::new();
        grid.add_point(LatLon::new(47.0, 7.0));
        grid.add_point(LatLon::new(47.6, 7.6));

        let point_count = grid.get_point_count();

        assert_eq!(2, point_count);
    }


    #[test]
    fn it_gets_a_grid_point_by_index() {
        let mut grid = UnstructuredGrid::new();
        let point = LatLon::new(47.0, 7.0);
        grid.add_point(point);

        let point = grid.get_point_by_idx(0);

        assert_eq!(47.0, point.lat);
        assert_eq!(7.0, point.lon);
    }


    #[test]
    fn it_gets_the_closest_index_by_lat_lon() {
        let mut grid = UnstructuredGrid::new();
        grid.add_point(LatLon::new(47.0, 7.0));
        grid.add_point(LatLon::new(47.6, 7.6));
        let lat_lon = LatLon::new(48.0, 8.0);

        let result = grid.get_idx_by_lat_lon(&lat_lon);

        assert_eq!(1, result);
    }
}
