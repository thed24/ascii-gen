use image::{DynamicImage, GenericImageView, Pixel};

/// Options for the ASCII art conversion.
pub struct AsciiOptions {
    width: u32,
    height: u32,
}

/// Default implementation for the ASCII art conversion options.
impl AsciiOptions {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

/// Default implementation for the ASCII art conversion options.
impl Default for AsciiOptions {
    fn default() -> Self {
        Self::new(80, 80)
    }
}

/// Trait for converting something generically to ASCII art.
pub trait ToAsciiArt {
    fn to_ascii_art(&self, options: Option<AsciiOptions>) -> String;
}

/// Implementation for converting an image to ASCII art.
pub struct ImageConverter {
    image: DynamicImage,
}

/// Implementation for converting an image to ASCII art.
impl ImageConverter {
    pub fn new(image: DynamicImage) -> Self {
        Self { image }
    }
}

/// Implementation for converting an image to ASCII art.
impl ToAsciiArt for ImageConverter {
    fn to_ascii_art(&self, options: Option<AsciiOptions>) -> String {
        let options = options.unwrap_or_default();

        let target_width = options.width;
        let target_height = options.height;

        let width_ratio = self.image.width() as f32 / target_width as f32;
        let height_ratio = self.image.height() as f32 / target_height as f32;

        let mut ascii_art = String::with_capacity((target_width * target_height) as usize);

        for y in 0..target_height {
            for x in 0..target_width {
                let start_x = (x as f32 * width_ratio) as u32;
                let start_y = (y as f32 * height_ratio) as u32;

                let mut total_r = 0;
                let mut total_g = 0;
                let mut total_b = 0;

                for dy in 0..height_ratio as u32 {
                    for dx in 0..width_ratio as u32 {
                        let pixel = self.image.get_pixel(start_x + dx, start_y + dy);
                        let channels = pixel.channels();
                        total_r += channels[0] as u32;
                        total_g += channels[1] as u32;
                        total_b += channels[2] as u32;
                    }
                }

                let count = (width_ratio * height_ratio) as u32;
                let avg_r = (total_r / count) as u8;
                let avg_g = (total_g / count) as u8;
                let avg_b = (total_b / count) as u8;

                let luminance = (0.2126 * avg_r as f32 + 0.7152 * avg_g as f32 + 0.0722 * avg_b as f32) as u8;
                let character = match luminance {
                    0..=31 => '#',
                    32..=63 => '@',
                    64..=95 => '8',
                    96..=127 => '&',
                    128..=159 => 'o',
                    160..=191 => ':',
                    192..=223 => '*',
                    224..=255 => '.',
                };

                ascii_art.push(character);
            }
            ascii_art.push('\n');
        }

        ascii_art
    }
}
