pub struct Temperature;


const ZERO_CELSIUS_IN_KELVIN: f32 = 273.15;


impl Temperature {
    pub fn from_kelvin_to_celsius(kelvin: f32) -> f32 {
        kelvin - ZERO_CELSIUS_IN_KELVIN
    }

    pub fn from_celsius_to_kelvin(celsius: f32) -> f32 {
        celsius + ZERO_CELSIUS_IN_KELVIN
    }
}


#[cfg(test)]
mod tests {
    use super::Temperature;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn it_converts_from_kelvin_to_celsius() {
        // given
        let kelvin = 300.0;

        // when
        let celsius = Temperature::from_kelvin_to_celsius(kelvin);

        // then
        assert_approx_eq!(celsius, 26.85, 0.01);
    }


    #[test]
    fn it_converts_from_celsius_to_kelvin() {
        // given
        let celsius = 26.85;

        // when
        let kelvin = Temperature::from_celsius_to_kelvin(celsius);

        // then
        assert_approx_eq!(kelvin, 300.0, 0.01);
    }
}
