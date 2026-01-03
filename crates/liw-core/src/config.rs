//! Configuration module for Life in Weeks
//!
//! Handles loading and saving user configuration from TOML files.

use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Visual theme for the wallpaper
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    /// Black dots on cream/white background
    MinimalInk,
    /// Green on black, terminal aesthetic
    TerminalGreen,
    /// Muted colors on dark gray
    #[default]
    SoftDark,
    /// Past weeks fade from warm to cool
    SunsetGradient,
    /// Custom colors defined by user
    Custom {
        background: String,
        past_week: String,
        current_week: String,
        future_week: String,
        accent: String,
    },
}

impl Theme {
    /// Get the color palette for this theme
    pub fn colors(&self) -> ThemeColors {
        match self {
            Theme::MinimalInk => ThemeColors {
                background: [250, 245, 235, 255],
                past_week: [30, 30, 30, 255],
                current_week: [220, 60, 60, 255],
                future_week: [200, 195, 185, 255],
                accent: [220, 60, 60, 255],
                text: [30, 30, 30, 255],
            },
            Theme::TerminalGreen => ThemeColors {
                background: [15, 15, 15, 255],
                past_week: [0, 180, 80, 255],
                current_week: [0, 255, 120, 255],
                future_week: [40, 60, 45, 255],
                accent: [0, 255, 120, 255],
                text: [0, 200, 100, 255],
            },
            Theme::SoftDark => ThemeColors {
                background: [28, 28, 32, 255],
                past_week: [140, 140, 160, 255],
                current_week: [255, 120, 100, 255],
                future_week: [55, 55, 65, 255],
                accent: [255, 120, 100, 255],
                text: [200, 200, 210, 255],
            },
            Theme::SunsetGradient => ThemeColors {
                background: [25, 25, 35, 255],
                past_week: [255, 140, 90, 255],
                current_week: [255, 220, 100, 255],
                future_week: [60, 60, 90, 255],
                accent: [255, 180, 100, 255],
                text: [240, 240, 250, 255],
            },
            Theme::Custom {
                background,
                past_week,
                current_week,
                future_week,
                accent,
            } => ThemeColors {
                background: parse_hex_color(background),
                past_week: parse_hex_color(past_week),
                current_week: parse_hex_color(current_week),
                future_week: parse_hex_color(future_week),
                accent: parse_hex_color(accent),
                text: [255, 255, 255, 255],
            },
        }
    }
}

/// Parsed color values for a theme
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub background: [u8; 4],
    pub past_week: [u8; 4],
    pub current_week: [u8; 4],
    pub future_week: [u8; 4],
    pub accent: [u8; 4],
    pub text: [u8; 4],
}

/// Parse a hex color string like "#FF5500" into RGBA
fn parse_hex_color(hex: &str) -> [u8; 4] {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(128);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(128);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(128);
        let a = if hex.len() >= 8 {
            u8::from_str_radix(&hex[6..8], 16).unwrap_or(255)
        } else {
            255
        };
        [r, g, b, a]
    } else {
        [128, 128, 128, 255]
    }
}

/// User configuration for Life in Weeks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Date of birth for life mode
    pub dob: Option<NaiveDate>,
    /// Expected lifespan in years
    #[serde(default = "default_lifespan")]
    pub lifespan_years: u8,
    /// Visual theme
    #[serde(default)]
    pub theme: Theme,
    /// Screen width for wallpaper generation
    #[serde(default = "default_width")]
    pub screen_width: u32,
    /// Screen height for wallpaper generation
    #[serde(default = "default_height")]
    pub screen_height: u32,
    /// Default mode to use
    #[serde(default)]
    pub default_mode: String,
    /// Number of months for next-months mode
    #[serde(default = "default_months")]
    pub next_months: u8,
}

fn default_lifespan() -> u8 {
    80
}
fn default_width() -> u32 {
    1920
}
fn default_height() -> u32 {
    1080
}
fn default_months() -> u8 {
    6
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dob: None,
            lifespan_years: default_lifespan(),
            theme: Theme::default(),
            screen_width: default_width(),
            screen_height: default_height(),
            default_mode: "year-end".to_string(),
            next_months: default_months(),
        }
    }
}

impl Config {
    /// Get the default config file path
    pub fn default_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not determine config directory")?
            .join("life-in-weeks");
        Ok(config_dir.join("config.toml"))
    }

    /// Get the default output path for generated wallpapers
    pub fn default_output_path() -> Result<PathBuf> {
        let data_dir = dirs::data_local_dir()
            .context("Could not determine data directory")?
            .join("life-in-weeks");
        Ok(data_dir.join("wallpaper.png"))
    }

    /// Load configuration from the default path
    pub fn load() -> Result<Self> {
        let path = Self::default_path()?;
        Self::load_from(&path)
    }

    /// Load configuration from a specific path
    pub fn load_from(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            let contents = fs::read_to_string(path)
                .with_context(|| format!("Failed to read config from {:?}", path))?;
            let config: Config = toml::from_str(&contents)
                .with_context(|| format!("Failed to parse config from {:?}", path))?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Save configuration to the default path
    pub fn save(&self) -> Result<()> {
        let path = Self::default_path()?;
        self.save_to(&path)
    }

    /// Save configuration to a specific path
    pub fn save_to(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory {:?}", parent))?;
        }
        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(path, contents)
            .with_context(|| format!("Failed to write config to {:?}", path))?;
        Ok(())
    }

    /// Update a single config value by key
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "dob" => {
                let date = NaiveDate::parse_from_str(value, "%Y-%m-%d")
                    .with_context(|| format!("Invalid date format: {}. Use YYYY-MM-DD", value))?;
                self.dob = Some(date);
            }
            "lifespan" | "lifespan_years" => {
                self.lifespan_years = value.parse()
                    .with_context(|| format!("Invalid lifespan: {}", value))?;
            }
            "theme" => {
                self.theme = match value.to_lowercase().as_str() {
                    "minimal" | "minimal_ink" | "minimal-ink" => Theme::MinimalInk,
                    "terminal" | "terminal_green" | "terminal-green" => Theme::TerminalGreen,
                    "dark" | "soft_dark" | "soft-dark" => Theme::SoftDark,
                    "sunset" | "sunset_gradient" | "sunset-gradient" => Theme::SunsetGradient,
                    _ => anyhow::bail!("Unknown theme: {}. Options: minimal, terminal, dark, sunset", value),
                };
            }
            "width" | "screen_width" => {
                self.screen_width = value.parse()
                    .with_context(|| format!("Invalid width: {}", value))?;
            }
            "height" | "screen_height" => {
                self.screen_height = value.parse()
                    .with_context(|| format!("Invalid height: {}", value))?;
            }
            "default_mode" | "mode" => {
                self.default_mode = value.to_string();
            }
            "next_months" | "months" => {
                self.next_months = value.parse()
                    .with_context(|| format!("Invalid months: {}", value))?;
            }
            _ => anyhow::bail!("Unknown config key: {}", key),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color() {
        assert_eq!(parse_hex_color("#FF0000"), [255, 0, 0, 255]);
        assert_eq!(parse_hex_color("00FF00"), [0, 255, 0, 255]);
        assert_eq!(parse_hex_color("#0000FFAA"), [0, 0, 255, 170]);
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.lifespan_years, 80);
        assert_eq!(config.theme, Theme::SoftDark);
    }
}
