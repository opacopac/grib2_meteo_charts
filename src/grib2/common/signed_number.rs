pub struct SignedNumber;

impl SignedNumber {
    pub fn from_u16(raw_value: u16) -> i16 {
        let value = (raw_value & 0x7FFF) as i16;

        return if (raw_value & 0x8000) > 0 {
            -value
        } else {
            value
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::grib2::common::signed_number::SignedNumber;


    #[test]
    fn it_converts_plus1_correctly() {
        let result = SignedNumber::from_u16(0x0001);

        assert_eq!(1, result);
    }


    #[test]
    fn it_converts_minus1_correctly() {
        let result = SignedNumber::from_u16(0x8001);

        assert_eq!(-1, result);
    }


    #[test]
    fn it_converts_most_positive_value_correctly() {
        let result = SignedNumber::from_u16(0x7FFF);

        assert_eq!(0x7FFF, result);
    }


    #[test]
    fn it_converts_most_negative_value_correctly() {
        let result = SignedNumber::from_u16(0xFFFF);

        assert_eq!(-0x7FFF, result);
    }
}
