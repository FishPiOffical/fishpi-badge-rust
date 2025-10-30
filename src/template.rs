use anyhow::Result;
use once_cell::sync::Lazy;
use serde::Serialize;
use tera::{Context, Tera};

static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::new("templates/**/*").unwrap();
    tera.autoescape_on(vec![]);
    tera
});

#[derive(Serialize)]
pub struct BadgeData {
    pub b64: String,
    pub bartxt: String,
    pub barlen: u32,
    pub size: u32,
    pub border: u32,
    pub font_b64: Option<String>,
    pub fontsize: u32,
    pub fontcolor: String,
    pub colors: Vec<String>,
    pub colors_str: String,
    pub shadow: f32,
    pub barradius: u32,
    pub radius: u32,
    pub anime: u32,
    pub direction: String,
    // pub fontway: Option<String>,
}

pub fn render_badge(data: BadgeData) -> Result<String> {
    let mut context = Context::new();
    context.insert("badge", &data);
    let rendered = TEMPLATES.render("badge.svg", &context)?;
    Ok(rendered)
}