//! OS scheduler integration for weekly wallpaper regeneration
//!
//! Creates scheduled tasks on Windows (Task Scheduler) and macOS (launchd).

use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

/// Install a weekly schedule to regenerate the wallpaper
pub fn install_schedule() -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        install_schedule_windows()
    }

    #[cfg(target_os = "macos")]
    {
        install_schedule_macos()
    }

    #[cfg(target_os = "linux")]
    {
        install_schedule_linux()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        anyhow::bail!("Scheduling not supported on this platform")
    }
}

/// Uninstall the weekly schedule
pub fn uninstall_schedule() -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        uninstall_schedule_windows()
    }

    #[cfg(target_os = "macos")]
    {
        uninstall_schedule_macos()
    }

    #[cfg(target_os = "linux")]
    {
        uninstall_schedule_linux()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        anyhow::bail!("Scheduling not supported on this platform")
    }
}

/// Check if the schedule is installed
pub fn is_schedule_installed() -> bool {
    #[cfg(target_os = "windows")]
    {
        is_schedule_installed_windows()
    }

    #[cfg(target_os = "macos")]
    {
        is_schedule_installed_macos()
    }

    #[cfg(target_os = "linux")]
    {
        is_schedule_installed_linux()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        false
    }
}

/// Get the path to the current executable
fn get_exe_path() -> Result<PathBuf> {
    env::current_exe().context("Could not determine executable path")
}

// ============================================================================
// Windows Implementation
// ============================================================================

#[cfg(target_os = "windows")]
const TASK_NAME: &str = "LifeInWeeksWallpaper";

#[cfg(target_os = "windows")]
fn install_schedule_windows() -> Result<()> {
    use std::process::Command;

    let exe_path = get_exe_path()?;
    let exe_path_str = exe_path
        .to_str()
        .context("Executable path contains invalid UTF-8")?;

    // Create a weekly task that runs every Monday at 6:00 AM
    let output = Command::new("schtasks")
        .args([
            "/Create",
            "/SC",
            "WEEKLY",
            "/D",
            "MON",
            "/TN",
            TASK_NAME,
            "/TR",
            &format!("\"{}\" generate", exe_path_str),
            "/ST",
            "06:00",
            "/F", // Force create (overwrite if exists)
        ])
        .output()
        .context("Failed to execute schtasks")?;

    if output.status.success() {
        println!("Weekly schedule installed successfully.");
        println!("The wallpaper will update every Monday at 6:00 AM.");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to create scheduled task: {}", stderr)
    }
}

#[cfg(target_os = "windows")]
fn uninstall_schedule_windows() -> Result<()> {
    use std::process::Command;

    let output = Command::new("schtasks")
        .args(["/Delete", "/TN", TASK_NAME, "/F"])
        .output()
        .context("Failed to execute schtasks")?;

    if output.status.success() {
        println!("Weekly schedule removed successfully.");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Don't fail if task doesn't exist
        if stderr.contains("does not exist") {
            println!("Schedule was not installed.");
            Ok(())
        } else {
            anyhow::bail!("Failed to remove scheduled task: {}", stderr)
        }
    }
}

#[cfg(target_os = "windows")]
fn is_schedule_installed_windows() -> bool {
    use std::process::Command;

    Command::new("schtasks")
        .args(["/Query", "/TN", TASK_NAME])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

// ============================================================================
// macOS Implementation
// ============================================================================

#[cfg(target_os = "macos")]
const LAUNCHD_LABEL: &str = "com.lifeinweeks.wallpaper";

#[cfg(target_os = "macos")]
fn get_plist_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    Ok(home
        .join("Library/LaunchAgents")
        .join(format!("{}.plist", LAUNCHD_LABEL)))
}

