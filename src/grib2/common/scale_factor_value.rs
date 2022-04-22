#[derive(Debug)]
pub struct ScaleFactorValue {
    pub factor: u8,
    pub value: u32
}


impl ScaleFactorValue {
    pub fn new(
        factor: u8,
        value: u32
    ) -> ScaleFactorValue {
        return ScaleFactorValue {
            factor,
            value
        }
    }
}
