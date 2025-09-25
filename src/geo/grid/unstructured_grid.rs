use crate::geo::common::lat_lon::LatLon;
use crate::geo::common::lat_lon_extent::LatLonExtent;
use crate::geo::grid::coord_dist::CoordDist;
use crate::geo::grid::lat_lon_grid::LatLonGrid;


#[derive(Clone)]
pub struct UnstructuredGrid {
    lat_lon_grid: LatLonGrid,
    coordinates: Vec<LatLon>,
    coord_dist_map: Vec<Option<CoordDist>>,
}


impl UnstructuredGrid {
    pub fn new(
        dimensions: (usize, usize),
        lat_lon_extent: LatLonExtent,
        coordinates: Vec<LatLon>,
    ) -> UnstructuredGrid {
        let lat_lon_grid = LatLonGrid::new(dimensions, lat_lon_extent);
        let coord_dist: Vec<Option<CoordDist>> = vec![None; dimensions.0 * dimensions.1];

        UnstructuredGrid {
            lat_lon_grid,
            coordinates,
            coord_dist_map: coord_dist,
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        self.lat_lon_grid.get_dimensions()
    }

    pub fn get_index_by_x_y(&self, x: usize, y: usize) -> Option<usize> {
        self.lat_lon_grid.get_index_by_x_y(x, y)
    }

    pub fn get_x_y_by_lat_lon(&self, pos: &LatLon) -> Option<(f32, f32)> {
        self.lat_lon_grid.get_x_y_by_lat_lon(pos)
    }

    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        &self.lat_lon_grid.get_lat_lon_extent()
    }

    pub fn get_coord_dist(&self, x: usize, y: usize) -> Option<&CoordDist> {
        let idx = self.get_index_by_x_y(x, y)?;

        self.coord_dist_map[idx].as_ref()
    }

    pub fn calc_coord_dist_lookup_map(&mut self, max_deg_coord_dist: f32) {
        let min_lat = self.get_lat_lon_extent().min_coord.lat - max_deg_coord_dist;
        let max_lat = self.get_lat_lon_extent().max_coord.lat + max_deg_coord_dist;
        let min_lon = self.get_lat_lon_extent().min_coord.lon - max_deg_coord_dist;
        let max_lon = self.get_lat_lon_extent().max_coord.lon + max_deg_coord_dist;
        let max_deg_coord_dist_squared = max_deg_coord_dist * max_deg_coord_dist;

        for i in 0..self.coordinates.len() {
            let coord = &self.coordinates[i];
            if coord.lat < min_lat
                || coord.lat > max_lat
                || coord.lon < min_lon
                || coord.lon > max_lon
            {
                continue; // skip coordinates outside the extent + max distance
            }

            let (min_xy, max_xy) = self.calc_min_max_xy_for_coord(coord, max_deg_coord_dist);
            for x in min_xy.0..=max_xy.0 {
                for y in min_xy.1..=max_xy.1 {
                    let idx = match self.get_index_by_x_y(x, y) {
                        Some(idx) => idx,
                        None => continue,
                    };
                    let lat_lon = match self
                        .lat_lon_grid
                        .get_lat_lon_by_x_y(x as f32 + 0.5, y as f32 + 0.5)
                    {
                        Some(lat_lon) => lat_lon,
                        None => continue,
                    };
                    let new_dist_squared = coord.calc_euclidean_dist_squared(&lat_lon);
                    if new_dist_squared > max_deg_coord_dist_squared {
                        continue;
                    }
                    let current_dist_squared = match self.coord_dist_map[idx] {
                        Some(coord_dist) => coord_dist.get_coord_dist_squared(),
                        None => max_deg_coord_dist_squared
                    };
                    if new_dist_squared > current_dist_squared {
                        continue;
                    } else {
                        self.coord_dist_map[idx] = Some(CoordDist::new(i, new_dist_squared));
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
            Some(xy) => (xy.0 as usize, xy.1 as usize), // rounding down to containing cell
            None => (0, 0),
        };

        let max_xy = match self.lat_lon_grid.get_x_y_by_lat_lon(&max_pos) {
            Some(xy) => (xy.0 as usize, xy.1 as usize), // rounding down to containing cell
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
    /*use crate::geo::lat_lon::LatLon;
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

        // empty coord_dist_lookup_map
        assert_eq!(6, grid.coord_dist_lookup_map.len());
        for i in 0..6 {
            let cdt = &grid.coord_dist_lookup_map[i];
            for i in 0..3 {
                assert!(cdt.get_coord_dist(i).is_none());
            }
        }
    }

    #[test]
    fn it_calculates_coord_dist_lookup_map_for_one_coordinate() {
        // given
        let dimensions = (5, 5);
        let coordinates = vec![LatLon::new(25.0, 25.0)];
        let lat_lon_extent = LatLonExtent::new(LatLon::new(0.0, 0.0), LatLon::new(50.0, 50.0));
        let mut grid = super::UnstructuredGrid::new(dimensions, lat_lon_extent, coordinates);
        let max_dist = 15.0;

        // when
        grid.calc_coord_dist_lookup_map(max_dist);

        // then
        // expect no entries in the "outer" ring
        for i in [0, 1, 2, 3, 4, 5, 9, 10, 14, 15, 19, 20, 21, 22, 23, 24] {
            let cdt = &grid.coord_dist_lookup_map[i];
            assert!(cdt.get_coord_dist(0).is_none());
            assert!(cdt.get_coord_dist(1).is_none());
            assert!(cdt.get_coord_dist(2).is_none());
        }

        // expect one entry of index 0 in the center & adjacent cells
        for i in [6, 7, 8, 11, 12, 13, 16, 17, 18] {
            let cdt = &grid.coord_dist_lookup_map[i];
            assert!(cdt.get_coord_dist(0).is_some());
            assert!(cdt.get_coord_dist(1).is_none());
            assert!(cdt.get_coord_dist(2).is_none());

            let dist = cdt.get_coord_dist(0).unwrap();

            assert_eq!(0, dist.get_coord_index());
            assert!(dist.get_coord_dist_squared() <= max_dist);
        }
    }*/
}
