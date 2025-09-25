#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
pub struct CoordDist {
    coord_index: usize,
    coord_dist_squared: f32,
}

impl CoordDist {
    pub fn new(coord_index: usize, coord_dist_squared: f32) -> CoordDist {
        CoordDist {
            coord_index,
            coord_dist_squared,
        }
    }

    pub fn get_coord_index(&self) -> usize {
        self.coord_index
    }

    pub fn get_coord_dist_squared(&self) -> f32 {
        self.coord_dist_squared
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_creates_a_new_instance() {
        // given
        let coord_index = 5;
        let coord_dist_squared = 10.0 * 10.0;

        // when
        let coord_dist_instance = super::CoordDist::new(coord_index, coord_dist_squared);

        // then
        assert_eq!(coord_index, coord_dist_instance.get_coord_index());
        assert_eq!(coord_dist_squared, coord_dist_instance.get_coord_dist_squared());
    }
}
