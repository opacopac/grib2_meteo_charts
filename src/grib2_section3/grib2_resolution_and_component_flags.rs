pub struct Grib2ResolutionAndComponentFlags {
    pub has_i_direction_increments: bool,
    pub has_j_direction_increments: bool,
    pub u_v_relative_to_e_n: bool
}


impl Grib2ResolutionAndComponentFlags {
    pub fn new(
        has_i_direction_increments: bool,
        has_j_direction_increments: bool,
        u_v_relative_to_e_n: bool
    ) -> Grib2ResolutionAndComponentFlags {
        return Grib2ResolutionAndComponentFlags {
            has_i_direction_increments,
            has_j_direction_increments,
            u_v_relative_to_e_n
        }
    }
}
