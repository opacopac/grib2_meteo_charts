use derive_more::Constructor;

#[derive(Debug, Constructor)]
pub struct ProductDefinitionTemplate4_0 {
    pub parameter_category: u8,
    pub parameter_number: u8,
    pub generating_process_type: u8,
    pub generating_process_identifier: u8,
    pub generating_process: u8
}

impl ProductDefinitionTemplate4_0 {
    pub const TPL_LENGTH_BYTES: u32 = 5;
}
