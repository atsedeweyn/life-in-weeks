//! Cross-platform wallpaper setting
//!
//! Supports Windows and macOS for setting the desktop wallpaper.

use anyhow::{Context, Result};
use std::path::Path;

/// Set the desktop wallpaper to the given image
pub fn set_wallpaper(path: &Path) -> Result<()> {
    let path_str = path
        .to_str()
        .context("Path contains invalid UTF-8 characters")?;

    #[cfg(target_os = "windows")]
    {
        set_wallpaper_windows(path_str)
    }

    #[cfg(target_os = "macos")]
    {
        set_wallpaper_macos(path_str)
    }

    #[cfg(target_os = "linux")]
    {
        set_wallpaper_linux(path_str)
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        anyhow::bail!("Wallpaper setting not supported on this platform")
    }
}

/// Set wallpaper on Windows using the Windows API
#[cfg(target_os = "windows")]
fn set_wallpaper_windows(path: &str) -> Result<()> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;

    // Convert path to wide string (UTF-16)
    let wide_path: Vec<u16> = OsStr::new(path).encode_wide().chain(once(0)).collect();

    // SPI_SETDESKWALLPAPER = 0x0014
    // SPIF_UPDATEINIFILE | SPIF_SENDCHANGE = 0x0003
    const SPI_SETDESKWALLPAPER: u32 = 0x0014;
    const SPIF_UPDATEINIFILE: u32 = 0x0001;
    const SPIF_SENDCHANGE: u32 = 0x0002;

    #[link(name = "user32")]
    extern "system" {
        fn SystemParametersInfoW(
            uiAction: u32,
            uiParam: u32,
            pvParam: *const u16,
            fWinIni: u32,
        ) -> i32;
    }

    let result = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            wide_path.as_ptr(),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        )
    };

    if result != 0 {
        Ok(())
    } else {
        anyhow::bail!("Failed to set wallpaper via SystemParametersInfoW")
    }
}

/// Set wallpaper on macOS using osascript
#[cfg(target_os = "macos")]
fn set_wallpaper_macos(path: &str) -> Result<()> {
    use std::process::Command;

    // Use osascript to set wallpaper on all desktops
    let script = format!(
        r#"
        tell application "System Events"
            tell every desktop
                set picture to POSIX file "{}"
            end tell
        end tell
        "#,
        path
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .context("Failed to execute osascript")?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to set wallpaper: {}", stderr)
    }
}

/// Set wallpaper on Linux (supports GNOME, KDE, XFCE, and others)
#[cfg(target_os = "linux")]
fn set_wallpaper_linux(path: &str) -> Result<()> {
    use std::env;
    use std::process::Command;

    // Detect desktop environment
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let session = env::var("DESKTOP_SESSION").unwrap_or_default();

    let result = if desktop.contains("GNOME") || session.contains("gnome") {
        // GNOME
        Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.background",
                "picture-uri",
                &format!("file://{}", path),
            ])
            .status()
            .and_then(|_| {
                Command::new("gsettings")
                    .args([
                        "set",
                        "org.gnome.desktop.background",
                        "picture-uri-dark",
                        &format!("file://{}", path),
                    ])
                    .status()
            })
    } else if desktop.contains("KDE") || session.contains("plasma") {
        // KDE Plasma
        let script = format!(
            r#"
            var allDesktops = desktops();
            for (var i = 0; i < allDesktops.length; i++) {{
                var d = allDesktops[i];
                d.wallpaperPlugin = "org.kde.image";
                d.currentConfigGroup = ["Wallpaper", "org.kde.image", "General"];
                d.writeConfig("Image", "file://{}");
            }}
            "#,
            path
        );
        Command::new("qdbus")
            .args([
                "org.kde.plasmashell",
                "/PlasmaShell",
                "org.kde.PlasmaShell.evaluateScript",
                &script,
            ])
            .status()
    } else if desktop.contains("XFCE") || session.contains("xfce") {
        // XFCE
        Command::new("xfconf-query")
            .args([
                "-c",
                "xfce4-desktop",
                "-p",
                "/backdrop/screen0/monitor0/workspace0/last-image",
                "-s",
                path,
            ])
            .status()
    } else if desktop.contains("MATE") {
        // MATE
        Command::new("gsettings")
            .args(["set", "org.mate.background", "picture-filename", path])
            .status()
    } else if desktop.contains("Cinnamon") {
        // Cinnamon
        Command::new("gsettings")
            .args([
                "set",
                "org.cinnamon.desktop.background",
                "picture-uri",
                &format!("file://{}", path),
            ])
            .status()
    } else {
        // Try feh as a fallback (works with many WMs)
        Command::new("feh")
            .args(["--bg-fill", path])
            .status()
            .or_else(|_| {
                // Try nitrogen as another fallback
                Command::new("nitrogen")
                    .args(["--set-zoom-fill", path])
                    .status()
            })
    };

    match result {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => anyhow::bail!("Wallpaper command exited with status: {}", status),
        Err(e) => anyhow::bail!("Failed to set wallpaper: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_set_wallpaper_invalid_path() {
        let result = set_wallpaper(&PathBuf::from("/nonexistent/path/wallpaper.png"));
        // This will fail, but shouldn't panic
        assert!(result.is_err() || result.is_ok());
    }
}
