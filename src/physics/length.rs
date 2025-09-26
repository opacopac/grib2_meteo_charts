pub struct Length;


const FEET_PER_M: f32 = 3.28084;


impl Length {
    pub fn from_meters_to_feet(meters: f32) -> f32 {
        meters * FEET_PER_M
    }


    pub fn from_feet_to_meters(feet: f32) -> f32 {
        feet / FEET_PER_M
    }
}


#[cfg(test)]
mod tests {
    use super::Length;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn it_converts_from_meters_to_feet() {
        // given
        let meters = 10.0;

        // when
        let feet = Length::from_meters_to_feet(meters);

        // then
        assert_approx_eq!(feet, 32.8084, 0.0001);
    }


    #[test]
    fn it_converts_from_feet_to_meters() {
        // given
        let feet = 10.0;

        // when
        let meters = Length::from_feet_to_meters(feet);

        // then
        assert_approx_eq!(meters, 3.048, 0.0001);
    }
}
