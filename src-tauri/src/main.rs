//! Life in Weeks Tauri Backend
//!
//! Exposes liw-core functionality to the web frontend via Tauri commands.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::NaiveDate;
use liw_core::{
    Config, Mode, Theme, WeekGrid,
    render_grid, set_wallpaper as core_set_wallpaper,
    install_schedule, uninstall_schedule,
    renderer::save_grid,
    scheduler::is_schedule_installed,
};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

/// Request payload for generating a preview
#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    mode: String,
    dob: Option<String>,
    lifespan: Option<u8>,
    months: Option<u8>,
    theme: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

/// Response with grid info and preview image
#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    title: String,
    subtitle: String,
    total_weeks: usize,
    elapsed_weeks: usize,
    remaining_weeks: usize,
    columns: usize,
    rows: usize,
    /// Base64-encoded PNG image
    image_base64: String,
}

/// Current configuration state
#[derive(Debug, Serialize)]
pub struct ConfigState {
    dob: Option<String>,
    lifespan_years: u8,
    theme: String,
    screen_width: u32,
    screen_height: u32,
    default_mode: String,
    next_months: u8,
    schedule_installed: bool,
}

/// Parse theme from string
fn parse_theme(s: &str) -> Theme {
    match s.to_lowercase().as_str() {
        "minimal" | "minimal_ink" | "minimal-ink" => Theme::MinimalInk,
        "terminal" | "terminal_green" | "terminal-green" => Theme::TerminalGreen,
        "dark" | "soft_dark" | "soft-dark" => Theme::SoftDark,
        "sunset" | "sunset_gradient" | "sunset-gradient" => Theme::SunsetGradient,
        _ => Theme::SoftDark,
    }
}

/// Theme to string name
fn theme_name(theme: &Theme) -> String {
    match theme {
        Theme::MinimalInk => "minimal".to_string(),
        Theme::TerminalGreen => "terminal".to_string(),
        Theme::SoftDark => "dark".to_string(),
        Theme::SunsetGradient => "sunset".to_string(),
        Theme::Custom { .. } => "custom".to_string(),
    }
}

