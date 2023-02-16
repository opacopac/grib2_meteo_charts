use crate::dwd::forecast_run::dwd_model_type::DwdModelType;

pub struct DwdModelConfig {
    pub model: DwdModelType,
    pub step_hours: Vec<u8>,
    pub run_hours_utc: Vec<u8>,
}


impl DwdModelConfig {
    pub fn get_dwd_model_config(model: DwdModelType) -> DwdModelConfig {
        return match model {
            DwdModelType::IconD2 => DwdModelConfig {
                model: DwdModelType::IconD2,
                step_hours: (0..48).collect(),
                run_hours_utc: vec![0, 3, 6, 9, 12, 15, 18, 21],
            },
            DwdModelType::IconEu => DwdModelConfig {
                model: DwdModelType::IconEu,
                step_hours: (0..78).collect(), // TODO: 81, 84, ...120
                run_hours_utc: vec![0, 6, 12, 18]
            },
            DwdModelType::Icon => DwdModelConfig {
                model: DwdModelType::Icon,
                step_hours: (0..78).collect(),  // TODO: 81, 84, ...120/180
                run_hours_utc: vec![0, 6, 12, 18]
            }
        }
    }
}
