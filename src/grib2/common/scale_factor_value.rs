use derive_more::Constructor;

#[derive(Debug, Constructor)]
pub struct ScaleFactorValue {
    pub factor: u8,
    pub value: u32
}
