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
