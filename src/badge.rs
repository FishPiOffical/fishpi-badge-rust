use crate::{image, template, cache, BadgeParams};
use anyhow::Result;
use palette::{FromColor, Hsl, Srgb};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

fn hex_to_rgb(s: &str) -> [u8; 3] {
    let s = s.trim_start_matches('#');
    if s.len() != 6 {
        return [255, 255, 255];
    }
    let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(255);
    [r, g, b]
}

fn hls_to_rgb(h: f32, l: f32, s: f32) -> (u8, u8, u8) {
    let l = l.clamp(0.0, 1.0);
    if s == 0.0 {
        let v = (l * 255.0).round() as u8;
        (v, v, v)
    } else {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let h = h % 360.0;
        let x = c * (1.0 - ((h / 60.0 % 2.0) - 1.0).abs());
        let m = l - c / 2.0;
        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        let r = ((r + m).clamp(0.0, 1.0) * 255.0).round() as u8;
        let g = ((g + m).clamp(0.0, 1.0) * 255.0).round() as u8;
        let b = ((b + m).clamp(0.0, 1.0) * 255.0).round() as u8;
        (r, g, b)
    }
}

fn process_fontcolor(params: &BadgeParams, backcolor_arr: [u8; 3]) -> String {
    let mut fontcolors: Vec<String> = vec![];
    if let Some(ref s) = params.fontcolor {
        if s == "auto" {
            // 基于背景亮度选择黑或白（纯色）
            let mean = (backcolor_arr[0] as f32 + backcolor_arr[1] as f32 + backcolor_arr[2] as f32) / 3.0;
            let fc = if mean > 214.0 { [33, 33, 33] } else { [255, 255, 255] };
            fontcolors.push(format!("rgb({},{},{})", fc[0], fc[1], fc[2]));
        } else if s.contains(',') {
            // 彩色渐变
            fontcolors = s.split(',').map(|x| {
                let rgb = hex_to_rgb(x.trim());
                format!("rgb({},{},{})", rgb[0], rgb[1], rgb[2])
            }).collect();
        } else {
            // 单色
            let rgb = hex_to_rgb(s);
            let color_str = format!("rgb({},{},{})", rgb[0], rgb[1], rgb[2]);
            fontcolors.push(color_str.clone());
            fontcolors.push(color_str);
        }
    } else {
        // 基于背景亮度选择黑或白（纯色）
        let mean = (backcolor_arr[0] as f32 + backcolor_arr[1] as f32 + backcolor_arr[2] as f32) / 3.0;
        let fc = if mean > 214.0 { [33, 33, 33] } else { [255, 255, 255] };
        let color_str = format!("rgb({},{},{})", fc[0], fc[1], fc[2]);
        fontcolors.push(color_str.clone());
        fontcolors.push(color_str);
    }
    println!("Font colors: {:?}", fontcolors);
    fontcolors.join(", ")
}


