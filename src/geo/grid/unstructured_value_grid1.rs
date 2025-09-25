use crate::geo::common::lat_lon::LatLon;
use crate::geo::common::lat_lon_extent::LatLonExtent;
use crate::geo::grid::grid_value_type::GridValueType;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::unstructured_grid1::UnstructuredGrid1;


pub struct UnstructuredValueGrid1<T> {
    grid: UnstructuredGrid1,
    values: Vec<T>,
    missing_value: T,
}


impl<T: GridValueType> UnstructuredValueGrid1<T>
{
    pub fn new(
        values: Vec<T>,
        missing_value: T,
        grid: UnstructuredGrid1,
    ) -> UnstructuredValueGrid1<T> {
        UnstructuredValueGrid1 {
            grid,
            values,
            missing_value,
        }
    }

    pub fn get_grid(&self) -> &UnstructuredGrid1 {
        &self.grid
    }

    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        self.grid.get_dimensions()
    }

    pub fn get_grid_lat_lon_extent(&self) -> &LatLonExtent {
        self.grid.get_lat_lon_extent()
    }

    pub fn get_missing_value(&self) -> T {
        self.missing_value
    }

    pub fn get_value_by_xy(&self, x: usize, y: usize) -> Option<T> {
        let coord_dist_triple = self.grid.get_coord_dist_triple(x, y)?;
        let coord_dists = coord_dist_triple.get_coord_dists();

        if coord_dists.is_empty() {
            return None;
        }

        let coord_dist_sum: f32 = coord_dists.iter().map(|cd| cd.get_coord_dist_squared().sqrt()).sum();

        let value: T = coord_dists
            .iter()
            .filter_map(|cd| {
                let value_index = cd.get_coord_index();
                if let Some(value) = self.values.get(value_index) {
                    if *value == self.missing_value {
                        return None;
                    }
                    return Some(value.scale(cd.get_coord_dist_squared().sqrt() / coord_dist_sum));
                } else {
                    return None;
                }
            })
            .sum();

        Some(value)
    }


    pub fn get_value_by_lat_lon(&self, pos: &LatLon) -> Option<T> {
        let (x0, y0) = self.grid.get_x_y_by_lat_lon(pos)?;

        let x = x0.round() as usize;
        let y = y0.round() as usize;

        self.get_value_by_xy(x, y)
    }


    pub fn create_regular_grid(&self) -> LatLonValueGrid<T> {
        let lat_lon_extent = self.grid.get_lat_lon_extent().clone();
        let dimensions = self.grid.get_dimensions();
        let values: Vec<T> = (0..dimensions.0 * dimensions.1)
            .map(|i| {
                let x = i % dimensions.0;
                let y = i / dimensions.0;
                self.get_value_by_xy(x, y).unwrap_or(self.missing_value)
            })
            .collect();

        LatLonValueGrid::new(values, self.missing_value, dimensions, lat_lon_extent)
    }
}
