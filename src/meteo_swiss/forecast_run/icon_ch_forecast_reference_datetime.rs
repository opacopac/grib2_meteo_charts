use crate::meteo_swiss::common::meteo_swiss_error::MeteoSwissError;
use chrono::{SecondsFormat, Timelike};


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IconChForecastReferenceDateTime {
    pub datetime: chrono::DateTime<chrono::Utc>,
}


impl IconChForecastReferenceDateTime {
    pub fn get_latest(datetime: chrono::DateTime<chrono::Utc>) -> Self {
        let hour = datetime.hour() - (datetime.hour() % 3);
        let latest_datetime = datetime
            .with_hour(hour)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        Self { datetime: latest_datetime }
    }


    pub fn from_str(datetime_str: &str) -> Result<Self, MeteoSwissError> {
        let datetime = chrono::DateTime::parse_from_rfc3339(datetime_str)?;

        // only allow datetimes that are on a 3-hour step
        if datetime.minute() != 0 || datetime.second() != 0 || datetime.hour() % 3 != 0 {
            let err_msg = "Only datetimes on a 3-hour step are allowed (e.g., 00:00, 03:00, 06:00, 09:00, 12:00, 15:00, 18:00, 21:00)".to_string();
            return Err(MeteoSwissError::InvalidParameters(err_msg));
        }

        Ok(Self { datetime: datetime.with_timezone(&chrono::Utc) })
    }


    // example: 2025-08-25T12:00:00Z
    pub fn get_name(&self) -> String {
        self.datetime
            .to_rfc3339_opts(SecondsFormat::Secs, true)
    }


    pub fn get_date(&self) -> chrono::NaiveDate {
        self.datetime.date_naive()
    }
}


#[cfg(test)]
mod tests {
    use crate::meteo_swiss::forecast_run::icon_ch_forecast_reference_datetime::IconChForecastReferenceDateTime;
    use chrono::Timelike;


    #[test]
    fn it_creates_an_instance_from_a_str() {
        // given
        let datetime_str = "2025-08-25T12:00:00Z";
        let datetime = chrono::DateTime::parse_from_rfc3339(datetime_str).unwrap();

        // when
        let reference_datetime = IconChForecastReferenceDateTime::from_str(datetime_str)
            .unwrap();

        // then
        assert_eq!(reference_datetime.datetime, datetime);
    }


    #[test]
    fn it_gets_the_name_of_an_instance() {
        // given
        let datetime_str = "2025-08-25T12:00:00Z";
        let reference_datetime = IconChForecastReferenceDateTime::from_str(datetime_str).unwrap();

        // when
        let name = reference_datetime.get_name();

        // then
        assert_eq!(name, datetime_str);
    }


    #[test]
    fn it_gets_the_date_of_an_instance() {
        // given
        let datetime_str = "2025-08-25T12:00:00Z";
        let reference_datetime = IconChForecastReferenceDateTime::from_str(datetime_str).unwrap();

        // when
        let date = reference_datetime.get_date();

        // then
        assert_eq!(date, chrono::NaiveDate::from_ymd_opt(2025, 8, 25).unwrap());
    }


    #[test]
    fn it_only_allows_three_hour_steps() {
        // given
        let datetime_str1 = "2025-08-25T12:00:00Z";
        let datetime_str2 = "2025-08-25T13:00:00Z";

        // when
        let reference_datetime1 = IconChForecastReferenceDateTime::from_str(datetime_str1);
        let reference_datetime2 = IconChForecastReferenceDateTime::from_str(datetime_str2);

        // then
        assert!(reference_datetime1.is_ok());
        assert!(reference_datetime2.is_err());
    }


    #[test]
    fn it_gets_the_latest_reference_datetime() {
        // given
        let datetime_str = "2025-08-25T14:37:22Z";
        let datetime = chrono::DateTime::parse_from_rfc3339(datetime_str)
            .unwrap()
            .with_timezone(&chrono::Utc);

        // when
        let reference_datetime = IconChForecastReferenceDateTime::get_latest(datetime);

        // then
        assert_eq!(reference_datetime.datetime.hour(), 12);
        assert_eq!(reference_datetime.datetime.minute(), 0);
        assert_eq!(reference_datetime.datetime.second(), 0);
    }
}
