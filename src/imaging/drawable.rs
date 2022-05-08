use image::ColorType;

use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::NUM_COLOR_VALUES;

pub struct Drawable {
    color_values: Vec<u8>,
    width: u32,
    height: u32
}


impl Drawable {
    const INIT_COLOR_VALUE: u8 = 0;


    pub fn create_with_data(width: u32, height: u32, px_rows: Vec<Vec<[u8; 4]>>) -> Result<Drawable, Grib2Error> {
        let mut drawable = Drawable::create_empty(width, height)?;
        let mut y = 0;

        for px_row in px_rows {
            let mut x = 0;
            for px in px_row {
                drawable.draw_point(x, height - y - 1, px);
                x += 1;
            }
            y += 1;
        }

        return Result::Ok(drawable);
    }


    pub fn create_empty(width: u32, height: u32) -> Result<Drawable, Grib2Error> {
        if width == 0 || height == 0 {
            return Err(Grib2Error::InvalidData(String::from("width/height must not be 0")));
        }

        let px_count = (width * height * NUM_COLOR_VALUES) as usize;
        let mut color_values = Vec::new();
        color_values.resize(px_count, Drawable::INIT_COLOR_VALUE);

        return Result::Ok(Drawable { color_values, width, height });
    }


    pub fn width(&self) -> u32 {
        return self.width;
    }


    pub fn height(&self) -> u32 {
        return self.height;
    }


    pub fn draw_point(&mut self, x: u32, y: u32, color: [u8; 4]) {
        if x >= self.width || y >= self.height {
            panic!("coordinates out of bound");
        }

        let idx = ((y * self.width + x) * NUM_COLOR_VALUES) as usize;

        self.color_values[idx] = color[0];
        self.color_values[idx + 1] = color[1];
        self.color_values[idx + 2] = color[2];
        self.color_values[idx + 3] = color[3];
    }


    pub fn draw_line(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, color: [u8; 4]) {
        let mut x0 = x0;
        let mut y0 = y0;
        let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
        let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err2;

        loop {
            self.draw_point(x0 as u32, y0 as u32, color);

            if x0 == x1 && y0 == y1 {
                break
            };

            err2 = 2 * err;
            if err2 > -dx {
                err -= dy;
                x0 += sx;
            }
            if err2 < dy {
                err += dx;
                y0 += sy;
            }
        }
    }


    pub fn safe_image(&self, filename: &str) -> Result<(), Grib2Error> {
        image::save_buffer(
            filename,
            &*self.color_values,
            self.width,
            self.height,
            ColorType::Rgba8
        )?;

        return Result::Ok(());
    }
}
