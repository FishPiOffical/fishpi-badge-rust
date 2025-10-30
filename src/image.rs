use crate::cache;
use anyhow::Result;
use image::{DynamicImage, GenericImageView, codecs::webp::WebPEncoder, ImageFormat};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub async fn process_image(url: &str, limit: u32) -> Result<(String, [u8; 3], bool)> {
    let image_data = cache::get_or_fetch_image(url).await?;

    let format = image::guess_format(&image_data).unwrap_or(ImageFormat::Png);
    if format == ImageFormat::Gif {
        let img = image::load_from_memory(&image_data)?;
        let backcolor = extract_dominant_color(&img);
        let b64 = STANDARD.encode(&image_data);
        Ok((b64, backcolor, true))
    } else {
        let img = image::load_from_memory(&image_data)?;
        let square_img = crop_to_square(img);

        let resized = if square_img.width() > limit {
            square_img.resize_exact(limit, limit, image::imageops::FilterType::Lanczos3)
        } else {
            square_img
        };

        let backcolor = extract_dominant_color(&resized);

        let mut buffer = Vec::new();
        let encoder = WebPEncoder::new_lossless(&mut buffer);
        encoder.encode(
            &resized.to_rgb8(),
            resized.width(),
            resized.height(),
            image::ColorType::Rgb8
        )?;
        let b64 = STANDARD.encode(&buffer);

        Ok((b64, backcolor, false))
    }
}

// 主色提取
fn extract_dominant_color(img: &DynamicImage) -> [u8; 3] {
    let rgb_img = img.to_rgb8();
    let pixels = rgb_img.pixels();
    let mut r_sum = 0u64;
    let mut g_sum = 0u64;
    let mut b_sum = 0u64;
    let count = pixels.len() as u64;
    for pixel in pixels {
        r_sum += pixel[0] as u64;
        g_sum += pixel[1] as u64;
        b_sum += pixel[2] as u64;
    }
    let r_avg = (r_sum / count) as u8;
    let g_avg = (g_sum / count) as u8;
    let b_avg = (b_sum / count) as u8;
    [r_avg, g_avg, b_avg]
}

// 裁剪
fn crop_to_square(img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let size = width.min(height);
    let x = (width - size) / 2;
    let y = (height - size) / 2;
    img.crop_imm(x, y, size, size)
}