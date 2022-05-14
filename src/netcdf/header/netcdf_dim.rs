use derive_new::new;

#[derive(new)]
pub struct NetCdfDim {
    pub name: String,
    pub length: u32
}
