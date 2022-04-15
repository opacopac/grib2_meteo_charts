#[cfg(test)]
mod grib2_cloud_cover_reader_tests {
    use crate::grib2_cloud_cover::grib2_cloud_cover_reader::Grib2CloudCoverReader;

    #[test]
    fn it_parses_a_grib2_file() {
        let grib2_file = "TODO.grib2";

        let result = Grib2CloudCoverReader::read_file(grib2_file);

        assert_eq!(false, result.is_err());
    }
}
