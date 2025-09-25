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
