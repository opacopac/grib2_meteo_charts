use std::fmt::Display;
use crate::meteo_common::meteo_common_error::MeteoCommonError;


#[derive(Debug, Clone)]
pub struct MeteoForecastRunStep {
    step_nr: usize,
    file_url: String,
}


impl Display for MeteoForecastRunStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.step_nr)
    }
}


impl MeteoForecastRunStep {
    pub fn new(step_nr: usize, file_url: String) -> MeteoForecastRunStep {
        MeteoForecastRunStep { step_nr, file_url }
    }


    pub fn get_step_nr(&self) -> usize {
        self.step_nr
    }


    pub fn get_file_url(&self) -> &str {
        self.file_url.as_str()
    }


    pub fn get_step_by_nr(step_list: &Vec<MeteoForecastRunStep>, step_nr: usize) -> Result<&MeteoForecastRunStep, MeteoCommonError> {
        let step = step_list
            .iter()
            .find(|s| s.get_step_nr() == step_nr);

        match step {
            Some(s) => Ok(s),
            None => Err(MeteoCommonError::InvalidStepNrError(step_nr)),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


    #[test]
    fn test_it_gets_the_correct_step_by_nr_in_a_list() {
        // given
        let step1 = MeteoForecastRunStep::new(0, "url1".to_string());
        let step2 = MeteoForecastRunStep::new(3, "url2".to_string());
        let step3 = MeteoForecastRunStep::new(6, "url3".to_string());
        let step_list = vec![step1.clone(), step2.clone(), step3.clone()];

        // when
        let found_step_result = MeteoForecastRunStep::get_step_by_nr(&step_list, 3);

        // then
        assert!(found_step_result.is_ok());
        let found_step = found_step_result.unwrap();
        assert_eq!(found_step.get_step_nr(), step2.step_nr);
        assert_eq!(found_step.get_file_url(), step2.file_url);
    }
}
