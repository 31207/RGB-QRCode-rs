use anyhow::{Context, Result};
use image::{GrayImage, Luma, RgbImage};
use qrcode::{QrCode, Version};

use crate::error::ErrorCorrection;

pub struct Encoder {
    error_correction: ErrorCorrection,
}

impl Encoder {
    pub fn new(error_correction: ErrorCorrection) -> Self {
        Self { error_correction }
    }

    pub fn encode(&self, data: &[u8]) -> Result<RgbImage> {
        // === 1. 拆分数据 ===
        let section_len = (data.len() + 2) / 3;

        let parts = [
            &data[0..section_len.min(data.len())],
            &data[section_len.min(data.len())..(section_len * 2).min(data.len())],
            &data[(section_len * 2).min(data.len())..],
        ];
        // === 2. 生成 QR（固定 version） ===
        let mut gray_images: Vec<GrayImage> = Vec::with_capacity(3);
        let mut codes: Vec<QrCode> = Vec::new();
        for part in parts {
            codes.push(
                QrCode::new(part).context("failed to create initial QR for version calculation")?,
            );
        }
        let versions: Vec<Version> = codes.iter().map(|c| c.version()).collect();
        if is_all_same_version(&versions) {
            for code in codes {
                let image = code.render::<Luma<u8>>().quiet_zone(true).build();
                gray_images.push(image);
            }
        } else {
            let max_version = get_max_version(versions);
            for part in parts {
                let code =
                    QrCode::with_version(part, max_version, self.error_correction.to_ec_level())
                        .context("failed to create QR with fixed version")?;
                let image = code.render::<Luma<u8>>().quiet_zone(true).build();
                gray_images.push(image);
            }
        }

        // === 3. 合并为 RGB ===
        let (w, h) = gray_images[0].dimensions();
        let r = gray_images[0].as_raw();
        let g = gray_images[1].as_raw();
        let b = gray_images[2].as_raw();

        let mut rgb = RgbImage::new(w, h);

        for (i, px) in rgb.as_mut().chunks_exact_mut(3).enumerate() {
            px[0] = r[i];
            px[1] = g[i];
            px[2] = b[i];
        }

        Ok(rgb)
    }
}

fn get_max_version(versions: Vec<Version>) -> Version {
    versions
        .into_iter()
        .max_by_key(|v| match v {
            Version::Normal(n) => *n,
            Version::Micro(n) => *n,
        })
        .unwrap_or(Version::Normal(40))
}

fn is_all_same_version(versions: &Vec<Version>) -> bool {
    if versions.is_empty() {
        return true;
    }
    let first = &versions[0];
    versions.iter().all(|v| v == first)
}
