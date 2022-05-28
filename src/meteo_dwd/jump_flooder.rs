use std::collections::HashMap;

pub struct JumpFlooder;


impl JumpFlooder {
    pub fn jump_flood(
        dimensions: (usize, usize),
        in_values: &Vec<f32>,
        missing_value: f32,
        first_step_size: usize
    ) -> Vec<f32> {
        // TODO: check dimensions (pow of 2 & same)

        println!("init...");
        let value_ids_and_coords = Self::init_value_ids_and_coords(&dimensions, in_values, missing_value);
        let mut value_ids = value_ids_and_coords.0;
        let coords = value_ids_and_coords.1;

        let mut step_size = first_step_size;
        while step_size >= 1 {
            println!("iteration with step size {}...", step_size);
            value_ids = Self::perform_flood_iteration(&dimensions, &value_ids, &coords, step_size as i32);
            step_size /= 2;
        }

        // perform final iteration with step size 1
        println!("final iteration...");
        value_ids = Self::perform_flood_iteration(&dimensions, &value_ids, &coords, 1);

        println!("output...");
        let out_values = Self::create_output_values(&dimensions, in_values, missing_value, &coords, &value_ids);

        return out_values;
    }


    fn init_value_ids_and_coords(
        dimensions: &(usize, usize),
        in_values: &Vec<f32>,
        missing_value: f32
    ) -> (Vec<usize>, HashMap<usize, (usize, usize)>) {
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

        return (value_ids, coords);
    }


    fn perform_flood_iteration(
        dimensions: &(usize, usize),
        value_ids: &Vec<usize>,
        coords: &HashMap<usize, (usize, usize)>,
        step_size: i32
    ) -> Vec<usize> {
        let mut new_value_ids: Vec<usize> = vec![];
        let dim0_i32 = dimensions.0 as i32;
        let dim1_i32 = dimensions.1 as i32;

        for y in 0..dimensions.1 {
            let y_i32 = y as i32;

            for x in 0..dimensions.0 {
                let x_i32 = x as i32;
                let own_idx = y * dimensions.0 + x;
                let own_value_id = value_ids[own_idx];
                let mut new_value_id = own_value_id;

                for j in [-step_size, 0, step_size] {
                    let y2 = y_i32 + j;
                    if y2 < 0 || y2 >= dim1_i32 {
                        continue;
                    }

                    for i in [-step_size, 0, step_size] {
                        let x2 = x_i32 + i;
                        if x2 < 0 || x2 >= dim0_i32 || (i == 0 && j == 0) {
                            continue;
                        }

                        let other_idx = (y2 * dim0_i32 + x2) as usize;
                        let other_value_id = value_ids[other_idx];

                        if other_value_id == 0 {
                            continue;
                        }

                        if own_value_id == 0 {
                            new_value_id = other_value_id;
                        } else {
                            let own_seed_coord = match coords.get(&own_value_id) {
                                Some(coord) => coord,
                                None => continue
                            };
                            let other_seed_coord = match coords.get(&other_value_id) {
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
                //print!("{} ", new_value_id);

                new_value_ids.push(new_value_id);
            }
            //println!();
        }
        //println!();

        return new_value_ids;
    }


    fn calc_sq_dist(coord1: &(usize, usize), coord2: (usize, usize)) -> isize {
        let coord_x_diff = coord2.0 as isize - coord1.0 as isize;
        let coord_y_diff = coord2.1 as isize - coord1.1 as isize;

        return coord_x_diff * coord_x_diff + coord_y_diff * coord_y_diff;
    }


    fn create_output_values(
        dimensions: &(usize, usize),
        in_values: &Vec<f32>,
        missing_value: f32,
        coords: &HashMap<usize, (usize, usize)>,
        value_ids: &Vec<usize>
    ) -> Vec<f32> {
        let mut out_values = vec![];

        for i in 0..(dimensions.0 * dimensions.1) {
            let value = match coords.get(&value_ids[i]) {
                Some(coord) => in_values[coord.1 * dimensions.0 + coord.0],
                None => missing_value
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

        let grid = JumpFlooder::jump_flood((2, 2), &values, 0.00);

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

        let grid = JumpFlooder::jump_flood((4, 4), &values, 0.00);

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

        let grid = JumpFlooder::jump_flood((4, 4), &values, 0.00);

        assert_eq!(1.0, grid[0]);
        assert_eq!(2.0, grid[3]);
        assert_eq!(3.0, grid[12]);
        assert_eq!(4.0, grid[15]);
    }
}
