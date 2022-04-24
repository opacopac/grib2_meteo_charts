use derive_more::Constructor;

#[derive(Debug, Constructor)]
pub struct ScanningModeFlags {
    pub scan_direction_first_row_i_is_positive: bool,
    pub scan_direction_first_row_j_is_negative: bool,
    pub adjacent_points_in_i_direction_consecutive: bool,
    pub all_rows_same_scan_direction: bool,
    pub odd_rows_offset_in_i_direction: bool,
    pub even_rows_offset_in_i_direction: bool,
    pub points_not_offset_in_j_direction: bool,
    pub rows_have_ni_points_cols_have_nj_points: bool
}
