use anyhow::Result;
use image::ImageFormat;
use rgb_qrcode_rs::encoder::Encoder;
use rgb_qrcode_rs::error::ErrorCorrection;
use rgb_qrcode_rs::{decoder::Decoder, utils};
fn main() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        // Test encoding and decoding of text data
        let data = "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let encoder = Encoder::new(ErrorCorrection::Low);
        let image = encoder.encode(data.as_bytes()).unwrap();
        image
            .save_with_format("textqr.png", ImageFormat::Png)
            .unwrap();
        println!("Saved textqr.png");

        let image = image::open("textqr.png").unwrap();
        let decoder = Decoder::new(false);
        let decoded_data = decoder.decode(&image).unwrap();
        println!("Decoded data: {}", String::from_utf8_lossy(&decoded_data));
        assert!(data.as_bytes() == decoded_data.as_slice());
    }

    #[test]
    fn test_encode_decode_binary() {
        // Generate binary data, and test encoding and decoding
        let data: Vec<u8> = (0..=255).collect();
        let encoder = Encoder::new(ErrorCorrection::Low);
        let image = encoder.encode(&data).unwrap();
        image
            .save_with_format("binaryqr.png", ImageFormat::Png)
            .unwrap();
        println!("Saved binaryqr.png");

        let image = image::open("binaryqr.png").unwrap();
        let decoder = Decoder::new(false);
        let decoded_data = decoder.decode(&image).unwrap();
        println!("Decoded data length: {}", decoded_data.len());
        assert!(data == decoded_data);
    }

    #[test]
    fn test_utils_encode_decode_files() -> Result<()> {
        // Test the utility functions for encoding and decoding files
        utils::encode_files(
            "res/test.png",
            "res/pics",
            Some(256),
            ErrorCorrection::High,
        )?;
        utils::decode_files("res/pics", "res/output.png")?;
        let original_hash = sha256_file("res/test.png")?;
        let output_hash = sha256_file("res/output.png")?;
        println!("Original SHA256: {}", original_hash);
        println!("Output SHA256:   {}", output_hash);
        assert_eq!(original_hash, output_hash);
        Ok(())
    }


    use sha2::{Digest, Sha256};
    use std::{
        fs::File,
        io::{BufReader, Read},
    };
    fn sha256_file(path: &str) -> Result<String> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];

        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }
}
