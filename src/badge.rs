use crate::{image, template, BadgeParams};
use anyhow::Result;
use palette::{FromColor, Hsl, Srgb};

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


pub async fn generate_badge(params: BadgeParams) -> Result<String> {
    let scale = params.scale.unwrap_or(1.0);
    let mut size = params.size.unwrap_or(32);
    let mut border = params.border.unwrap_or(3);
    let mut fontsize = params.fontsize.unwrap_or(15);
    let mut barradius = params.barradius.unwrap_or(5);
    let anime = params.anime.unwrap_or(0.5);
    let shadow = params.shadow.unwrap_or(0.5);
    let txt = params.txt.clone().unwrap_or_else(|| "Operater".to_string());

    // barlen 处理
    let mut barlen = if let Some(ref bl) = params.barlen {
        if bl == "auto" {
            let mut l = txt.chars().count() as f32;
            let wide_count = txt.chars().filter(|c| (*c as u32) > 127).count() as f32;
            l += wide_count * 0.84;
            (fontsize as f32 * l * 0.55 + 2.6 * border as f32) as u32
        } else {
            bl.parse().unwrap_or(0)
        }
    } else {
        let mut l = txt.chars().count() as f32;
        let wide_count = txt.chars().filter(|c| (*c as u32) > 127).count() as f32;
        l += wide_count * 0.84;
        (fontsize as f32 * l * 0.55 + 2.6 * border as f32) as u32
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
    let mut backcolor_arr = b_color;
    if let Some(ref s) = params.backcolor {
        if s == "auto" {
            backcolor_arr = b_color;
        } else if s.contains(',') {
            colors = s.split(',').map(|x| x.trim().to_string()).collect();
            backcolor_arr = hex_to_rgb(&colors[1]);
        } else {
            backcolor_arr = hex_to_rgb(s);
        }
    }

    // 渐变色
    let (color1_str, color2_str) = if colors.len() == 2 {
        let c1 = hex_to_rgb(&colors[0]);
        let c2 = hex_to_rgb(&colors[1]);
        (format!("rgb({},{},{})", c1[0], c1[1], c1[2]), format!("rgb({},{},{})", c2[0], c2[1], c2[2]))
    } else {
        let srgb = Srgb::new(
            backcolor_arr[0] as f32 / 255.0,
            backcolor_arr[1] as f32 / 255.0,
            backcolor_arr[2] as f32 / 255.0,
        );
        let hsl: Hsl = Hsl::from_color(srgb);
        println!("h: {:?}, l: {}, s: {}", hsl.hue, hsl.lightness, hsl.saturation);
        let h_deg = hsl.hue.into_degrees();
        let l = hsl.lightness;
        let s = hsl.saturation;
        let c1 = hls_to_rgb(h_deg, (l + 0.02).min(1.0), s);
        let c2 = hls_to_rgb(h_deg, (l - 0.06).max(0.0), s);
        (format!("rgb({},{},{})", c1.0, c1.1, c1.2), format!("rgb({},{},{})", c2.0, c2.1, c2.2))
    };
    
    // fontcolor
    let fontcolor_str = {
        let fc = if let Some(ref s) = params.fontcolor {
            if s == "auto" {
                let mean = (backcolor_arr[0] as f32 + backcolor_arr[1] as f32 + backcolor_arr[2] as f32) / 3.0;
                if mean > 214.0 { [33, 33, 33] } else { [255, 255, 255] }
            } else {
                hex_to_rgb(s)
            }
        } else {
            let mean = (backcolor_arr[0] as f32 + backcolor_arr[1] as f32 + backcolor_arr[2] as f32) / 3.0;
            if mean > 214.0 { [33, 33, 33] } else { [255, 255, 255] }
        };
        format!("rgb({},{},{})", fc[0], fc[1], fc[2])
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

    let mime = if is_gif { "image/gif" } else { "image/webp" };
    let b64 = format!("data:{};base64,{}", mime, image_data);
    
    let badge_data = template::BadgeData {
        b64,
        bartxt: txt.clone(),
        barlen,
        size,
        border,
        fontsize,
        fontcolor: fontcolor_str,
        color1: color1_str,
        color2: color2_str,
        shadow,
        barradius,
        radius: 99999,
        anime: (anime * 1000.0) as u32,
        direction,
    };

    let svg = template::render_badge(badge_data)?;
    Ok(svg)
}