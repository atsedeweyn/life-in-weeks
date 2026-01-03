//! Life in Weeks CLI
//!
//! Command-line interface for generating and managing Life in Weeks wallpapers.

use anyhow::{Context, Result};
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use liw_core::{
    Config, Mode, WeekGrid,
    render_grid, set_wallpaper,
    install_schedule, uninstall_schedule,
    renderer::save_grid,
    scheduler::is_schedule_installed,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "liw",
    about = "Life in Weeks - Dynamic wallpaper generator",
    version,
    author
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate and optionally set wallpaper
    Generate {
        /// Mode: life, year-end, or next-months
        #[arg(short, long, default_value = "year-end")]
        mode: String,

        /// Date of birth (YYYY-MM-DD) for life mode
        #[arg(long)]
        dob: Option<String>,

        /// Expected lifespan in years (default: 80)
        #[arg(long)]
        lifespan: Option<u8>,

        /// Number of months for next-months mode (default: 6)
        #[arg(long)]
        months: Option<u8>,

        /// Just preview, don't set as wallpaper
        #[arg(short, long)]
        preview: bool,

        /// Output file path (default: auto-generated)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Screen width
        #[arg(long)]
        width: Option<u32>,

        /// Screen height
        #[arg(long)]
        height: Option<u32>,

        /// Theme: minimal, terminal, dark, sunset
        #[arg(short, long)]
        theme: Option<String>,
    },

    /// Manage configuration
    #[command(subcommand)]
    Config(ConfigCommands),

    /// Manage weekly schedule
    #[command(subcommand)]
    Schedule(ScheduleCommands),
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show,

    /// Set a configuration value
    Set {
        /// Config key (dob, lifespan, theme, width, height, default_mode, next_months)
        key: String,
        /// Value to set
        value: String,
    },

    /// Reset configuration to defaults
    Reset,

    /// Show config file path
    Path,
}

#[derive(Subcommand)]
enum ScheduleCommands {
    /// Install weekly schedule
    Install,

    /// Uninstall weekly schedule
    Uninstall,

    /// Check schedule status
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            mode,
            dob,
            lifespan,
            months,
            preview,
            output,
            width,
            height,
            theme,
        } => cmd_generate(mode, dob, lifespan, months, preview, output, width, height, theme),
        Commands::Config(cmd) => match cmd {
            ConfigCommands::Show => cmd_config_show(),
            ConfigCommands::Set { key, value } => cmd_config_set(&key, &value),
            ConfigCommands::Reset => cmd_config_reset(),
            ConfigCommands::Path => cmd_config_path(),
        },
        Commands::Schedule(cmd) => match cmd {
            ScheduleCommands::Install => cmd_schedule_install(),
            ScheduleCommands::Uninstall => cmd_schedule_uninstall(),
            ScheduleCommands::Status => cmd_schedule_status(),
        },
    }
}

fn cmd_generate(
    mode_str: String,
    dob_str: Option<String>,
    lifespan: Option<u8>,
    months: Option<u8>,
    preview: bool,
    output: Option<PathBuf>,
    width: Option<u32>,
    height: Option<u32>,
    theme_str: Option<String>,
) -> Result<()> {
    // Load config for defaults
    let mut config = Config::load().unwrap_or_default();

    // Apply overrides from CLI
    if let Some(ref t) = theme_str {
        config.set("theme", t)?;
    }
    if let Some(w) = width {
        config.screen_width = w;
    }
    if let Some(h) = height {
        config.screen_height = h;
    }
    if let Some(l) = lifespan {
        config.lifespan_years = l;
    }
    if let Some(m) = months {
        config.next_months = m;
    }

    // Parse DOB
    let dob = if let Some(ref dob_str) = dob_str {
        Some(NaiveDate::parse_from_str(dob_str, "%Y-%m-%d")
            .with_context(|| format!("Invalid date format: {}. Use YYYY-MM-DD", dob_str))?)
    } else {
        config.dob
    };

    // Parse mode
    let mode = Mode::from_str_with_params(
        &mode_str,
        dob,
        Some(config.lifespan_years),
        Some(config.next_months),
    )
    .map_err(|e| anyhow::anyhow!(e))?;

    println!("Generating wallpaper...");
    println!("  Mode: {:?}", mode);
    println!("  Resolution: {}x{}", config.screen_width, config.screen_height);
    println!("  Theme: {:?}", config.theme);

    // Calculate the grid
    let grid = WeekGrid::calculate(&mode);
    println!("\n{}", grid.title);
    println!("{}", grid.subtitle);
    println!("  Grid: {} columns x {} rows", grid.columns, grid.rows);

    // Render the image
    let image = render_grid(&grid, &config.theme, config.screen_width, config.screen_height);

    // Determine output path
    let output_path = if let Some(path) = output {
        path
    } else {
        Config::default_output_path()?
    };

    // Ensure parent directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Save the image
    save_grid(&image, &output_path)?;
    println!("\nWallpaper saved to: {:?}", output_path);

    // Set as wallpaper unless preview mode
    if !preview {
        println!("Setting as wallpaper...");
        set_wallpaper(&output_path)?;
        println!("Done! Wallpaper has been updated.");
    } else {
        println!("Preview mode - wallpaper not set.");
    }

    Ok(())
}

fn cmd_config_show() -> Result<()> {
    let config = Config::load().unwrap_or_default();
    
    println!("Life in Weeks Configuration");
    println!("============================");
    println!();
    println!("Date of Birth:     {:?}", config.dob);
    println!("Lifespan (years):  {}", config.lifespan_years);
    println!("Theme:             {:?}", config.theme);
    println!("Screen Width:      {}", config.screen_width);
    println!("Screen Height:     {}", config.screen_height);
    println!("Default Mode:      {}", config.default_mode);
    println!("Next Months:       {}", config.next_months);
    
    Ok(())
}

fn cmd_config_set(key: &str, value: &str) -> Result<()> {
    let mut config = Config::load().unwrap_or_default();
    
    config.set(key, value)?;
    config.save()?;
    
    println!("Configuration updated: {} = {}", key, value);
    
    Ok(())
}

fn cmd_config_reset() -> Result<()> {
    let config = Config::default();
    config.save()?;
    
    println!("Configuration reset to defaults.");
    
    Ok(())
}

fn cmd_config_path() -> Result<()> {
    let path = Config::default_path()?;
    println!("Config file: {:?}", path);
    
    let output_path = Config::default_output_path()?;
    println!("Output file: {:?}", output_path);
    
    Ok(())
}

fn cmd_schedule_install() -> Result<()> {
    install_schedule()
}

fn cmd_schedule_uninstall() -> Result<()> {
    uninstall_schedule()
}

fn cmd_schedule_status() -> Result<()> {
    if is_schedule_installed() {
        println!("Weekly schedule is INSTALLED.");
        println!("The wallpaper will update every Monday at 6:00 AM.");
    } else {
        println!("Weekly schedule is NOT installed.");
        println!("Run 'liw schedule install' to enable automatic updates.");
    }
    
    Ok(())
}