pub async fn generate_badge(params: BadgeParams) -> Result<String> {
    let scale = params.scale.unwrap_or(1.0);
    let mut size = params.size.unwrap_or(32);
    let mut border = params.border.unwrap_or(3);
    let mut fontsize = params.fontsize.unwrap_or(15);
    let mut barradius = params.barradius.unwrap_or(size / 2);
    let anime = params.anime.unwrap_or(0.5);
    let shadow = params.shadow.unwrap_or(0.5);
    let txt = params.txt.clone().unwrap_or_default();

    let calculate_barlen = || -> u32 {
        if txt.is_empty() {
            return 0;
        }
        let mut l = txt.chars().count() as f32;
        let wide_count = txt.chars().filter(|c| (*c as u32) > 127).count() as f32;
        l += wide_count * 0.84;
        (fontsize as f32 * l * 0.55 + 2.6 * border as f32) as u32
    };

    // barlen 处理
    let mut barlen = match params.barlen.as_deref() {
        Some("auto") => calculate_barlen(),
        Some(bl) => bl.parse().unwrap_or(0),
        None => calculate_barlen(),
    };

    // 参数缩放
    size = (size as f32 * scale) as u32;
    border = (border as f32 * scale) as u32;
    barlen = (barlen as f32 * scale) as u32;
    fontsize = (fontsize as f32 * scale) as u32;
    barradius = (barradius as f32 * scale) as u32;


    // 图片和主色
    let (image_data, b_color, is_gif) = if let Some(url) = params.url.clone() {
        image::process_image(&url, (size - 2 * border) * 4).await?
    } else {
        (String::new(), [0, 123, 255], false)
    };

    // backcolor 处理
    let mut colors: Vec<String> = vec![];
    let backcolor_arr;
    if let Some(ref s) = params.backcolor {
        if s == "auto" {
            backcolor_arr = b_color;
            // 默认渐变
            let srgb = Srgb::new(
                backcolor_arr[0] as f32 / 255.0,
                backcolor_arr[1] as f32 / 255.0,
                backcolor_arr[2] as f32 / 255.0,
            );
            let hsl: Hsl = Hsl::from_color(srgb);
            let h_deg = hsl.hue.into_degrees();
            let l = hsl.lightness;
            let s_val = hsl.saturation;
            let c1 = hls_to_rgb(h_deg, (l + 0.02).min(1.0), s_val);
            let c2 = hls_to_rgb(h_deg, (l - 0.06).max(0.0), s_val);
            colors.push(format!("rgb({},{},{})", c1.0, c1.1, c1.2));
            colors.push(format!("rgb({},{},{})", c2.0, c2.1, c2.2));
        } else if s.contains(',') {
            colors = s.split(',').map(|x| {
                let rgb = hex_to_rgb(x.trim());
                format!("rgb({},{},{})", rgb[0], rgb[1], rgb[2])
            }).collect();
            backcolor_arr = hex_to_rgb(&colors[0]);
        } else {
            // 单一颜色
            let rgb = hex_to_rgb(s);
            colors.push(format!("rgb({},{},{})", rgb[0], rgb[1], rgb[2]));
            backcolor_arr = rgb;
        }
    } else {
        // 默认渐变
        backcolor_arr = b_color;
        let srgb = Srgb::new(
            backcolor_arr[0] as f32 / 255.0,
            backcolor_arr[1] as f32 / 255.0,
            backcolor_arr[2] as f32 / 255.0,
        );
        let hsl: Hsl = Hsl::from_color(srgb);
        let h_deg = hsl.hue.into_degrees();
        let l = hsl.lightness;
        let s_val = hsl.saturation;
        let c1 = hls_to_rgb(h_deg, (l + 0.02).min(1.0), s_val);
        let c2 = hls_to_rgb(h_deg, (l - 0.06).max(0.0), s_val);
        colors.push(format!("rgb({},{},{})", c1.0, c1.1, c1.2));
        colors.push(format!("rgb({},{},{})", c2.0, c2.1, c2.2));
    }
    
    let colors_str = colors.join(", ");

    // fontcolor
    let fontcolor_str = process_fontcolor(&params, backcolor_arr);

    // 定义字体
    let font_b64 = if let Some(ref font_url) = params.font {
        if font_url.starts_with("http") {
            // 下载字体并 base64 编码
            let font_data = cache::get_or_fetch(font_url).await?;
            Some(format!("data:font/ttf;base64,{}", STANDARD.encode(&font_data)))
        } else {
            // 本地字体文件路径，读取并 base64 编码（仅用于测试）
            let font_data = tokio::fs::read(font_url).await?;
            Some(format!("data:font/ttf;base64,{}", STANDARD.encode(&font_data)))
        }
    } else {
        None
    };

    let direction = match params.way.as_deref() {
        Some("right") => "to right".to_string(),
        Some("left") => "to left".to_string(),
        Some("top") => "to top".to_string(),
        Some("bottom") => "to bottom".to_string(),
        Some("top-right") => "to top right".to_string(),
        Some("top-left") => "to top left".to_string(),
        Some("bottom-right") => "to bottom right".to_string(),
        Some("bottom-left") => "to bottom left".to_string(),
        Some(custom) => custom.to_string(),
        None => "to bottom".to_string(),
    };

    let fontway = match params.fontway.as_deref() {
        Some("right") => "to right".to_string(),
        Some("left") => "to left".to_string(),
        Some("top") => "to top".to_string(),
        Some("bottom") => "to bottom".to_string(),
        Some("top-right") => "to top right".to_string(),
        Some("top-left") => "to top left".to_string(),
        Some("bottom-right") => "to bottom right".to_string(),
        Some("bottom-left") => "to bottom left".to_string(),
        Some(custom) => custom.to_string(),
        None => "to bottom".to_string(),
    };

    let mime = if is_gif { "image/gif" } else { "image/webp" };
    let b64 = format!("data:{};base64,{}", mime, image_data);
    
    let badge_data = template::BadgeData {
        b64,
        bartxt: txt.clone(),
        barlen,
        size,
        border,
        font_b64,
        fontsize,
        fontcolor: fontcolor_str,
        colors,
        colors_str,
        shadow,
        barradius,
        radius: 99999,
        anime: (anime * 1000.0) as u32,
        direction,
        fontway: Some(fontway),
    };

    let svg = template::render_badge(badge_data)?;
    Ok(svg)
}