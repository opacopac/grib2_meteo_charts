#[derive(Debug, PartialEq, Clone)]
pub enum IconD2EUForecastRunName {
    Run00,
    Run03,
    Run06,
    Run09,
    Run12,
    Run15,
    Run18,
    Run21
}


impl IconD2EUForecastRunName {
    pub fn get_all() -> Vec<IconD2EUForecastRunName> {
        vec![
            IconD2EUForecastRunName::Run00,
            IconD2EUForecastRunName::Run03,
            IconD2EUForecastRunName::Run06,
            IconD2EUForecastRunName::Run09,
            IconD2EUForecastRunName::Run12,
            IconD2EUForecastRunName::Run15,
            IconD2EUForecastRunName::Run18,
            IconD2EUForecastRunName::Run21
        ]
    }


    pub fn get_all_reversed() -> Vec<IconD2EUForecastRunName> {
        IconD2EUForecastRunName::get_all()
            .into_iter()
            .rev()
            .collect()
    }


    pub fn get_name(&self) -> String {
        match self {
            IconD2EUForecastRunName::Run00 => "00".to_string(),
            IconD2EUForecastRunName::Run03 => "03".to_string(),
            IconD2EUForecastRunName::Run06 => "06".to_string(),
            IconD2EUForecastRunName::Run09 => "09".to_string(),
            IconD2EUForecastRunName::Run12 => "12".to_string(),
            IconD2EUForecastRunName::Run15 => "15".to_string(),
            IconD2EUForecastRunName::Run18 => "18".to_string(),
            IconD2EUForecastRunName::Run21 => "21".to_string()
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::forecast_run::icon_d2_eu_forecast_run_name::IconD2EUForecastRunName;

    #[test]
    fn it_gets_the_correct_name() {
        let run_name = IconD2EUForecastRunName::Run12;

        let result = run_name.get_name();

        assert_eq!("12", result);
    }


    #[test]
    fn it_gets_the_full_list() {
        let result = IconD2EUForecastRunName::get_all();

        assert_eq!(8, result.len());
        assert_eq!(IconD2EUForecastRunName::Run00, result[0]);
        assert_eq!(IconD2EUForecastRunName::Run21, result[7]);
    }


    #[test]
    fn it_gets_the_reversed_list() {
        let result = IconD2EUForecastRunName::get_all_reversed();

        assert_eq!(8, result.len());
        assert_eq!(IconD2EUForecastRunName::Run21, result[0]);
        assert_eq!(IconD2EUForecastRunName::Run00, result[7]);
    }
}
