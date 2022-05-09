use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::image::Image;

const WIND_ARROW_IMG_FILES: [&str; 17] = [
    "./resources/wind_arrows/wind_0kt.png",
    "./resources/wind_arrows/wind_1-2kt.png",
    "./resources/wind_arrows/wind_5kt.png",
    "./resources/wind_arrows/wind_10kt.png",
    "./resources/wind_arrows/wind_15kt.png",
    "./resources/wind_arrows/wind_20kt.png",
    "./resources/wind_arrows/wind_25kt.png",
    "./resources/wind_arrows/wind_30kt.png",
    "./resources/wind_arrows/wind_35kt.png",
    "./resources/wind_arrows/wind_40kt.png",
    "./resources/wind_arrows/wind_45kt.png",
    "./resources/wind_arrows/wind_50kt.png",
    "./resources/wind_arrows/wind_60kt.png",
    "./resources/wind_arrows/wind_70kt.png",
    "./resources/wind_arrows/wind_80kt.png",
    "./resources/wind_arrows/wind_90kt.png",
    "./resources/wind_arrows/wind_100kt.png"
];


pub struct WindArrowService {
    wind_arrow_imgs: Vec<Image>
}


impl WindArrowService {
    pub fn new() -> Result<WindArrowService, Grib2Error> {
        let wind_arrow_imgs = Self::load_wind_arrows()?;
        let service = WindArrowService { wind_arrow_imgs };

        return Ok(service);
    }


    pub fn get_arrow(&self, wind_speed_kts: f32) -> Result<&Image, Grib2Error> {
        let idx = Self::get_arrow_index(wind_speed_kts) as usize;
        let img = &self.wind_arrow_imgs[idx];

        return Ok(img);
    }


    pub fn get_arrow_index(wind_speed_kts: f32) -> u32 {
        let speed = wind_speed_kts.round() as u32;
        return match speed {
            0..=0 => 0,   // 0kt
            1..=2 => 1,   // 1-2kt
            3..=7 => 2,   // 5kt
            8..=12 => 3,  // 10kt
            13..=17 => 4, // 15kt
            18..=22 => 5, // 20kt
            23..=27 => 6, // 25kt
            28..=32 => 7, // 30kt
            33..=37 => 8,   // 35kt
            38..=42 => 10,  // 40kt
            43..=47 => 10,  // 45kt
            48..=55 => 10,  // 50kt
            56..=65 => 10,  // 60kt
            66..=75 => 10,  // 70kt
            76..=85 => 10,  // 80kt
            86..=95 => 10,  // 90kt
            _ => 100           // 100kt +
        }
    }


    fn load_wind_arrows() -> Result<Vec<Image>, Grib2Error> {
        let mut images: Vec<Image> = vec![];

        for img_file in WIND_ARROW_IMG_FILES {
            let img = Image::load_img(img_file)?;
            images.push(img);
        }

        return Ok(images);
    }
}
