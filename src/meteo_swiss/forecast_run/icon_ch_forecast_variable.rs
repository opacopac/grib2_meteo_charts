#[derive(Debug, PartialEq, Clone)]
pub enum IconChForecastVariable {
    Ceiling,
    Clc,
    Clct,
    T2m,
    U,
    U10m,
    V,
    V10m,
    VMax10m,
}


impl IconChForecastVariable {
    pub fn get_name(&self) -> String {
        match self {
            IconChForecastVariable::Ceiling => "CEILING".to_string(),
            IconChForecastVariable::Clc => "CLC".to_string(),
            IconChForecastVariable::Clct => "CLCT".to_string(),
            IconChForecastVariable::T2m => "T_2M".to_string(),
            IconChForecastVariable::U => "U".to_string(),
            IconChForecastVariable::U10m => "U_10M".to_string(),
            IconChForecastVariable::V => "V".to_string(),
            IconChForecastVariable::V10m => "V_10M".to_string(),
            IconChForecastVariable::VMax10m => "VMAX_10M".to_string(),
        }
    }
}
