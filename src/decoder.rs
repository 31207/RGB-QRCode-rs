use anyhow::{Result, bail};
use image::{DynamicImage, GrayImage, Luma};
use quircs::Quirc;

#[derive(Clone, Copy)]
pub struct Decoder {
    debug: bool,
}

impl Decoder {
    pub fn new(debug: bool) -> Self {
        Self { debug }
    }

    pub fn decode(self, image: &DynamicImage) -> Result<Vec<u8>> {
        let rgb = image.to_rgb8();
        let mut out = Vec::new();

        for channel in 0..3 {
            let gray = self.extract_channel(&rgb, channel);

            if self.debug {
                let (w, h) = gray.dimensions();
                let gray_image = GrayImage::from_raw(w, h, gray.to_owned().as_raw().to_vec())
                    .expect("failed to create debug gray image");
                gray_image
                    .save(format!("debug_gray_{}.png", channel))
                    .expect("failed to save debug gray image");
                println!("Saved debug_gray_{}.png", channel);
            }

            let data = self.decode_gray(&gray)?;
            out.extend_from_slice(&data);
        }

        Ok(out)
    }
    fn extract_channel(self, rgb: &image::RgbImage, channel: usize) -> GrayImage {
        let (w, h) = rgb.dimensions();
        let mut gray = GrayImage::new(w, h);

        for y in 0..h {
            for x in 0..w {
                gray.put_pixel(x, y, Luma([rgb.get_pixel(x, y)[channel]]));
            }
        }

        gray
    }

    fn decode_gray(self, gray: &GrayImage) -> Result<Vec<u8>> {
        let (w, h) = gray.dimensions();
        let mut quirc = Quirc::new();

        let codes = quirc.identify(w as usize, h as usize, gray.as_raw());

        for code in codes {
            if let Ok(code) = code {
                let decoded = code.decode();
                if let Ok(data) = decoded {
                    return Ok(data.payload);
                }
            }
        }
        bail!("no QR code found")
    }
}
