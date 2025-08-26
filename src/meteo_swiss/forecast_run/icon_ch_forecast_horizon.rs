#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IconChForecastHorizon {
    pub days: u8,
    pub hours: u8,
}


impl IconChForecastHorizon {
    pub fn new(days: u8, hours: u8) -> Self {
        Self { days, hours }
    }


    pub fn create_zero() -> Self {
        Self { days: 0, hours: 0 }
    }


    pub fn from_step(step: u8) -> Self {
        let days = step / 24;
        let hours = step % 24;
        Self { days, hours }
    }


    pub fn get_step(&self) -> u8 {
        self.days * 24 + self.hours
    }


    pub fn get_name(&self) -> String {
        //Example: P0DT00H00M00S
        format!("P{}DT{:02}H00M00S", self.days, self.hours)
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_builds_a_zero_instance() {
        // given

        // when
        let horizon = super::IconChForecastHorizon::create_zero();

        // then
        assert_eq!(horizon.days, 0);
        assert_eq!(horizon.hours, 0);
    }


    #[test]
    fn it_builds_a_custom_instance() {
        // given

        // when
        let horizon = super::IconChForecastHorizon::new(1, 6);

        // then
        assert_eq!(horizon.days, 1);
        assert_eq!(horizon.hours, 6);
    }


    #[test]
    fn it_converts_a_step_to_a_horizon() {
        // given
        let step = 30;

        // when
        let horizon = super::IconChForecastHorizon::from_step(step);

        // then
        assert_eq!(horizon.days, 1);
        assert_eq!(horizon.hours, 6);
    }


    #[test]
    fn it_converts_a_horizon_to_a_step() {
        // given
        let horizon = super::IconChForecastHorizon::new(1, 6);

        // when
        let step = horizon.get_step();

        // then
        assert_eq!(step, 30);
    }


    #[test]
    fn it_gets_the_name_of_a_horizon() {
        // given
        let horizon = super::IconChForecastHorizon::new(1, 6);

        // when
        let name = horizon.get_name();

        // then
        assert_eq!(name, "P1DT06H00M00S".to_string());
    }
}
