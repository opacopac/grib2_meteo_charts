use std::collections::HashMap;

pub struct JumpFlooder {
    dimensions: (usize, usize),
    dim_i32: (i32, i32),
    missing_value: f32,
    value_ids: Vec<usize>,
    coords: HashMap<usize, (usize, usize)>
}


impl JumpFlooder {
    pub fn new(
        dimensions: (usize, usize),
        in_values: &Vec<f32>,
        missing_value: f32,
    ) -> JumpFlooder {
        let dim_i32 = (dimensions.0 as i32, dimensions.1 as i32);
        let mut coords: HashMap<usize, (usize, usize)> = HashMap::new();
        let mut value_ids: Vec<usize> = vec![];
        let mut i = 0;

        for y in 0..dimensions.1 {
            for x in 0..dimensions.0 {
                let idx = y * dimensions.0 + x;
                let value = in_values[idx];
                if value != missing_value {
                    i += 1;
                    coords.insert(i, (x, y));
                    value_ids.push(i);
                } else {
                    value_ids.push(0);
                }
            }
        }

        let jump_flooder = JumpFlooder { dimensions, dim_i32, missing_value, value_ids, coords };

        return jump_flooder;
    }


    pub fn jump_flood(&mut self, in_values: &Vec<f32>, first_step_size: usize) -> Vec<f32> {
        println!("init...");
        let mut step_size = first_step_size;
        while step_size >= 1 {
            println!("iteration with step size {}...", step_size);
            self.value_ids = self.perform_flood_iteration(step_size as i32);
            step_size /= 2;
        }

        // perform final iteration with step size 1
        println!("final iteration...");
        self.value_ids = self.perform_flood_iteration(1);

        println!("output...");
        let out_values = self.create_output_values(in_values);

        return out_values;
    }


    fn perform_flood_iteration(&self, step_size: i32) -> Vec<usize> {
        let mut new_value_ids: Vec<usize> = vec![];

        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                let own_idx = y * self.dimensions.0 + x;
                let own_value_id = self.value_ids[own_idx];
                let new_value_id = self.calc_new_value_id(x, y, step_size, own_value_id);

                new_value_ids.push(new_value_id);
            }
        }

        return new_value_ids;
    }


    fn calc_new_value_id(
        &self,
        x: usize,
        y: usize,
        step_size: i32,
        own_value_id: usize,
    ) -> usize {
        let mut new_value_id = own_value_id;

        for j in [-step_size, 0, step_size] {
            let y2 = y as i32 + j;
            if y2 < 0 || y2 >= self.dim_i32.1 {
                continue;
            }

            for i in [-step_size, 0, step_size] {
                let x2 = x as i32 + i;
                if x2 < 0 || x2 >= self.dim_i32.0 || (i == 0 && j == 0) {
                    continue;
                }

                let other_idx = (y2 * self.dim_i32.0 + x2) as usize;
                let other_value_id = self.value_ids[other_idx];

                if other_value_id == 0 {
                    continue;
                }

                if own_value_id == 0 {
                    new_value_id = other_value_id;
                } else {
                    let own_seed_coord = match self.coords.get(&own_value_id) {
                        Some(coord) => coord,
                        None => continue
                    };
                    let other_seed_coord = match self.coords.get(&other_value_id) {
                        Some(coord) => coord,
                        None => continue
                    };

                    let own_dist = Self::calc_sq_dist(own_seed_coord, (x, y));
                    let other_dist = Self::calc_sq_dist(other_seed_coord, (x, y));
                    if other_dist < own_dist {
                        new_value_id = other_value_id;
                    }
                }
            }
        }

        return new_value_id;
    }


    fn calc_sq_dist(coord1: &(usize, usize), coord2: (usize, usize)) -> isize {
        let coord_x_diff = coord2.0 as isize - coord1.0 as isize;
        let coord_y_diff = coord2.1 as isize - coord1.1 as isize;

        return coord_x_diff * coord_x_diff + coord_y_diff * coord_y_diff;
    }


    fn create_output_values(&self, in_values: &Vec<f32>) -> Vec<f32> {
        let mut out_values = vec![];

        for i in 0..(self.dimensions.0 * self.dimensions.1) {
            let value = match self.coords.get(&self.value_ids[i]) {
                Some(coord) => in_values[coord.1 * self.dimensions.0 + coord.0],
                None => self.missing_value
            };
            out_values.push(value);
        }

        return out_values;
    }
}


#[cfg(test)]
mod tests {
    use crate::meteo_dwd::jump_flooder::JumpFlooder;

    #[test]
    fn it_doesnt_change_a_grid_without_missing_values() {
        let values = vec![
            1.00, 9.00,
            6.00, 2.00,
        ];

        let mut jump_flooder = JumpFlooder::new((2, 2), &values, 0.00);
        let grid = jump_flooder.jump_flood(&values, 1);

        assert_eq!(1.0, grid[0]);
        assert_eq!(9.0, grid[1]);
        assert_eq!(6.0, grid[2]);
        assert_eq!(2.0, grid[3]);
    }


    #[test]
    fn it_fills_a_grid_of_one_color() {
        let values = vec![
            0.00, 0.00, 0.00, 0.00,
            0.00, 2.00, 0.00, 0.00,
            0.00, 0.00, 0.00, 0.00,
            0.00, 0.00, 0.00, 0.00
        ];

        let mut jump_flooder = JumpFlooder::new((4, 4), &values, 0.00);
        let grid = jump_flooder.jump_flood(&values, 2);

        for i in 0..16 {
            assert_eq!(2.0, grid[i]);
        }
    }


    #[test]
    fn it_fills_a_grid_with_four_colors() {
        let values = vec![
            0.00, 0.00, 0.00, 0.00,
            0.00, 1.00, 2.00, 0.00,
            0.00, 3.00, 4.00, 0.00,
            0.00, 0.00, 0.00, 0.00
        ];

        let mut jump_flooder = JumpFlooder::new((4, 4), &values, 0.00);
        let grid = jump_flooder.jump_flood(&values, 2);

        assert_eq!(1.0, grid[0]);
        assert_eq!(2.0, grid[3]);
        assert_eq!(3.0, grid[12]);
        assert_eq!(4.0, grid[15]);
    }
}
