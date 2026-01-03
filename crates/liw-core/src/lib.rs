//! Life in Weeks Core Library
//!
//! This crate provides the core functionality for generating "Life in Weeks" wallpapers.
//! It includes date calculations, grid rendering, wallpaper setting, and scheduling.

pub mod config;
pub mod modes;
pub mod renderer;
pub mod scheduler;
pub mod wallpaper;

pub use config::{Config, Theme};
pub use modes::{Mode, WeekGrid, WeekStatus};
pub use renderer::render_grid;
pub use scheduler::{install_schedule, uninstall_schedule};
pub use wallpaper::set_wallpaper;
