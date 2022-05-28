use std::collections::HashMap;

pub struct JumpFlooder;


impl JumpFlooder {
    pub fn jump_fill(
        dimensions: (usize, usize),
        in_values: &Vec<f32>,
        missing_value: f32
    ) -> Vec<f32> {
        let (value_ids, coords) = Self::init_grid_values_and_coord_lookup(&dimensions, in_values, missing_value);

        let out_values = Self::create_output_values(&dimensions, in_values, missing_value, &coords, &value_ids);

        println!("{:?}", value_ids);
        println!("{:?}", coords);
        println!("{:?}", out_values);

        return out_values;
    }


    fn init_grid_values_and_coord_lookup(
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

        let grid = JumpFlooder::jump_fill((2, 2), &values, 0.00);

        assert_eq!(1.0, grid[0]);
        assert_eq!(9.0, grid[1]);
        assert_eq!(6.0, grid[2]);
        assert_eq!(2.0, grid[3]);
    }


    #[test]
    fn it_fills_a_grid_of_one_color() {
        let values = vec![
            0.00, 0.00, 0.00,
            0.00, 2.00, 0.00,
            0.00, 0.00, 0.00,
        ];

        let grid = JumpFlooder::jump_fill((3, 3), &values, 0.00);

        for i in 0..9 {
            assert_eq!(2.0, grid[i]);
        }
    }
}
