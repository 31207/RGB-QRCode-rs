use anyhow::Result;
use rgb_qrcode_rs::encoder::Encoder;
use rgb_qrcode_rs::error::ErrorCorrection;
use image::ImageFormat;
use rgb_qrcode_rs::utils;
fn main() -> Result<()> {
    let data = "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let encoder = Encoder::new(ErrorCorrection::Low);
    let image = encoder.encode(data.as_bytes())?;

    image.save_with_format("textqr.png", ImageFormat::Png)?;
    println!("Saved textqr.png");

    let image = image::open("textqr.png")?;
    let decoded_data = rgb_qrcode_rs::decoder::Decoder::decode(&image)?;
    println!("Decoded data: {}", String::from_utf8_lossy(&decoded_data));

    // utils::encode_files("res/test.gif", "res/pics", Some(4096))?;
    // utils::decode_files("res/pics", "res/output.gif")?;

    Ok(())
}
