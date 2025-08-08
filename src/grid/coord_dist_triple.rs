use crate::grid::coord_dist::CoordDist;

pub struct CoordDistTriple {
    coord_dists: [Option<CoordDist>; 3],
}

impl CoordDistTriple {
    pub fn new() -> CoordDistTriple {
        CoordDistTriple {
            coord_dists: [None, None, None],
        }
    }

    pub fn get_coord_dist(&self, index: usize) -> Option<&CoordDist> {
        if index < 3 {
            self.coord_dists[index].as_ref()
        } else {
            None
        }
    }

    pub fn add_coord_dist(&mut self, coord_dist: CoordDist) {
        // find coord_dist larger than the provided coord_dist (or empty slot)
        let mut highest_dist = coord_dist.get_coord_dist();
        let mut highest_dist_idx = 0;

        for i in 0..3 {
            if let Some(cd) = &self.coord_dists[i] {
                if cd.get_coord_dist() > highest_dist {
                    highest_dist = cd.get_coord_dist();
                    highest_dist_idx = i;
                }
            } else {
                highest_dist_idx = i;
                break;
            }
        }

        // replace the coord_dist at the index with the highest distance
        self.coord_dists[highest_dist_idx] = Some(coord_dist);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_creates_a_new_instance_which_is_empty() {
        // given

        // when
        let coord_dist_triple = super::CoordDistTriple::new();

        // then
        assert!(coord_dist_triple.get_coord_dist(0).is_none());
        assert!(coord_dist_triple.get_coord_dist(1).is_none());
        assert!(coord_dist_triple.get_coord_dist(2).is_none());
        assert!(coord_dist_triple.get_coord_dist(3).is_none()); // out of bounds index
    }

    #[test]
    fn it_adds_a_single_coord_dist() {
        // given
        let mut coord_dist_triple = super::CoordDistTriple::new();
        let coord_dist = crate::grid::coord_dist::CoordDist::new(0, 10.0);

        // when
        coord_dist_triple.add_coord_dist(coord_dist);

        // then
        assert!(coord_dist_triple.get_coord_dist(0).is_some());
        assert!(coord_dist_triple.get_coord_dist(1).is_none());
        assert!(coord_dist_triple.get_coord_dist(2).is_none());
    }

    #[test]
    fn it_adds_coord_dist_and_replaces_highest_distance() {
        // given
        let mut coord_dist_triple = super::CoordDistTriple::new();
        let coord_dist1 = crate::grid::coord_dist::CoordDist::new(0, 10.0);
        let coord_dist2 = crate::grid::coord_dist::CoordDist::new(1, 5.0);
        let coord_dist3 = crate::grid::coord_dist::CoordDist::new(2, 15.0);
        let coord_dist4 = crate::grid::coord_dist::CoordDist::new(3, 8.0);

        // when
        coord_dist_triple.add_coord_dist(coord_dist1);
        coord_dist_triple.add_coord_dist(coord_dist2);
        coord_dist_triple.add_coord_dist(coord_dist3);
        coord_dist_triple.add_coord_dist(coord_dist4);

        // then
        assert!(coord_dist_triple.get_coord_dist(0).is_some());
        assert_eq!(coord_dist_triple.get_coord_dist(0).unwrap().get_coord_index(), 0);
        assert!(coord_dist_triple.get_coord_dist(1).is_some());
        assert_eq!(coord_dist_triple.get_coord_dist(1).unwrap().get_coord_index(), 1);
        assert!(coord_dist_triple.get_coord_dist(2).is_some());
        assert_eq!(coord_dist_triple.get_coord_dist(2).unwrap().get_coord_index(), 3);
    }
}
