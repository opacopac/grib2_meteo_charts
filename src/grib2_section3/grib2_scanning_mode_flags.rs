pub struct Grib2ScanningModeFlags {
    pub scan_direction_first_row_i_is_positive: bool,
    pub scan_direction_first_row_j_is_negative: bool,
    pub adjacent_points_in_i_direction_consecutive: bool,
    pub all_rows_same_scan_direction: bool,
    pub odd_rows_offset_in_i_direction: bool,
    pub even_rows_offset_in_i_direction: bool,
    pub points_not_offset_in_j_direction: bool,
    pub rows_have_ni_points_cols_have_nj_points: bool
}


impl Grib2ScanningModeFlags {
    pub fn new(
        scan_direction_first_row_i_is_positive: bool,
        scan_direction_first_row_j_is_negative: bool,
        adjacent_points_in_i_direction_consecutive: bool,
        all_rows_same_scan_direction: bool,
        odd_rows_offset_in_i_direction: bool,
        even_rows_offset_in_i_direction: bool,
        points_not_offset_in_j_direction: bool,
        rows_have_ni_points_cols_have_nj_points: bool
    ) -> Grib2ScanningModeFlags {
        return Grib2ScanningModeFlags {
            scan_direction_first_row_i_is_positive,
            scan_direction_first_row_j_is_negative,
            adjacent_points_in_i_direction_consecutive,
            all_rows_same_scan_direction,
            odd_rows_offset_in_i_direction,
            even_rows_offset_in_i_direction,
            points_not_offset_in_j_direction,
            rows_have_ni_points_cols_have_nj_points
        }
    }
}
