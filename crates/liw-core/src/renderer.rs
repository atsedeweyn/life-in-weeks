//! Grid renderer for Life in Weeks
//!
//! Generates wallpaper images with the week grid visualization.

use crate::config::Theme;
use crate::modes::{WeekGrid, WeekStatus};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use std::path::Path;

/// Padding around the grid (percentage of screen size)
const PADDING_PERCENT: f32 = 0.08;
/// Gap between cells (percentage of cell size)
const GAP_PERCENT: f32 = 0.15;
/// Corner radius for cells (percentage of cell size)
const CORNER_RADIUS_PERCENT: f32 = 0.2;

/// Render the week grid to an image
pub fn render_grid(grid: &WeekGrid, theme: &Theme, width: u32, height: u32) -> DynamicImage {
    let colors = theme.colors();
    let mut img: RgbaImage = ImageBuffer::from_pixel(width, height, Rgba(colors.background));

    // Calculate layout
    let padding_x = (width as f32 * PADDING_PERCENT) as u32;
    let padding_y = (height as f32 * PADDING_PERCENT) as u32;
    
    // Reserve space for title and subtitle
    let title_height = (height as f32 * 0.06) as u32;
    let subtitle_height = (height as f32 * 0.03) as u32;
    let header_height = title_height + subtitle_height + padding_y / 2;
    
    let grid_width = width - 2 * padding_x;
    let grid_height = height - 2 * padding_y - header_height;

    // Calculate cell size based on grid dimensions
    let cell_width = grid_width as f32 / grid.columns as f32;
    let cell_height = grid_height as f32 / grid.rows as f32;
    let cell_size = cell_width.min(cell_height);
    
    let gap = (cell_size * GAP_PERCENT) as u32;
    let actual_cell_size = (cell_size - gap as f32) as u32;
    let corner_radius = (actual_cell_size as f32 * CORNER_RADIUS_PERCENT) as u32;

    // Center the grid
    let total_grid_width = (cell_size * grid.columns as f32) as u32;
    let total_grid_height = (cell_size * grid.rows as f32) as u32;
    let start_x = (width - total_grid_width) / 2;
    let start_y = header_height + (grid_height - total_grid_height) / 2 + padding_y;

    // Draw each week cell
    for (i, week) in grid.weeks.iter().enumerate() {
        let col = i % grid.columns;
        let row = i / grid.columns;

        let x = start_x + (col as f32 * cell_size) as u32 + gap / 2;
        let y = start_y + (row as f32 * cell_size) as u32 + gap / 2;

        let cell_color = match week.status {
            WeekStatus::Past => colors.past_week,
            WeekStatus::Current => colors.current_week,
            WeekStatus::Future => colors.future_week,
        };

        // Draw the cell (rounded rectangle)
        draw_rounded_rect(&mut img, x, y, actual_cell_size, actual_cell_size, corner_radius, cell_color);

        // Draw accent border for current week
        if week.status == WeekStatus::Current {
            draw_rounded_rect_outline(
                &mut img,
                x.saturating_sub(2),
                y.saturating_sub(2),
                actual_cell_size + 4,
                actual_cell_size + 4,
                corner_radius + 1,
                colors.accent,
                2,
            );
        }
    }

    // Draw title (simple pixel-based text rendering)
    draw_text_centered(
        &mut img,
        &grid.title,
        width / 2,
        padding_y + title_height / 2,
        title_height / 2,
        colors.text,
    );

    // Draw subtitle
    draw_text_centered(
        &mut img,
        &grid.subtitle,
        width / 2,
        padding_y + title_height + subtitle_height / 2,
        subtitle_height / 2,
        colors.text,
    );

    DynamicImage::ImageRgba8(img)
}

/// Save a rendered grid to a file
pub fn save_grid(image: &DynamicImage, path: &Path) -> Result<(), image::ImageError> {
    image.save(path)
}

