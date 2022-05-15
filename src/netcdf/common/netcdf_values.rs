use std::fmt::{Display, Formatter};

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
    pub fn get_bytes(&self) -> Vec<u8> {
        return match &self {
            NetCdfValues::ByteValues(bytes) => bytes.to_vec(),
            _ => panic!("invalid value type, expected 'ByteValues")
        }
    }


    pub fn get_chars(&self) -> String {
        return match &self {
            NetCdfValues::CharValues(chars) => chars.iter().collect(),
            _ => panic!("invalid value type, expected 'CharValues")
        }
    }


    pub fn get_shorts(&self) -> Vec<i16> {
        return match &self {
            NetCdfValues::ShortValues(shorts) => shorts.to_vec(),
            _ => panic!("invalid value type, expected 'ShortValues")
        }
    }


    pub fn get_ints(&self) -> Vec<i32> {
        return match &self {
            NetCdfValues::IntValues(ints) => ints.to_vec(),
            _ => panic!("invalid value type, expected 'IntValues")
        }
    }


    pub fn get_floats(&self) -> Vec<f32> {
        return match &self {
            NetCdfValues::FloatValues(floats) => floats.to_vec(),
            _ => panic!("invalid value type, expected 'FloatValues")
        }
    }


    pub fn get_doubles(&self) -> Vec<f64> {
        return match &self {
            NetCdfValues::DoubleValues(doubles) => doubles.to_vec(),
            _ => panic!("invalid value type, expected 'DoubleValues")
        }
    }
}


impl Display for NetCdfValues {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NetCdfValues::ByteValues(values) => write!(f, "{}", values.iter().fold(String::new(), |val, acc| format!("{} {}", acc, val))),
            NetCdfValues::CharValues(values) => write!(f, "{}", values.iter().collect::<String>()),
            NetCdfValues::ShortValues(values) => write!(f, "{}", values.iter().fold(String::new(), |val, acc| format!("{} {}", acc, val))),
            NetCdfValues::IntValues(values) => write!(f, "{}", values.iter().fold(String::new(), |val, acc| format!("{} {}", acc, val))),
            NetCdfValues::FloatValues(values) => write!(f, "{}", values.iter().fold(String::new(), |val, acc| format!("{} {}", acc, val))),
            NetCdfValues::DoubleValues(values) => write!(f, "{}", values.iter().fold(String::new(), |val, acc| format!("{} {}", acc, val))),
        }
    }
}
