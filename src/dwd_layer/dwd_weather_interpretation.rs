#[derive(Debug, PartialEq)]
pub enum DwdWeatherInterpretation {
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
    ThunderstormWithHeavyRain
}


impl DwdWeatherInterpretation {
    pub fn from_value(value: u8) -> DwdWeatherInterpretation {
        return match value {
            0 => DwdWeatherInterpretation::ClearSky,
            1 => DwdWeatherInterpretation::MainlyClearSky,
            2 => DwdWeatherInterpretation::PartlyCloudy,
            3 => DwdWeatherInterpretation::Overcast,
            45 => DwdWeatherInterpretation::Fog,
            48 => DwdWeatherInterpretation::FogDepositingRime,
            51 => DwdWeatherInterpretation::SlightDrizzle,
            53 => DwdWeatherInterpretation::ModerateDrizzle,
            55 => DwdWeatherInterpretation::HeavyDrizzle,
            56 => DwdWeatherInterpretation::DrizzleFreezingSlight,
            57 => DwdWeatherInterpretation::DrizzleFreezingModerateOrHeavy,
            61 => DwdWeatherInterpretation::SlightRainNotFreezing,
            63 => DwdWeatherInterpretation::ModerateRainNotFreezing,
            65 => DwdWeatherInterpretation::HeavyRainNotFreezing,
            66 => DwdWeatherInterpretation::RainFreezingSlight,
            67 => DwdWeatherInterpretation::RainFreezingModerateOrHeavy,
            71 => DwdWeatherInterpretation::SlightFallOfSnowflakes,
            73 => DwdWeatherInterpretation::ModerateFallOfSnowflakes,
            75 => DwdWeatherInterpretation::HeavyFallOfSnowflakes,
            77 => DwdWeatherInterpretation::SnowGrains,
            80 => DwdWeatherInterpretation::RainShowerSlight,
            81 => DwdWeatherInterpretation::RainShowerModerateOrHeavy,
            82 => DwdWeatherInterpretation::RainShowerViolent,
            85 => DwdWeatherInterpretation::SnowShowerSlight,
            86 => DwdWeatherInterpretation::SnowShowerModerateOrHeavy,
            95 => DwdWeatherInterpretation::ThunderstormSlightOrModerate,
            96 => DwdWeatherInterpretation::ThunderstormWithHailOrHeavyThunderstorm,
            99 => DwdWeatherInterpretation::ThunderstormWithHeavyRain,
            _ => panic!("unknown value {} for weather interpretation", value)
        }
    }


    pub fn to_value(&self) -> u8 {
        return match self {
            DwdWeatherInterpretation::ClearSky => 0,
            DwdWeatherInterpretation::MainlyClearSky => 1,
            DwdWeatherInterpretation::PartlyCloudy => 2,
            DwdWeatherInterpretation::Overcast => 3,
            DwdWeatherInterpretation::Fog => 45,
            DwdWeatherInterpretation::FogDepositingRime => 48,
            DwdWeatherInterpretation::SlightDrizzle => 51,
            DwdWeatherInterpretation::ModerateDrizzle => 53,
            DwdWeatherInterpretation::HeavyDrizzle => 55,
            DwdWeatherInterpretation::DrizzleFreezingSlight => 56,
            DwdWeatherInterpretation::DrizzleFreezingModerateOrHeavy => 57,
            DwdWeatherInterpretation::SlightRainNotFreezing => 61,
            DwdWeatherInterpretation::ModerateRainNotFreezing => 63,
            DwdWeatherInterpretation::HeavyRainNotFreezing => 65,
            DwdWeatherInterpretation::RainFreezingSlight => 66,
            DwdWeatherInterpretation::RainFreezingModerateOrHeavy => 67,
            DwdWeatherInterpretation::SlightFallOfSnowflakes => 71,
            DwdWeatherInterpretation::ModerateFallOfSnowflakes => 73,
            DwdWeatherInterpretation::HeavyFallOfSnowflakes => 75,
            DwdWeatherInterpretation::SnowGrains => 77,
            DwdWeatherInterpretation::RainShowerSlight => 80,
            DwdWeatherInterpretation::RainShowerModerateOrHeavy => 81,
            DwdWeatherInterpretation::RainShowerViolent => 82,
            DwdWeatherInterpretation::SnowShowerSlight => 85,
            DwdWeatherInterpretation::SnowShowerModerateOrHeavy => 86,
            DwdWeatherInterpretation::ThunderstormSlightOrModerate => 95,
            DwdWeatherInterpretation::ThunderstormWithHailOrHeavyThunderstorm => 96,
            DwdWeatherInterpretation::ThunderstormWithHeavyRain => 99
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::dwd_layer::dwd_weather_interpretation::DwdWeatherInterpretation;

    #[test]
    fn it_converts_from_a_byte_value() {
        let in_value = 45;

        let result = DwdWeatherInterpretation::from_value(in_value);

        assert_eq!(DwdWeatherInterpretation::Fog, result);
    }


    #[test]
    fn it_converts_to_a_byte_value() {
        let in_value = DwdWeatherInterpretation::SnowGrains;

        let result = in_value.to_value();

        assert_eq!(77, result);
    }
}
