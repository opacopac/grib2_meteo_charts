pub struct MeteoForecastRun2Step {
    step_nr: usize,
    file_url: String,
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
}