/// Generate a preview image and return as base64
#[tauri::command]
fn generate_preview(request: GenerateRequest) -> Result<GenerateResponse, String> {
    // Load config for defaults
    let config = Config::load().unwrap_or_default();

    // Parse DOB if provided
    let dob = if let Some(ref dob_str) = request.dob {
        NaiveDate::parse_from_str(dob_str, "%Y-%m-%d").ok()
    } else {
        config.dob
    };

    // Parse mode
    let mode = Mode::from_str_with_params(
        &request.mode,
        dob,
        Some(request.lifespan.unwrap_or(config.lifespan_years)),
        Some(request.months.unwrap_or(config.next_months)),
    )
    .map_err(|e| e.to_string())?;

    // Get theme
    let theme = request
        .theme
        .as_ref()
        .map(|t| parse_theme(t))
        .unwrap_or(config.theme);

    let width = request.width.unwrap_or(config.screen_width);
    let height = request.height.unwrap_or(config.screen_height);

    // Calculate grid and render
    let grid = WeekGrid::calculate(&mode);
    let image = render_grid(&grid, &theme, width, height);

    // Encode as PNG to base64
    let mut buffer = Cursor::new(Vec::new());
    image
        .write_to(&mut buffer, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to encode image: {}", e))?;
    
    let image_base64 = BASE64.encode(buffer.get_ref());

    let remaining = grid.total_weeks.saturating_sub(grid.elapsed_weeks + 1);

    Ok(GenerateResponse {
        title: grid.title,
        subtitle: grid.subtitle,
        total_weeks: grid.total_weeks,
        elapsed_weeks: grid.elapsed_weeks,
        remaining_weeks: remaining,
        columns: grid.columns,
        rows: grid.rows,
        image_base64,
    })
}

/// Generate and set as wallpaper
#[tauri::command]
fn set_wallpaper_cmd(request: GenerateRequest) -> Result<String, String> {
    // Load config for defaults
    let config = Config::load().unwrap_or_default();

    // Parse DOB if provided
    let dob = if let Some(ref dob_str) = request.dob {
        NaiveDate::parse_from_str(dob_str, "%Y-%m-%d").ok()
    } else {
        config.dob
    };

    // Parse mode
    let mode = Mode::from_str_with_params(
        &request.mode,
        dob,
        Some(request.lifespan.unwrap_or(config.lifespan_years)),
        Some(request.months.unwrap_or(config.next_months)),
    )
    .map_err(|e| e.to_string())?;

    // Get theme
    let theme = request
        .theme
        .as_ref()
        .map(|t| parse_theme(t))
        .unwrap_or(config.theme);

    let width = request.width.unwrap_or(config.screen_width);
    let height = request.height.unwrap_or(config.screen_height);

    // Calculate grid and render
    let grid = WeekGrid::calculate(&mode);
    let image = render_grid(&grid, &theme, width, height);

    // Save to output path
    let output_path = Config::default_output_path()
        .map_err(|e| format!("Failed to get output path: {}", e))?;
    
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    save_grid(&image, &output_path)
        .map_err(|e| format!("Failed to save wallpaper: {}", e))?;

    // Set as wallpaper
    core_set_wallpaper(&output_path)
        .map_err(|e| format!("Failed to set wallpaper: {}", e))?;

    Ok(format!("Wallpaper set successfully: {:?}", output_path))
}

/// Get current configuration
#[tauri::command]
fn get_config() -> Result<ConfigState, String> {
    let config = Config::load().unwrap_or_default();
    
    Ok(ConfigState {
        dob: config.dob.map(|d| d.format("%Y-%m-%d").to_string()),
        lifespan_years: config.lifespan_years,
        theme: theme_name(&config.theme),
        screen_width: config.screen_width,
        screen_height: config.screen_height,
        default_mode: config.default_mode,
        next_months: config.next_months,
        schedule_installed: is_schedule_installed(),
    })
}

/// Save configuration
#[tauri::command]
fn save_config(
    dob: Option<String>,
    lifespan: Option<u8>,
    theme: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    default_mode: Option<String>,
    months: Option<u8>,
) -> Result<String, String> {
    let mut config = Config::load().unwrap_or_default();

    if let Some(ref dob_str) = dob {
        if !dob_str.is_empty() {
            config.dob = NaiveDate::parse_from_str(dob_str, "%Y-%m-%d").ok();
        } else {
            config.dob = None;
        }
    }

    if let Some(l) = lifespan {
        config.lifespan_years = l;
    }

    if let Some(ref t) = theme {
        config.theme = parse_theme(t);
    }

    if let Some(w) = width {
        config.screen_width = w;
    }

    if let Some(h) = height {
        config.screen_height = h;
    }

    if let Some(ref m) = default_mode {
        config.default_mode = m.clone();
    }

    if let Some(n) = months {
        config.next_months = n;
    }

    config.save().map_err(|e| format!("Failed to save config: {}", e))?;

    Ok("Configuration saved".to_string())
}

/// Toggle automatic schedule
#[tauri::command]
fn toggle_schedule(enabled: bool) -> Result<String, String> {
    if enabled {
        install_schedule().map_err(|e| format!("Failed to install schedule: {}", e))?;
        Ok("Weekly schedule installed".to_string())
    } else {
        uninstall_schedule().map_err(|e| format!("Failed to uninstall schedule: {}", e))?;
        Ok("Weekly schedule removed".to_string())
    }
}

/// Get schedule status
#[tauri::command]
fn get_schedule_status() -> bool {
    is_schedule_installed()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            generate_preview,
            set_wallpaper_cmd,
            get_config,
            save_config,
            toggle_schedule,
            get_schedule_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
