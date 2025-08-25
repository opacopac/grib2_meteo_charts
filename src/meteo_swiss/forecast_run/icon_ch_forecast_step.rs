pub struct IconCh1ForecastStep {
    pub title: String,
    pub step: String,
    pub href: String,
}


impl IconCh1ForecastStep {
    pub fn new(title: String, step: String, href: String) -> Self {
        Self { title, step, href }
    }
}


impl Iterator for IconCh1ForecastStep {
    type Item = IconCh1ForecastStep;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
