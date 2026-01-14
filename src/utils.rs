use anyhow;
use crate::{decoder,encoder,error};

pub fn encode_files(file_path: &str, pic_path: &str, block_size: Option<usize>, error_correction: error::ErrorCorrection) -> anyhow::Result<()> {
    let block_size = block_size.unwrap_or(2048);
    let data = std::fs::read(file_path)?;
    let encoder = encoder::Encoder::new(error_correction);
    for (i, chunk) in data.chunks(block_size).enumerate() {
        let image = encoder.encode(chunk)?;
        let save_path = format!("{}/{}.png", pic_path, i);
        image.save(&save_path)?;
        println!("Saved {}", save_path);
    }
    
    Ok(())
}


pub fn decode_files(pic_path: &str, output_path: &str) -> anyhow::Result<()> {
    let decoder = decoder::Decoder::new(false);
    let mut result = Vec::new();
    let file_count = std::fs::read_dir(pic_path)?.count();
    for name in 0..file_count {
        let full_path = format!("{}/{}.png", pic_path, name);
        let image = image::open(&full_path)?;
        let decoded_data = decoder.decode(&image)?;
        result.extend_from_slice(&decoded_data);
        println!("Decoded data from {}", full_path);
    }
    std::fs::write(output_path, &result)?;
    Ok(())
}