/// Render and save in one step
pub fn render_and_save(
    grid: &WeekGrid,
    theme: &Theme,
    width: u32,
    height: u32,
    path: &Path,
) -> Result<(), image::ImageError> {
    let image = render_grid(grid, theme, width, height);
    save_grid(&image, path)
}

/// Draw a filled rounded rectangle
fn draw_rounded_rect(
    img: &mut RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    radius: u32,
    color: [u8; 4],
) {
    let radius = radius.min(width / 2).min(height / 2);
    let pixel = Rgba(color);
    let (img_width, img_height) = img.dimensions();

    for dy in 0..height {
        for dx in 0..width {
            let px = x + dx;
            let py = y + dy;

            if px >= img_width || py >= img_height {
                continue;
            }

            // Check if pixel is inside rounded rectangle
            let inside = if dx < radius && dy < radius {
                // Top-left corner
                is_in_circle(dx, dy, radius, radius, radius)
            } else if dx >= width - radius && dy < radius {
                // Top-right corner
                is_in_circle(dx, dy, width - radius - 1, radius, radius)
            } else if dx < radius && dy >= height - radius {
                // Bottom-left corner
                is_in_circle(dx, dy, radius, height - radius - 1, radius)
            } else if dx >= width - radius && dy >= height - radius {
                // Bottom-right corner
                is_in_circle(dx, dy, width - radius - 1, height - radius - 1, radius)
            } else {
                true
            };

            if inside {
                img.put_pixel(px, py, pixel);
            }
        }
    }
}

/// Draw a rounded rectangle outline
fn draw_rounded_rect_outline(
    img: &mut RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    radius: u32,
    color: [u8; 4],
    thickness: u32,
) {
    let radius = radius.min(width / 2).min(height / 2);
    let pixel = Rgba(color);
    let (img_width, img_height) = img.dimensions();

    for dy in 0..height {
        for dx in 0..width {
            let px = x + dx;
            let py = y + dy;

            if px >= img_width || py >= img_height {
                continue;
            }

            // Check if on the border
            let on_edge = dx < thickness
                || dx >= width - thickness
                || dy < thickness
                || dy >= height - thickness;

            if !on_edge {
                continue;
            }

            // Check if pixel is inside rounded rectangle
            let inside = if dx < radius && dy < radius {
                is_in_circle(dx, dy, radius, radius, radius)
            } else if dx >= width - radius && dy < radius {
                is_in_circle(dx, dy, width - radius - 1, radius, radius)
            } else if dx < radius && dy >= height - radius {
                is_in_circle(dx, dy, radius, height - radius - 1, radius)
            } else if dx >= width - radius && dy >= height - radius {
                is_in_circle(dx, dy, width - radius - 1, height - radius - 1, radius)
            } else {
                true
            };

            if inside {
                img.put_pixel(px, py, pixel);
            }
        }
    }
}

/// Check if a point is inside a circle
fn is_in_circle(x: u32, y: u32, cx: u32, cy: u32, r: u32) -> bool {
    let dx = x as i32 - cx as i32;
    let dy = y as i32 - cy as i32;
    (dx * dx + dy * dy) <= (r * r) as i32
}

/// Draw centered text (simplified bitmap font)
/// This is a basic implementation - for production, consider using rusttype or ab_glyph
fn draw_text_centered(
    img: &mut RgbaImage,
    text: &str,
    center_x: u32,
    center_y: u32,
    font_size: u32,
    color: [u8; 4],
) {
    // Simple bitmap-based character rendering
    // Each character is roughly 0.6 * font_size wide
    let char_width = (font_size as f32 * 0.6) as u32;
    let total_width = char_width * text.len() as u32;
    let start_x = center_x.saturating_sub(total_width / 2);
    let start_y = center_y.saturating_sub(font_size / 2);

    let pixel = Rgba(color);

    for (i, c) in text.chars().enumerate() {
        let char_x = start_x + (i as u32 * char_width);
        draw_char(img, c, char_x, start_y, font_size, pixel);
    }
}

