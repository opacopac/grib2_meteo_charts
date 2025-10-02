pub struct MeteoForecastRun2Step {
    idx: usize,
    file_url: String,
}


impl MeteoForecastRun2Step {
    pub fn new(idx: usize, file_url: String) -> MeteoForecastRun2Step {
        MeteoForecastRun2Step { idx, file_url }
    }


    pub fn get_index(&self) -> usize {
        self.idx
    }


    pub fn get_file_url(&self) -> String {
        self.file_url.clone()
    }
}
