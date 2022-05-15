#[derive(Debug)]
pub enum NetCdfValues {
    ByteValues(Vec<u8>),
    CharValues(Vec<char>),
    ShortValues(Vec<i16>),
    IntValues(Vec<i32>),
    FloatValues(Vec<f32>),
    DoubleValues(Vec<f64>)
}


impl NetCdfValues {
    pub fn get_chars(&self) -> String {
        return match &self {
            NetCdfValues::CharValues(chars) => chars.into_iter().collect(),
            _ => panic!("invalid value type")
        }
    }


    pub fn get_ints(&self) -> Vec<i32> {
        return match &self {
            NetCdfValues::IntValues(ints) => ints.to_vec(),
            _ => panic!("invalid value type")
        }
    }


    pub fn get_doubles(&self) -> Vec<f64> {
        return match &self {
            NetCdfValues::DoubleValues(doubles) => doubles.to_vec(),
            _ => panic!("invalid value type")
        }
    }
}
