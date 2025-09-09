use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grid::grid_value_type::GridValueType;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grid::unstructured_grid::UnstructuredGrid;


pub struct UnstructuredValueGrid<T> {
    grid: UnstructuredGrid,
    values: Vec<T>,
    missing_value: T,
}


impl<T: GridValueType> UnstructuredValueGrid<T>
{
    pub fn new(
        values: Vec<T>,
        missing_value: T,
        grid: UnstructuredGrid,
    ) -> UnstructuredValueGrid<T> {
        UnstructuredValueGrid {
            grid,
            values,
            missing_value,
        }
    }


    pub fn get_grid(&self) -> &UnstructuredGrid {
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
        let coord_dist = match self.grid.get_coord_dist(x, y) {
            Some(coord_dist) => coord_dist,
            None => return None,
        };

        let value_index = coord_dist.get_coord_index();
        let value = match self.values.get(value_index) {
            Some(val) => *val,
            None => return None,
        };

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
