pub struct Speed;


const KNOTS_PER_MPS: f32 = 1.94384;


impl Speed {
    pub fn from_mps_to_knots(mps: f32) -> f32 {
        mps * KNOTS_PER_MPS
    }


    pub fn from_knots_to_mps(knots: f32) -> f32 {
        knots / KNOTS_PER_MPS
    }
}


#[cfg(test)]
mod tests {
    use super::Speed;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn it_converts_from_mps_to_knots() {
        // given
        let mps = 10.0;

        // when
        let knots = Speed::from_mps_to_knots(mps);

        // then
        assert_approx_eq!(knots, 19.4384, 0.0001);
    }


    #[test]
    fn it_converts_from_knots_to_mps() {
        // given
        let knots = 10.0;

        // when
        let mps = Speed::from_knots_to_mps(knots);

        // then
        assert_approx_eq!(mps, 5.14444, 0.0001);
    }
}