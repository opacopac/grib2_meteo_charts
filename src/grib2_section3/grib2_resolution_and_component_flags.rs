pub struct Grib2ResolutionAndComponentFlags {
    pub i_direction_increments_not_given: bool,
    pub j_direction_increments_not_given: bool,
    pub u_v_relative_to_e_n: bool
}


impl Grib2ResolutionAndComponentFlags {
    pub fn new(
        has_i_direction_increments: bool,
        has_j_direction_increments: bool,
        u_v_relative_to_e_n: bool
    ) -> Grib2ResolutionAndComponentFlags {
        return Grib2ResolutionAndComponentFlags {
            i_direction_increments_not_given: has_i_direction_increments,
            j_direction_increments_not_given: has_j_direction_increments,
            u_v_relative_to_e_n
        }
    }
}
