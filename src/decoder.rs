use anyhow::{bail, Result};
use image::{DynamicImage, GrayImage, Luma};
use quircs::Quirc;

pub struct Decoder;

impl Decoder {
    pub fn decode(image: &DynamicImage) -> Result<Vec<u8>> {
        let rgb = image.to_rgb8();
        let mut out = Vec::new();

        for channel in 0..3 {
            let gray = extract_channel(&rgb, channel);
            let data = decode_gray(&gray)?;
            out.extend_from_slice(&data);
        }

        Ok(out)
    }
}

fn extract_channel(rgb: &image::RgbImage, channel: usize) -> GrayImage {
    let (w, h) = rgb.dimensions();
    let mut gray = GrayImage::new(w, h);

    for y in 0..h {
        for x in 0..w {
            gray.put_pixel(x, y, Luma([rgb.get_pixel(x, y)[channel]]));
        }
    }

    gray
}

fn decode_gray(gray: &GrayImage) -> Result<Vec<u8>> {
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
