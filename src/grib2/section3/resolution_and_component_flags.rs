use derive_more::Constructor;

#[derive(Debug, Constructor)]
pub struct ResolutionAndComponentFlags {
    pub i_direction_increments_not_given: bool,
    pub j_direction_increments_not_given: bool,
    pub u_v_relative_to_e_n: bool
}
