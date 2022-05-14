#[derive(Debug)]
pub enum NetCdfValues {
    ByteValues(Vec<u8>),
    CharValues(Vec<char>),
    ShortValues(Vec<i16>),
    IntValues(Vec<i32>),
    FloatValues(Vec<f32>),
    DoubleValues(Vec<f64>)
}
