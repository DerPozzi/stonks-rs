use std::path::PathBuf;

use anyhow::Result;
use config::{Config, File};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use stonks_rs::{
    service::{helpers::get_all_transactions, service::init_db},
    types::{Connection, Transaction},
};
use strum::EnumIter;

#[derive(Debug, Serialize, Deserialize, Default)]
struct Settings {
    app: AppConfig,
}
#[derive(Debug, Serialize, Deserialize, Default)]
struct AppConfig {
    theme: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ThemeConfig {
    pub main: String,
    pub accent: String,
    pub text: String,
    pub muted: String,
    pub error: String,
    pub success: String,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub main: Color,
    pub accent: Color,
    pub text: Color,
    pub muted: Color,
    pub error: Color,
    pub success: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            // background: Color::Rgb(30, 30, 30),
            // foreground: Color::Rgb(220, 220, 220),
            main: Color::Rgb(97, 175, 239),
            accent: Color::Rgb(198, 120, 221),

            success: Color::Rgb(152, 195, 121),
            // warning: Color::Rgb(229, 192, 123),
            error: Color::Rgb(224, 108, 117),

            // border: Color::Rgb(70, 70, 70),
            text: Color::Rgb(235, 235, 235),
            muted: Color::Rgb(128, 128, 128),
        }
    }
}

impl TryFrom<ThemeConfig> for Theme {
    type Error = anyhow::Error;

    fn try_from(config: ThemeConfig) -> Result<Self> {
        Ok(Self {
            main: parse_color(&config.main)?,
            accent: parse_color(&config.accent)?,
            text: parse_color(&config.text)?,
            muted: parse_color(&config.muted)?,
            error: parse_color(&config.error)?,
            success: parse_color(&config.success)?,
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

    let path = config_dir.join("theme").join(format!("{name}.toml"));

    let content =
        std::fs::read_to_string(&path).expect(format!("Failed to read theme '{name}'").as_str());

    let config: ThemeConfig = toml::from_str(&content)?;

    config.try_into()
}

fn load_config(path: PathBuf) -> Result<Settings> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if !path.exists() {
        let defaults = Settings::default();

        println!("{:?}", defaults);

        let content =
            toml::to_string_pretty(&defaults).expect("Failed to serialize default config");

        println!("{}", content);

        std::fs::write(&path, content)
            .expect(format!("Failed to write config to {}", path.display()).as_str());
    }

    let config = Config::builder().add_source(File::from(path)).build()?;

    let settings = config.try_deserialize()?;
    Ok(settings)
}

fn init() -> Result<(Connection, Vec<Transaction>, Settings, Theme)> {
    let home_path = dirs::home_dir().expect("Could not find home directory of current user.");
    let config_path = home_path.join(".config").join("stonks-rs");

    let settings = load_config(config_path.join("config.toml"))?;
    let conn = init_db(home_path)?;

    let theme = load_theme(&config_path.join("/themes"), settings.app.theme.as_deref())?;

    let transactions = get_all_transactions(&conn)?;

    Ok((conn, transactions, settings, theme))
}

#[derive(Debug, PartialEq, EnumIter)]
pub enum Page {
    Dashboard,
    Overview,
    Transactions,
    Dividends,
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Page::Dashboard => write!(f, "Dashboard"),
            Page::Overview => write!(f, "Overview"),
            Page::Transactions => write!(f, "Transactions"),
            Page::Dividends => write!(f, "Dividends"),
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Page::Dashboard
    }
}

#[derive(Debug)]
pub enum Action {
    None,
    Add,
    Edit(u64),
    Delete(u64),
}

impl Default for Action {
    fn default() -> Self {
        Action::None
    }
}

#[derive(Debug)]
pub struct App {
    pub transactions: Vec<Transaction>,
    pub db_connection: Connection,
    pub settings: Settings,
    pub should_quit: bool,
    pub current_page: Page,
    pub current_action: Action,
    pub theme: Theme,
}

impl Default for App {
    fn default() -> Self {
        let (db_connection, transactions, settings, theme) = init().expect("Failed to init app");

        Self {
            transactions,
            db_connection,
            settings,
            should_quit: false,
            current_page: Page::default(),
            current_action: Action::default(),
            theme,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next_page(&mut self) {
        self.current_action = Action::None;
        self.current_page = match self.current_page {
            Page::Dashboard => Page::Overview,
            Page::Overview => Page::Transactions,
            Page::Transactions => Page::Dividends,
            Page::Dividends => Page::Dashboard,
        }
    }
    pub fn previous_page(&mut self) {
        self.current_action = Action::None;
        self.current_page = match self.current_page {
            Page::Dashboard => Page::Dividends,
            Page::Overview => Page::Dashboard,
            Page::Transactions => Page::Overview,
            Page::Dividends => Page::Transactions,
        }
    }
}