/// Draw a single character using a simple bitmap approach
fn draw_char(img: &mut RgbaImage, c: char, x: u32, y: u32, size: u32, pixel: Rgba<u8>) {
    let bitmap = get_char_bitmap(c);
    let scale = size as f32 / 8.0;
    let (img_width, img_height) = img.dimensions();

    for (row, bits) in bitmap.iter().enumerate() {
        for col in 0..6 {
            if (bits >> (5 - col)) & 1 == 1 {
                let px = x + (col as f32 * scale) as u32;
                let py = y + (row as f32 * scale) as u32;

                // Draw a scaled pixel (multiple pixels for larger sizes)
                for dy in 0..scale.ceil() as u32 {
                    for dx in 0..scale.ceil() as u32 {
                        let final_x = px + dx;
                        let final_y = py + dy;
                        if final_x < img_width && final_y < img_height {
                            img.put_pixel(final_x, final_y, pixel);
                        }
                    }
                }
            }
        }
    }
}

/// Get a simple 6x8 bitmap for a character
fn get_char_bitmap(c: char) -> [u8; 8] {
    match c {
        '0' => [0b011110, 0b110011, 0b110011, 0b110011, 0b110011, 0b110011, 0b011110, 0b000000],
        '1' => [0b001100, 0b011100, 0b001100, 0b001100, 0b001100, 0b001100, 0b111111, 0b000000],
        '2' => [0b011110, 0b110011, 0b000011, 0b000110, 0b001100, 0b011000, 0b111111, 0b000000],
        '3' => [0b011110, 0b110011, 0b000011, 0b001110, 0b000011, 0b110011, 0b011110, 0b000000],
        '4' => [0b000110, 0b001110, 0b011110, 0b110110, 0b111111, 0b000110, 0b000110, 0b000000],
        '5' => [0b111111, 0b110000, 0b111110, 0b000011, 0b000011, 0b110011, 0b011110, 0b000000],
        '6' => [0b011110, 0b110000, 0b111110, 0b110011, 0b110011, 0b110011, 0b011110, 0b000000],
        '7' => [0b111111, 0b000011, 0b000110, 0b001100, 0b011000, 0b011000, 0b011000, 0b000000],
        '8' => [0b011110, 0b110011, 0b110011, 0b011110, 0b110011, 0b110011, 0b011110, 0b000000],
        '9' => [0b011110, 0b110011, 0b110011, 0b011111, 0b000011, 0b000011, 0b011110, 0b000000],
        'A' | 'a' => [0b001100, 0b011110, 0b110011, 0b110011, 0b111111, 0b110011, 0b110011, 0b000000],
        'B' | 'b' => [0b111110, 0b110011, 0b110011, 0b111110, 0b110011, 0b110011, 0b111110, 0b000000],
        'C' | 'c' => [0b011110, 0b110011, 0b110000, 0b110000, 0b110000, 0b110011, 0b011110, 0b000000],
        'D' | 'd' => [0b111100, 0b110110, 0b110011, 0b110011, 0b110011, 0b110110, 0b111100, 0b000000],
        'E' | 'e' => [0b111111, 0b110000, 0b110000, 0b111110, 0b110000, 0b110000, 0b111111, 0b000000],
        'F' | 'f' => [0b111111, 0b110000, 0b110000, 0b111110, 0b110000, 0b110000, 0b110000, 0b000000],
        'G' | 'g' => [0b011110, 0b110011, 0b110000, 0b110111, 0b110011, 0b110011, 0b011110, 0b000000],
        'H' | 'h' => [0b110011, 0b110011, 0b110011, 0b111111, 0b110011, 0b110011, 0b110011, 0b000000],
        'I' | 'i' => [0b111111, 0b001100, 0b001100, 0b001100, 0b001100, 0b001100, 0b111111, 0b000000],
        'J' | 'j' => [0b000111, 0b000011, 0b000011, 0b000011, 0b110011, 0b110011, 0b011110, 0b000000],
        'K' | 'k' => [0b110011, 0b110110, 0b111100, 0b111000, 0b111100, 0b110110, 0b110011, 0b000000],
        'L' | 'l' => [0b110000, 0b110000, 0b110000, 0b110000, 0b110000, 0b110000, 0b111111, 0b000000],
        'M' | 'm' => [0b110011, 0b111111, 0b111111, 0b110011, 0b110011, 0b110011, 0b110011, 0b000000],
        'N' | 'n' => [0b110011, 0b111011, 0b111111, 0b110111, 0b110011, 0b110011, 0b110011, 0b000000],
        'O' | 'o' => [0b011110, 0b110011, 0b110011, 0b110011, 0b110011, 0b110011, 0b011110, 0b000000],
        'P' | 'p' => [0b111110, 0b110011, 0b110011, 0b111110, 0b110000, 0b110000, 0b110000, 0b000000],
        'Q' | 'q' => [0b011110, 0b110011, 0b110011, 0b110011, 0b110111, 0b011110, 0b000011, 0b000000],
        'R' | 'r' => [0b111110, 0b110011, 0b110011, 0b111110, 0b111100, 0b110110, 0b110011, 0b000000],
        'S' | 's' => [0b011110, 0b110011, 0b110000, 0b011110, 0b000011, 0b110011, 0b011110, 0b000000],
        'T' | 't' => [0b111111, 0b001100, 0b001100, 0b001100, 0b001100, 0b001100, 0b001100, 0b000000],
        'U' | 'u' => [0b110011, 0b110011, 0b110011, 0b110011, 0b110011, 0b110011, 0b011110, 0b000000],
        'V' | 'v' => [0b110011, 0b110011, 0b110011, 0b110011, 0b011110, 0b001100, 0b001100, 0b000000],
        'W' | 'w' => [0b110011, 0b110011, 0b110011, 0b110011, 0b111111, 0b111111, 0b110011, 0b000000],
        'X' | 'x' => [0b110011, 0b110011, 0b011110, 0b001100, 0b011110, 0b110011, 0b110011, 0b000000],
        'Y' | 'y' => [0b110011, 0b110011, 0b011110, 0b001100, 0b001100, 0b001100, 0b001100, 0b000000],
        'Z' | 'z' => [0b111111, 0b000011, 0b000110, 0b001100, 0b011000, 0b110000, 0b111111, 0b000000],
        ' ' => [0b000000, 0b000000, 0b000000, 0b000000, 0b000000, 0b000000, 0b000000, 0b000000],
        '-' => [0b000000, 0b000000, 0b000000, 0b111111, 0b000000, 0b000000, 0b000000, 0b000000],
        '(' => [0b000110, 0b001100, 0b011000, 0b011000, 0b011000, 0b001100, 0b000110, 0b000000],
        ')' => [0b110000, 0b011000, 0b001100, 0b001100, 0b001100, 0b011000, 0b110000, 0b000000],
        '%' => [0b110001, 0b110011, 0b000110, 0b001100, 0b011000, 0b110011, 0b100011, 0b000000],
        _ => [0b000000, 0b000000, 0b000000, 0b000000, 0b000000, 0b000000, 0b000000, 0b000000],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modes::Mode;
    use chrono::NaiveDate;

    #[test]
    fn test_render_year_end() {
        let grid = WeekGrid::calculate(&Mode::YearEnd);
        let image = render_grid(&grid, &Theme::SoftDark, 1920, 1080);
        
        assert_eq!(image.width(), 1920);
        assert_eq!(image.height(), 1080);
    }

    #[test]
    fn test_render_life_mode() {
        let dob = NaiveDate::from_ymd_opt(1990, 1, 1).unwrap();
        let grid = WeekGrid::calculate(&Mode::Life {
            dob,
            lifespan_years: 80,
        });
        let image = render_grid(&grid, &Theme::TerminalGreen, 1920, 1080);
        
        assert_eq!(image.width(), 1920);
        assert_eq!(image.height(), 1080);
    }
}
