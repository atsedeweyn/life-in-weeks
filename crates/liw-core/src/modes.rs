//! Date calculation modes for Life in Weeks
//!
//! Supports three modes:
//! - Next N months
//! - Until end of year
//! - Life in weeks (DOB to expected lifespan)

use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};

/// The mode for calculating weeks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Mode {
    /// Show the next N months from today
    NextMonths { months: u8 },
    /// Show weeks from now until the end of the current year
    YearEnd,
    /// Show entire life in weeks from DOB to expected lifespan
    Life { dob: NaiveDate, lifespan_years: u8 },
}

impl Mode {
    /// Parse mode from string with optional parameters
    pub fn from_str_with_params(
        mode: &str,
        dob: Option<NaiveDate>,
        lifespan: Option<u8>,
        months: Option<u8>,
    ) -> Result<Self, String> {
        match mode.to_lowercase().as_str() {
            "next-months" | "next_months" | "months" => Ok(Mode::NextMonths {
                months: months.unwrap_or(6),
            }),
            "year-end" | "year_end" | "year" => Ok(Mode::YearEnd),
            "life" | "life-weeks" | "life_weeks" => {
                let dob = dob.ok_or("DOB is required for life mode")?;
                Ok(Mode::Life {
                    dob,
                    lifespan_years: lifespan.unwrap_or(80),
                })
            }
            _ => Err(format!(
                "Unknown mode: {}. Options: next-months, year-end, life",
                mode
            )),
        }
    }
}

/// Status of a single week
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekStatus {
    /// Week is in the past
    Past,
    /// Current week
    Current,
    /// Week is in the future
    Future,
}

/// A single week in the grid
#[derive(Debug, Clone)]
pub struct Week {
    /// Start date of this week (Monday)
    pub start_date: NaiveDate,
    /// Status of this week
    pub status: WeekStatus,
    /// Optional label (e.g., year marker)
    pub label: Option<String>,
    /// Year this week belongs to (for life mode year markers)
    pub year: i32,
    /// Week number within the year (1-52/53)
    pub week_of_year: u32,
}

/// Grid of weeks for rendering
#[derive(Debug, Clone)]
pub struct WeekGrid {
    /// All weeks in the grid
    pub weeks: Vec<Week>,
    /// Total number of weeks
    pub total_weeks: usize,
    /// Number of elapsed (past) weeks
    pub elapsed_weeks: usize,
    /// Index of the current week (if in range)
    pub current_week_index: Option<usize>,
    /// Number of columns in the grid
    pub columns: usize,
    /// Number of rows in the grid
    pub rows: usize,
    /// Title for the grid
    pub title: String,
    /// Subtitle with stats
    pub subtitle: String,
}

impl WeekGrid {
    /// Calculate the grid based on the mode
    pub fn calculate(mode: &Mode) -> Self {
        let today = Local::now().date_naive();

        match mode {
            Mode::NextMonths { months } => Self::calculate_next_months(*months, today),
            Mode::YearEnd => Self::calculate_year_end(today),
            Mode::Life {
                dob,
                lifespan_years,
            } => Self::calculate_life(*dob, *lifespan_years, today),
        }
    }

    /// Calculate weeks for the next N months
    fn calculate_next_months(months: u8, today: NaiveDate) -> Self {
        let start = week_start(today);
        let end_date = add_months(today, months as i32);
        let end = week_start(end_date);

        let mut weeks = Vec::new();
        let mut current = start;
        let mut current_week_index = None;

        while current <= end {
            let status = if current <= today && today < current + chrono::Duration::days(7) {
                current_week_index = Some(weeks.len());
                WeekStatus::Current
            } else if current < today {
                WeekStatus::Past
            } else {
                WeekStatus::Future
            };

            weeks.push(Week {
                start_date: current,
                status,
                label: None,
                year: current.year(),
                week_of_year: current.iso_week().week(),
            });

            current += chrono::Duration::days(7);
        }

        let total_weeks = weeks.len();
        let elapsed_weeks = weeks
            .iter()
            .filter(|w| w.status == WeekStatus::Past)
            .count();

        // Calculate grid dimensions (prefer wider layout)
        let columns = (total_weeks as f64).sqrt().ceil() as usize;
        let rows = total_weeks.div_ceil(columns);

        Self {
            weeks,
            total_weeks,
            elapsed_weeks,
            current_week_index,
            columns,
            rows,
            title: format!("Next {} Months", months),
            subtitle: format!("{} weeks remaining", total_weeks - elapsed_weeks - 1),
        }
    }

