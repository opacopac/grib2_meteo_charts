#[derive(Debug)]
pub struct CoordDist {
    coord_index: usize,
    coord_dist: f32,
}

impl CoordDist {
    pub fn new(coord_index: usize, coord_dist: f32) -> CoordDist {
        CoordDist {
            coord_index,
            coord_dist,
        }
    }

    pub fn get_coord_index(&self) -> usize {
        self.coord_index
    }

    pub fn get_coord_dist(&self) -> f32 {
        self.coord_dist
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_creates_a_new_instance() {
        // given
        let coord_index = 5;
        let coord_dist = 10.0;

        // when
        let coord_dist_instance = super::CoordDist::new(coord_index, coord_dist);

        // then
        assert_eq!(coord_index, coord_dist_instance.get_coord_index());
        assert_eq!(coord_dist, coord_dist_instance.get_coord_dist());
    }
}
