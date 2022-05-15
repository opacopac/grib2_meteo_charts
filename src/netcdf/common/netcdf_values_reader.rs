use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::common::netcdf_value_type::NetCdfValueType;
use crate::netcdf::common::netcdf_values::NetCdfValues;

pub struct NetCdfValuesReader;

impl NetCdfValuesReader {
    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>, value_count: usize, nc_type: &NetCdfValueType) -> Result<NetCdfValues, NetCdfError> {
        let values = match nc_type {
            NetCdfValueType::NcByte => Self::read_byte_values(reader, value_count)?,
            NetCdfValueType::NcChar => Self::read_char_values(reader, value_count)?,
            NetCdfValueType::NcShort => Self::read_short_values(reader, value_count)?,
            NetCdfValueType::NcInt => Self::read_int_values(reader, value_count)?,
            NetCdfValueType::NcFloat => Self::read_float_values(reader, value_count)?,
            NetCdfValueType::NcDouble => Self::read_double_values(reader, value_count)?,
        };

        return Ok(values);
    }


    fn read_byte_values<T: Read + Seek>(reader: &mut BufReader<T>, value_count: usize) -> Result<NetCdfValues, NetCdfError> {
        let mut byte_values = vec![0; value_count];
        reader.read_exact(&mut byte_values)?;

        Self::read_padding(reader, value_count, 1)?;

        let values = NetCdfValues::ByteValues(byte_values);

        return Ok(values);
    }


    fn read_char_values<T: Read + Seek>(reader: &mut BufReader<T>, value_count: usize) -> Result<NetCdfValues, NetCdfError> {
        let mut char_values: Vec<char> = vec![];
        for _ in 0..value_count {
            let value = reader.read_u8()?;
            char_values.push(value as char);
        }

        Self::read_padding(reader, value_count, 1)?;

        let values = NetCdfValues::CharValues(char_values);

        return Ok(values);
    }


    fn read_short_values<T: Read + Seek>(reader: &mut BufReader<T>, value_count: usize) -> Result<NetCdfValues, NetCdfError> {
        let mut short_values = vec![0 as i16; value_count];
        reader.read_i16_into::<BigEndian>(&mut short_values)?;

        Self::read_padding(reader, value_count, 2)?;

        let values = NetCdfValues::ShortValues(short_values);

        return Ok(values);
    }


    fn read_int_values<T: Read + Seek>(reader: &mut BufReader<T>, value_count: usize) -> Result<NetCdfValues, NetCdfError> {
        let mut int_values = vec![0 as i32; value_count];
        reader.read_i32_into::<BigEndian>(&mut int_values)?;

        let values = NetCdfValues::IntValues(int_values);

        return Ok(values);
    }


    fn read_float_values<T: Read + Seek>(reader: &mut BufReader<T>, value_count: usize) -> Result<NetCdfValues, NetCdfError> {
        let mut float_values = vec![0 as f32; value_count];
        reader.read_f32_into::<BigEndian>(&mut float_values)?;

        let values = NetCdfValues::FloatValues(float_values);

        return Ok(values);
    }


    fn read_double_values<T: Read + Seek>(reader: &mut BufReader<T>, value_count: usize) -> Result<NetCdfValues, NetCdfError> {
        let mut double_values = vec![0 as f64; value_count];
        reader.read_f64_into::<BigEndian>(&mut double_values)?;

        let values = NetCdfValues::DoubleValues(double_values);

        return Ok(values);
    }


    fn read_padding<T: Read + Seek>(reader: &mut BufReader<T>, value_len: usize, value_size_bytes: u32) -> Result<(), NetCdfError> {
        let padding = value_len * value_size_bytes as usize % 4;
        if padding > 0 {
            reader.seek_relative(4 - padding as i64)?;
        }

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use byteorder::{BigEndian, ReadBytesExt};

    use crate::netcdf::common::netcdf_value_type::NetCdfValueType;
    use crate::netcdf::common::netcdf_values::NetCdfValues;
    use crate::netcdf::common::netcdf_values_reader::NetCdfValuesReader;

    #[test]
    fn it_reads_byte_values_correctly() {
        let mut reader = BufReader::new(Cursor::new([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C
        ]));

        let result = NetCdfValuesReader::read(&mut reader, 9, &NetCdfValueType::NcByte);
        assert!(result.is_ok());

        let byte_values = result.unwrap();
        match byte_values {
            NetCdfValues::ByteValues(values) => {
                assert_eq!(9, values.len());
                assert_eq!(0x01, values[0]);
                assert_eq!(0x09, values[8]);
            }
            _ => panic!("wrong value type {:?}", byte_values)
        };

        assert_eq!(12 as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_reads_char_values_correctly() {
        let mut reader = BufReader::new(Cursor::new([
            0x76, 0x65, 0x72, 0x74, 0x65, 0x78, 0x00, 0x00
        ]));

        let result = NetCdfValuesReader::read(&mut reader, 6, &NetCdfValueType::NcChar);
        assert!(result.is_ok());

        let byte_values = result.unwrap();
        match byte_values {
            NetCdfValues::CharValues(values) => {
                assert_eq!(6, values.len());
                assert_eq!('v', values[0]);
                assert_eq!('x', values[5]);
            }
            _ => panic!("wrong value type {:?}", byte_values)
        };

        assert_eq!(8 as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_reads_short_values_correctly() {
        let mut reader = BufReader::new(Cursor::new([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C
        ]));

        let result = NetCdfValuesReader::read(&mut reader, 3, &NetCdfValueType::NcShort);
        assert!(result.is_ok());

        let byte_values = result.unwrap();
        match byte_values {
            NetCdfValues::ShortValues(values) => {
                assert_eq!(3, values.len());
                assert_eq!(0x102, values[0]);
                assert_eq!(0x506, values[2]);
            }
            _ => panic!("wrong value type {:?}", byte_values)
        };

        assert_eq!(8 as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_reads_int_values_correctly() {
        let mut reader = BufReader::new(Cursor::new([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C
        ]));

        let result = NetCdfValuesReader::read(&mut reader, 2, &NetCdfValueType::NcInt);
        assert!(result.is_ok());

        let byte_values = result.unwrap();
        match byte_values {
            NetCdfValues::IntValues(values) => {
                assert_eq!(2, values.len());
                assert_eq!(0x01020304, values[0]);
                assert_eq!(0x05060708, values[1]);
            }
            _ => panic!("wrong value type {:?}", byte_values)
        };

        assert_eq!(8 as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_reads_float_values_correctly() {
        let mut reader = BufReader::new(Cursor::new([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C
        ]));

        let result = NetCdfValuesReader::read(&mut reader, 2, &NetCdfValueType::NcFloat);
        assert!(result.is_ok());

        let byte_values = result.unwrap();
        match byte_values {
            NetCdfValues::FloatValues(values) => {
                assert_eq!(2, values.len());
            }
            _ => panic!("wrong value type {:?}", byte_values)
        };

        assert_eq!(8 as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_reads_double_values_correctly() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x00, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C
        ]));

        let result = NetCdfValuesReader::read(&mut reader, 1, &NetCdfValueType::NcDouble);
        assert!(result.is_ok());

        let byte_values = result.unwrap();
        match byte_values {
            NetCdfValues::DoubleValues(values) => {
                assert_eq!(1, values.len());
            }
            _ => panic!("wrong value type {:?}", byte_values)
        };

        assert_eq!(8 as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_pads_bytes_correctly() {
        let mut reader = BufReader::new(Cursor::new([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C
        ]));

        reader.read_u8().unwrap();

        let result = NetCdfValuesReader::read_padding(&mut reader, 1, 1);
        assert!(result.is_ok());

        assert_eq!(4 as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_pads_shorts_correctly() {
        let mut reader = BufReader::new(Cursor::new([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C
        ]));

        reader.read_i16::<BigEndian>().unwrap();
        reader.read_i16::<BigEndian>().unwrap();
        reader.read_i16::<BigEndian>().unwrap();

        let result = NetCdfValuesReader::read_padding(&mut reader, 3, 2);
        assert!(result.is_ok());

        assert_eq!(8 as u64, reader.stream_position().unwrap())
    }
}