#[cfg(target_os = "macos")]
fn install_schedule_macos() -> Result<()> {
    use std::process::Command;

    let exe_path = get_exe_path()?;
    let exe_path_str = exe_path
        .to_str()
        .context("Executable path contains invalid UTF-8")?;

    let plist_path = get_plist_path()?;

    // Create LaunchAgents directory if it doesn't exist
    if let Some(parent) = plist_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create the plist file
    // Schedule: Every Monday at 6:00 AM (Weekday 1 = Monday)
    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>generate</string>
    </array>
    <key>StartCalendarInterval</key>
    <dict>
        <key>Weekday</key>
        <integer>1</integer>
        <key>Hour</key>
        <integer>6</integer>
        <key>Minute</key>
        <integer>0</integer>
    </dict>
    <key>RunAtLoad</key>
    <false/>
</dict>
</plist>
"#,
        LAUNCHD_LABEL, exe_path_str
    );

    fs::write(&plist_path, plist_content).context("Failed to write plist file")?;

    // Load the job
    let output = Command::new("launchctl")
        .args(["load", plist_path.to_str().unwrap()])
        .output()
        .context("Failed to execute launchctl")?;

    if output.status.success() {
        println!("Weekly schedule installed successfully.");
        println!("The wallpaper will update every Monday at 6:00 AM.");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to load launch agent: {}", stderr)
    }
}

#[cfg(target_os = "macos")]
fn uninstall_schedule_macos() -> Result<()> {
    use std::process::Command;

    let plist_path = get_plist_path()?;

    if plist_path.exists() {
        // Unload the job
        let _ = Command::new("launchctl")
            .args(["unload", plist_path.to_str().unwrap()])
            .output();

        // Remove the plist file
        fs::remove_file(&plist_path).context("Failed to remove plist file")?;

        println!("Weekly schedule removed successfully.");
    } else {
        println!("Schedule was not installed.");
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn is_schedule_installed_macos() -> bool {
    get_plist_path().map(|path| path.exists()).unwrap_or(false)
}

// ============================================================================
// Linux Implementation
// ============================================================================

#[cfg(target_os = "linux")]
fn get_systemd_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("Could not determine config directory")?;
    Ok(config_dir.join("systemd/user"))
}

#[cfg(target_os = "linux")]
fn install_schedule_linux() -> Result<()> {
    use std::process::Command;

    let exe_path = get_exe_path()?;
    let exe_path_str = exe_path
        .to_str()
        .context("Executable path contains invalid UTF-8")?;

    let systemd_dir = get_systemd_path()?;
    fs::create_dir_all(&systemd_dir)?;

    // Create the service file
    let service_content = format!(
        r#"[Unit]
Description=Life in Weeks Wallpaper Generator

[Service]
Type=oneshot
ExecStart={} generate
"#,
        exe_path_str
    );

    let service_path = systemd_dir.join("liw-wallpaper.service");
    fs::write(&service_path, service_content).context("Failed to write service file")?;

    // Create the timer file (every Monday at 6:00 AM)
    let timer_content = r#"[Unit]
Description=Weekly Life in Weeks Wallpaper Update

[Timer]
OnCalendar=Mon *-*-* 06:00:00
Persistent=true

[Install]
WantedBy=timers.target
"#;

    let timer_path = systemd_dir.join("liw-wallpaper.timer");
    fs::write(&timer_path, timer_content).context("Failed to write timer file")?;

    // Reload systemd and enable the timer
    Command::new("systemctl")
        .args(["--user", "daemon-reload"])
        .output()?;

    let output = Command::new("systemctl")
        .args(["--user", "enable", "--now", "liw-wallpaper.timer"])
        .output()
        .context("Failed to enable timer")?;

    if output.status.success() {
        println!("Weekly schedule installed successfully.");
        println!("The wallpaper will update every Monday at 6:00 AM.");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to enable timer: {}", stderr)
    }
}

#[cfg(target_os = "linux")]
fn uninstall_schedule_linux() -> Result<()> {
    use std::process::Command;

    // Disable and stop the timer
    let _ = Command::new("systemctl")
        .args(["--user", "disable", "--now", "liw-wallpaper.timer"])
        .output();

    // Remove the files
    let systemd_dir = get_systemd_path()?;
    let _ = fs::remove_file(systemd_dir.join("liw-wallpaper.service"));
    let _ = fs::remove_file(systemd_dir.join("liw-wallpaper.timer"));

    // Reload systemd
    let _ = Command::new("systemctl")
        .args(["--user", "daemon-reload"])
        .output();

    println!("Weekly schedule removed successfully.");
    Ok(())
}

#[cfg(target_os = "linux")]
fn is_schedule_installed_linux() -> bool {
    use std::process::Command;

    Command::new("systemctl")
        .args(["--user", "is-enabled", "liw-wallpaper.timer"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_exe_path() {
        let result = get_exe_path();
        assert!(result.is_ok());
    }
}
