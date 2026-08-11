use std::path::PathBuf;

use ratatui::style::Color;
use serde::Deserialize;

use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct ThemeConfig {
    pub background: String,
    pub text: String,
    pub primary: String,
    pub secondary: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub border: String,
    pub muted: String,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
    pub text: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub border: Color,
    pub muted: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Rgb(20, 20, 20),
            text: Color::Rgb(230, 230, 230),

            primary: Color::Rgb(100, 180, 255),
            secondary: Color::Rgb(180, 180, 180),

            success: Color::Rgb(100, 200, 100),
            warning: Color::Rgb(230, 200, 80),
            error: Color::Rgb(230, 90, 90),

            border: Color::Rgb(60, 60, 60),
            muted: Color::Rgb(120, 120, 120),
        }
    }
}

impl TryFrom<ThemeConfig> for Theme {
    type Error = anyhow::Error;

    fn try_from(config: ThemeConfig) -> Result<Self> {
        Ok(Self {
            background: parse_color(&config.background)?,
            text: parse_color(&config.text)?,
            primary: parse_color(&config.primary)?,
            secondary: parse_color(&config.secondary)?,
            success: parse_color(&config.success)?,
            warning: parse_color(&config.warning)?,
            error: parse_color(&config.error)?,
            border: parse_color(&config.border)?,
            muted: parse_color(&config.muted)?,
        })
    }
}

fn parse_color(value: &str) -> Result<Color> {
    let value = value.trim_start_matches('#');

    if value.len() != 6 {
        anyhow::bail!("Invalid color '{}': expected #RRGGBB", value);
    }

    let r = u8::from_str_radix(&value[0..2], 16)?;
    let g = u8::from_str_radix(&value[2..4], 16)?;
    let b = u8::from_str_radix(&value[4..6], 16)?;

    Ok(Color::Rgb(r, g, b))
}

pub fn load_theme(config_dir: &PathBuf, name: Option<&str>) -> Result<Theme> {
    let Some(name) = name else {
        return Ok(Theme::default());
    };

    let path = config_dir.join("themes/").join(format!("{name}.toml"));

    println!("{}", path.display());

    let content =
        std::fs::read_to_string(&path).expect(format!("Failed to read theme '{name}'").as_str());

    let config: ThemeConfig = toml::from_str(&content)?;

    config.try_into()
}
