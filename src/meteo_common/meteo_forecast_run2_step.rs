use std::fmt::Display;
use crate::meteo_common::meteo_common_error::MeteoCommonError;


#[derive(Debug, Clone)]
pub struct MeteoForecastRun2Step {
    step_nr: usize,
    file_url: String,
}


impl Display for MeteoForecastRun2Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.step_nr)
    }
}


impl MeteoForecastRun2Step {
    pub fn new(step_nr: usize, file_url: String) -> MeteoForecastRun2Step {
        MeteoForecastRun2Step { step_nr, file_url }
    }


    pub fn get_step_nr(&self) -> usize {
        self.step_nr
    }


    pub fn get_file_url(&self) -> String {
        self.file_url.clone()
    }


    pub fn get_step_by_nr(step_list: &Vec<MeteoForecastRun2Step>, step_nr: usize) -> Result<&MeteoForecastRun2Step, MeteoCommonError> {
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
    use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;


    #[test]
    fn test_it_gets_the_correct_step_by_nr_in_a_list() {
        // given
        let step1 = MeteoForecastRun2Step::new(0, "url1".to_string());
        let step2 = MeteoForecastRun2Step::new(3, "url2".to_string());
        let step3 = MeteoForecastRun2Step::new(6, "url3".to_string());
        let step_list = vec![step1.clone(), step2.clone(), step3.clone()];

        // when
        let found_step_result = MeteoForecastRun2Step::get_step_by_nr(&step_list, 3);

        // then
        assert!(found_step_result.is_ok());
        let found_step = found_step_result.unwrap();
        assert_eq!(found_step.get_step_nr(), step2.step_nr);
        assert_eq!(found_step.get_file_url(), step2.file_url);
    }
}
