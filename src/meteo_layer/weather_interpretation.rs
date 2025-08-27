#[derive(Debug, PartialEq)]
pub enum WeatherInterpretation {
    ClearSky,
    MainlyClearSky,
    PartlyCloudy,
    Overcast,
    Fog,
    FogDepositingRime,
    SlightDrizzle,
    ModerateDrizzle,
    HeavyDrizzle,
    DrizzleFreezingSlight,
    DrizzleFreezingModerateOrHeavy,
    SlightRainNotFreezing,
    ModerateRainNotFreezing,
    HeavyRainNotFreezing,
    RainFreezingSlight,
    RainFreezingModerateOrHeavy,
    SlightFallOfSnowflakes,
    ModerateFallOfSnowflakes,
    HeavyFallOfSnowflakes,
    SnowGrains,
    RainShowerSlight,
    RainShowerModerateOrHeavy,
    RainShowerViolent,
    SnowShowerSlight,
    SnowShowerModerateOrHeavy,
    ThunderstormSlightOrModerate,
    ThunderstormWithHailOrHeavyThunderstorm,
    ThunderstormWithHeavyRain,
}


impl WeatherInterpretation {
    pub fn from_value(value: u8) -> WeatherInterpretation {
        return match value {
            0 => WeatherInterpretation::ClearSky,
            1 => WeatherInterpretation::MainlyClearSky,
            2 => WeatherInterpretation::PartlyCloudy,
            3 => WeatherInterpretation::Overcast,
            45 => WeatherInterpretation::Fog,
            48 => WeatherInterpretation::FogDepositingRime,
            51 => WeatherInterpretation::SlightDrizzle,
            53 => WeatherInterpretation::ModerateDrizzle,
            55 => WeatherInterpretation::HeavyDrizzle,
            56 => WeatherInterpretation::DrizzleFreezingSlight,
            57 => WeatherInterpretation::DrizzleFreezingModerateOrHeavy,
            61 => WeatherInterpretation::SlightRainNotFreezing,
            63 => WeatherInterpretation::ModerateRainNotFreezing,
            65 => WeatherInterpretation::HeavyRainNotFreezing,
            66 => WeatherInterpretation::RainFreezingSlight,
            67 => WeatherInterpretation::RainFreezingModerateOrHeavy,
            71 => WeatherInterpretation::SlightFallOfSnowflakes,
            73 => WeatherInterpretation::ModerateFallOfSnowflakes,
            75 => WeatherInterpretation::HeavyFallOfSnowflakes,
            77 => WeatherInterpretation::SnowGrains,
            80 => WeatherInterpretation::RainShowerSlight,
            81 => WeatherInterpretation::RainShowerModerateOrHeavy,
            82 => WeatherInterpretation::RainShowerViolent,
            85 => WeatherInterpretation::SnowShowerSlight,
            86 => WeatherInterpretation::SnowShowerModerateOrHeavy,
            95 => WeatherInterpretation::ThunderstormSlightOrModerate,
            96 => WeatherInterpretation::ThunderstormWithHailOrHeavyThunderstorm,
            99 => WeatherInterpretation::ThunderstormWithHeavyRain,
            _ => panic!("unknown value {} for weather interpretation", value)
        };
    }


    pub fn to_value(&self) -> u8 {
        match self {
            WeatherInterpretation::ClearSky => 0,
            WeatherInterpretation::MainlyClearSky => 1,
            WeatherInterpretation::PartlyCloudy => 2,
            WeatherInterpretation::Overcast => 3,
            WeatherInterpretation::Fog => 45,
            WeatherInterpretation::FogDepositingRime => 48,
            WeatherInterpretation::SlightDrizzle => 51,
            WeatherInterpretation::ModerateDrizzle => 53,
            WeatherInterpretation::HeavyDrizzle => 55,
            WeatherInterpretation::DrizzleFreezingSlight => 56,
            WeatherInterpretation::DrizzleFreezingModerateOrHeavy => 57,
            WeatherInterpretation::SlightRainNotFreezing => 61,
            WeatherInterpretation::ModerateRainNotFreezing => 63,
            WeatherInterpretation::HeavyRainNotFreezing => 65,
            WeatherInterpretation::RainFreezingSlight => 66,
            WeatherInterpretation::RainFreezingModerateOrHeavy => 67,
            WeatherInterpretation::SlightFallOfSnowflakes => 71,
            WeatherInterpretation::ModerateFallOfSnowflakes => 73,
            WeatherInterpretation::HeavyFallOfSnowflakes => 75,
            WeatherInterpretation::SnowGrains => 77,
            WeatherInterpretation::RainShowerSlight => 80,
            WeatherInterpretation::RainShowerModerateOrHeavy => 81,
            WeatherInterpretation::RainShowerViolent => 82,
            WeatherInterpretation::SnowShowerSlight => 85,
            WeatherInterpretation::SnowShowerModerateOrHeavy => 86,
            WeatherInterpretation::ThunderstormSlightOrModerate => 95,
            WeatherInterpretation::ThunderstormWithHailOrHeavyThunderstorm => 96,
            WeatherInterpretation::ThunderstormWithHeavyRain => 99
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::meteo_layer::weather_interpretation::WeatherInterpretation;

    #[test]
    fn it_converts_from_a_byte_value() {
        let in_value = 45;

        let result = WeatherInterpretation::from_value(in_value);

        assert_eq!(WeatherInterpretation::Fog, result);
    }


    #[test]
    fn it_converts_to_a_byte_value() {
        let in_value = WeatherInterpretation::SnowGrains;

        let result = in_value.to_value();

        assert_eq!(77, result);
    }
}
