use derive_new::new;

#[derive(Debug, new)]
pub struct ScaleFactorValue {
    pub factor: u8,
    pub value: u32
}
