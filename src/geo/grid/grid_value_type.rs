use std::iter::Sum;


pub trait GridValueType: Copy + PartialEq + Send + Sync + Sum {
    fn zero() -> Self;

    fn scale(self, factor: f32) -> Self;

    fn add (self, other: Self) -> Self;
}


impl GridValueType for u8 {
    fn zero() -> Self {
        0
    }


    fn scale(self, factor: f32) -> Self {
        let val = (self as f32) * factor;

        // treat NaN as 0.0, clamp to [0.0, 255.0], then round
        val
            .is_nan()
            .then(|| 0.0)
            .unwrap_or_else(|| val.clamp(0.0, 255.0))
            .round() as u8
    }


    fn add(self, other: Self) -> Self {
        let sum = (self as u16) + (other as u16);
        if sum > 255 {
            255
        } else {
            sum as u8
        }
    }
}


impl GridValueType for f32 {
    fn zero() -> Self {
        0.0
    }


    fn scale(self, factor: f32) -> Self {
        self * factor
    }


    fn add(self, other: Self) -> Self {
        self + other
    }
}


pub fn grid_value_type_sum<T, I>(iter: I) -> T
where
    T: GridValueType,
    I: IntoIterator<Item = T>,
{
    let mut acc = T::zero();
    for v in iter {
        acc = acc.add(v);
    }
    acc
}