    /// Calculate weeks until end of year
    fn calculate_year_end(today: NaiveDate) -> Self {
        let start = week_start(today);
        let year_end = NaiveDate::from_ymd_opt(today.year(), 12, 31).unwrap();
        let end = week_start(year_end);

        let mut weeks = Vec::new();
        let mut current = start;
        let mut current_week_index = None;

        while current <= end {
            let status = if current <= today && today < current + chrono::Duration::days(7) {
                current_week_index = Some(weeks.len());
                WeekStatus::Current
            } else if current < today {
                WeekStatus::Past
            } else {
                WeekStatus::Future
            };

            weeks.push(Week {
                start_date: current,
                status,
                label: None,
                year: current.year(),
                week_of_year: current.iso_week().week(),
            });

            current += chrono::Duration::days(7);
        }

        let total_weeks = weeks.len();
        let elapsed_weeks = weeks
            .iter()
            .filter(|w| w.status == WeekStatus::Past)
            .count();
        let remaining = total_weeks - elapsed_weeks - 1;

        // Single row for year-end mode
        let columns = total_weeks;
        let rows = 1;

        Self {
            weeks,
            total_weeks,
            elapsed_weeks,
            current_week_index,
            columns,
            rows,
            title: format!("Weeks Remaining in {}", today.year()),
            subtitle: format!("{} weeks to go", remaining),
        }
    }

    /// Calculate life in weeks from DOB to expected lifespan
    fn calculate_life(dob: NaiveDate, lifespan_years: u8, today: NaiveDate) -> Self {
        // Start from the Monday of the week containing DOB
        let start = week_start(dob);
        // End at expected lifespan
        let expected_end = add_years(dob, lifespan_years as i32);
        let end = week_start(expected_end);

        let mut weeks = Vec::new();
        let mut current = start;
        let mut current_week_index = None;
        let mut last_year = dob.year();

        while current <= end {
            let status = if current <= today && today < current + chrono::Duration::days(7) {
                current_week_index = Some(weeks.len());
                WeekStatus::Current
            } else if current < today {
                WeekStatus::Past
            } else {
                WeekStatus::Future
            };

            // Add year label at the start of each new year
            let label = if current.year() != last_year {
                last_year = current.year();
                Some(format!("{}", current.year()))
            } else {
                None
            };

            weeks.push(Week {
                start_date: current,
                status,
                label,
                year: current.year(),
                week_of_year: current.iso_week().week(),
            });

            current += chrono::Duration::days(7);
        }

        let total_weeks = weeks.len();
        let elapsed_weeks = weeks
            .iter()
            .filter(|w| w.status == WeekStatus::Past)
            .count();
        let remaining = total_weeks.saturating_sub(elapsed_weeks + 1);

        // Life mode: 52 columns (weeks per year) x lifespan rows
        let columns = 52;
        let rows = total_weeks.div_ceil(columns);

        let age_years = (today - dob).num_days() / 365;
        let percentage = (elapsed_weeks as f64 / total_weeks as f64 * 100.0) as u32;

        Self {
            weeks,
            total_weeks,
            elapsed_weeks,
            current_week_index,
            columns,
            rows,
            title: format!("Life in Weeks (Age {})", age_years),
            subtitle: format!(
                "{} of {} weeks lived ({}%) - {} remaining",
                elapsed_weeks, total_weeks, percentage, remaining
            ),
        }
    }
}

/// Get the Monday of the week containing the given date
fn week_start(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday();
    let days_since_monday = weekday.num_days_from_monday();
    date - chrono::Duration::days(days_since_monday as i64)
}

/// Add months to a date
fn add_months(date: NaiveDate, months: i32) -> NaiveDate {
    let total_months = date.month() as i32 + months;
    let years_to_add = (total_months - 1) / 12;
    let new_month = ((total_months - 1) % 12 + 1) as u32;
    let new_year = date.year() + years_to_add;

    // Handle month overflow (e.g., Jan 31 + 1 month)
    let max_day = days_in_month(new_year, new_month);
    let new_day = date.day().min(max_day);

    NaiveDate::from_ymd_opt(new_year, new_month, new_day).unwrap()
}

/// Add years to a date
fn add_years(date: NaiveDate, years: i32) -> NaiveDate {
    let new_year = date.year() + years;
    // Handle Feb 29 on non-leap years
    let max_day = days_in_month(new_year, date.month());
    let new_day = date.day().min(max_day);

    NaiveDate::from_ymd_opt(new_year, date.month(), new_day).unwrap()
}

/// Get the number of days in a month
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Check if a year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_week_start() {
        // Test with a Wednesday
        let wed = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let monday = week_start(wed);
        assert_eq!(monday.weekday(), Weekday::Mon);
        assert_eq!(monday, NaiveDate::from_ymd_opt(2024, 1, 8).unwrap());
    }

    #[test]
    fn test_life_mode() {
        let dob = NaiveDate::from_ymd_opt(1990, 1, 1).unwrap();
        let mode = Mode::Life {
            dob,
            lifespan_years: 80,
        };
        let grid = WeekGrid::calculate(&mode);

        // 80 years * 52 weeks ≈ 4160 weeks
        assert!(grid.total_weeks >= 4160 && grid.total_weeks <= 4200);
        assert_eq!(grid.columns, 52);
    }

    #[test]
    fn test_year_end_mode() {
        let grid = WeekGrid::calculate(&Mode::YearEnd);

        // Should have weeks until end of year
        assert!(grid.total_weeks > 0);
        assert!(grid.total_weeks <= 53);
    }
}
