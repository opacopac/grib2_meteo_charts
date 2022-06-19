#[derive(Debug, PartialEq)]
pub enum IconD2ForecastRunName {
    Run00,
    Run03,
    Run06,
    Run09,
    Run12,
    Run15,
    Run18,
    Run21
}


impl IconD2ForecastRunName {
    pub fn get_all() -> Vec<IconD2ForecastRunName> {
        return vec![
            IconD2ForecastRunName::Run00,
            IconD2ForecastRunName::Run03,
            IconD2ForecastRunName::Run06,
            IconD2ForecastRunName::Run09,
            IconD2ForecastRunName::Run12,
            IconD2ForecastRunName::Run15,
            IconD2ForecastRunName::Run18,
            IconD2ForecastRunName::Run21
        ];
    }


    pub fn get_all_reversed() -> Vec<IconD2ForecastRunName> {
        let mut all = IconD2ForecastRunName::get_all();
        all.reverse();

        return all;
    }


    pub fn get_name(&self) -> String {
        return match self {
            IconD2ForecastRunName::Run00 => "00".to_string(),
            IconD2ForecastRunName::Run03 => "03".to_string(),
            IconD2ForecastRunName::Run06 => "06".to_string(),
            IconD2ForecastRunName::Run09 => "09".to_string(),
            IconD2ForecastRunName::Run12 => "12".to_string(),
            IconD2ForecastRunName::Run15 => "15".to_string(),
            IconD2ForecastRunName::Run18 => "18".to_string(),
            IconD2ForecastRunName::Run21 => "21".to_string()
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd_forecast_runs::icon_d2_forecast_run_name::IconD2ForecastRunName;

    #[test]
    fn it_gets_the_correct_name() {
        let run_name = IconD2ForecastRunName::Run12;

        let result = run_name.get_name();

        assert_eq!("12", result);
    }


    #[test]
    fn it_gets_the_full_list() {
        let result = IconD2ForecastRunName::get_all();

        assert_eq!(8, result.len());
        assert_eq!(IconD2ForecastRunName::Run00, result[0]);
        assert_eq!(IconD2ForecastRunName::Run21, result[7]);
    }


    #[test]
    fn it_gets_the_reversed_list() {
        let result = IconD2ForecastRunName::get_all_reversed();

        assert_eq!(8, result.len());
        assert_eq!(IconD2ForecastRunName::Run21, result[0]);
        assert_eq!(IconD2ForecastRunName::Run00, result[7]);
    }
}
