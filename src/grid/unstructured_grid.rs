use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grid::coord_dist::CoordDist;
use crate::grid::coord_dist_triple::CoordDistTriple;
use crate::grid::lat_lon_grid::LatLonGrid;

pub struct UnstructuredGrid {
    lat_lon_grid: LatLonGrid,
    coordinates: Vec<LatLon>,
    coord_dist_lookup_map: Vec<CoordDistTriple>,
}

impl UnstructuredGrid {
    pub fn new(
        dimensions: (usize, usize),
        lat_lon_extent: LatLonExtent,
        coordinates: Vec<LatLon>,
    ) -> UnstructuredGrid {
        let lat_lon_grid = LatLonGrid::new(dimensions, lat_lon_extent);
        let coord_dist_lookup_map = vec![CoordDistTriple::new(); dimensions.0 * dimensions.1];

        UnstructuredGrid {
            lat_lon_grid,
            coordinates,
            coord_dist_lookup_map,
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        self.lat_lon_grid.get_dimensions()
    }

    pub fn get_index_by_x_y(&self, x: usize, y: usize) -> Option<usize> {
        self.lat_lon_grid.get_index_by_x_y(x, y)
    }

    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        &self.lat_lon_grid.get_lat_lon_extent()
    }

    pub fn get_coord_dist_triple(&self, x: usize, y: usize) -> Option<&CoordDistTriple> {
        let idx = self.get_index_by_x_y(x, y)?;
        Some(&self.coord_dist_lookup_map[idx])
    }

    pub fn calc_coord_dist_lookup_map(&mut self, max_coord_dist_deg: f32) {
        for i in 0..self.coordinates.len() {
            let coord = &self.coordinates[i];
            let (min_xy, max_xy) = self.calc_min_max_xy_for_coord(coord, max_coord_dist_deg);

            for x in min_xy.0..=max_xy.0 {
                for y in min_xy.1..=max_xy.1 {
                    if let Some(idx) = self.get_index_by_x_y(x, y) {
                        let lat_lon = self.lat_lon_grid.get_lat_lon_by_x_y(x, y);
                        let dist = match lat_lon {
                            Some(lat_lon) => coord.calc_euclidean_dist(&lat_lon),
                            None => continue, // skip if lat_lon is not found
                        };
                        let coord_dist = CoordDist::new(i, dist);
                        let coord_triple = &mut self.coord_dist_lookup_map[idx];
                        coord_triple.add_coord_dist(coord_dist);
                    }
                }
            }
        }
    }

    fn calc_min_max_xy_for_coord(
        &self,
        coord: &LatLon,
        max_coord_dist_deg: f32,
    ) -> ((usize, usize), (usize, usize)) {
        let min_pos = LatLon::new(
            coord.lat - max_coord_dist_deg,
            coord.lon - max_coord_dist_deg,
        );
        let max_pos = LatLon::new(
            coord.lat + max_coord_dist_deg,
            coord.lon + max_coord_dist_deg,
        );

        let min_xy = match self.lat_lon_grid.get_x_y_by_lat_lon(&min_pos) {
            Some(xy) => (xy.0 as usize, xy.1 as usize),
            None => (0, 0),
        };

        let max_xy = match self.lat_lon_grid.get_x_y_by_lat_lon(&max_pos) {
            Some(xy) => (xy.0 as usize, xy.1 as usize),
            None => (
                self.lat_lon_grid.get_dimensions().0 - 1,
                self.lat_lon_grid.get_dimensions().1 - 1,
            ),
        };

        (min_xy, max_xy)
    }
}

#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_extent::LatLonExtent;

    #[test]
    fn it_creates_a_new_instance() {
        // given
        let dimensions = (2, 3);
        let coordinates = vec![
            LatLon::new(40.0, 7.0),
            LatLon::new(41.0, 7.0),
            LatLon::new(42.0, 7.0),
        ];
        let lat_lon_extent = LatLonExtent::MAX_EXTENT;

        // when
        let grid = super::UnstructuredGrid::new(dimensions, lat_lon_extent, coordinates);

        // then
        assert_eq!((2, 3), grid.get_dimensions());
        assert_eq!(3, grid.coordinates.len());
        assert_eq!(&LatLonExtent::MAX_EXTENT, grid.get_lat_lon_extent());
    }

    #[test]
    fn it_calculates_coord_dist_lookup_map_for_one_coordinate() {
        // given
        let dimensions = (5, 5);
        let coordinates = vec![LatLon::new(0.0, 0.0)];
        let lat_lon_extent = LatLonExtent::new(LatLon::new(-50.0, -50.0), LatLon::new(50.0, 50.0));
        let mut grid = super::UnstructuredGrid::new(dimensions, lat_lon_extent, coordinates);
        let max_dist = 11.0;

        // when
        grid.calc_coord_dist_lookup_map(max_dist);

        // then
        assert_eq!(25, grid.coord_dist_lookup_map.len());
        let cdt_0_0 = grid.get_coord_dist_triple(0, 0).unwrap();
        assert!(cdt_0_0.get_coord_dist(0).is_none());
        assert!(cdt_0_0.get_coord_dist(1).is_none());
        assert!(cdt_0_0.get_coord_dist(2).is_none());

        let cdt_2_1 = grid.get_coord_dist_triple(2, 1).unwrap();
        assert!(cdt_2_1.get_coord_dist(0).is_some());
        assert!(cdt_2_1.get_coord_dist(1).is_none());
        assert!(cdt_2_1.get_coord_dist(2).is_none());

        let cdt_2_1_0 = cdt_2_1.get_coord_dist(0).unwrap();
        assert_eq!(0, cdt_2_1_0.get_coord_index());
        assert!(cdt_2_1_0.get_coord_dist() < max_dist);

        let cdt_2_2 = grid.get_coord_dist_triple(2, 2).unwrap();
        assert!(cdt_2_2.get_coord_dist(0).is_some());
        assert!(cdt_2_2.get_coord_dist(1).is_none());
        assert!(cdt_2_2.get_coord_dist(2).is_none());

        let cdt_2_2_0 = cdt_2_2.get_coord_dist(0).unwrap();
        assert_eq!(0, cdt_2_2_0.get_coord_index());
        assert!(cdt_2_2_0.get_coord_dist() < max_dist);
    }
}
