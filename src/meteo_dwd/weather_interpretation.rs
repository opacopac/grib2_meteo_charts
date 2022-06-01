#[derive(Debug)]
pub enum WeatherInterpretation {
    ClearSky = 0,
    MainlyClearSky = 1,
    PartlyCloudy = 2,
    Overcast = 3,
    Fog = 45,
    FogDepositingRime = 48,
    SlightDrizzle = 51,
    ModerateDrizzle = 53,
    HeavyDrizzle = 55,
    DrizzleFreezingSlight = 56,
    DrizzleFreezingModerateOrHeavy = 57,
    SlightRainNotFreezing = 61,
    ModerateRainNotFreezing = 63,
    HeavyRainNotFreezing = 65,
    RainFreezingSlight = 66,
    RainFreezingModerateOrHeavy = 67,
    SlightFallOfSnowflakes = 71,
    ModerateFallOfSnowflakes = 73,
    HeavyFallOfSnowflakes = 75,
    SnowGrains = 77,
    RainShowerSlight = 80,
    RainShowerModerateOrHeavy = 81,
    RainShowerViolent = 83,
    SnowShowerSlight = 85,
    SnowShowerModerateOrHeavy = 86,
    ThunderstormSlightOrModerate = 95,
    ThunderstormWithHailOrHeavyThunderstorm = 96
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
            83 => WeatherInterpretation::RainShowerViolent,
            85 => WeatherInterpretation::SnowShowerSlight,
            86 => WeatherInterpretation::SnowShowerModerateOrHeavy,
            95 => WeatherInterpretation::ThunderstormSlightOrModerate,
            96 => WeatherInterpretation::ThunderstormWithHailOrHeavyThunderstorm,
            _ => panic!("unknown value {} for weather interpretation", value)
        }
    }
}
