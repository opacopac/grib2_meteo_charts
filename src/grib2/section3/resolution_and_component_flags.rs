#[derive(Debug)]
pub struct ResolutionAndComponentFlags {
    pub i_direction_increments_not_given: bool,
    pub j_direction_increments_not_given: bool,
    pub u_v_relative_to_e_n: bool
}


impl ResolutionAndComponentFlags {
    pub fn new(
        has_i_direction_increments: bool,
        has_j_direction_increments: bool,
        u_v_relative_to_e_n: bool
    ) -> ResolutionAndComponentFlags {
        return ResolutionAndComponentFlags {
            i_direction_increments_not_given: has_i_direction_increments,
            j_direction_increments_not_given: has_j_direction_increments,
            u_v_relative_to_e_n
        }
    }
}
