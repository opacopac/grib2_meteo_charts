#[derive(Debug, PartialEq, Clone)]
pub enum DwdModelType {
    IconD2,
    IconEu,
    Icon,
}


impl DwdModelType {
    pub fn get_name(&self) -> String {
        match self {
            DwdModelType::IconD2 => "icon-d2".to_string(),
            DwdModelType::IconEu => "icon-eu".to_string(),
            DwdModelType::Icon => "icon-global".to_string(),
        }
    }
}
