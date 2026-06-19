use std::time::SystemTime;

pub struct ClockService;

impl ClockService {
    pub fn format_time(format: &str) -> String {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        let secs = now.as_secs();
        Self::format_from_epoch(secs, format)
    }

    pub fn format_from_epoch(secs: u64, format: &str) -> String {
        let days = (secs / 86400) as i64;
        let time_of_day = secs % 86400;
        let hours = (time_of_day / 3600) as u32;
        let minutes = ((time_of_day % 3600) / 60) as u32;
        let seconds = (time_of_day % 60) as u32;

        // Simple date calculation (approximation)
        let mut year = 1970;
        let mut remaining_days = days;
        loop {
            let days_in_year = if Self::is_leap_year(year) { 366 } else { 365 };
            if remaining_days < days_in_year {
                break;
            }
            remaining_days -= days_in_year;
            year += 1;
        }

        let mut month = 1u32;
        let mut day = remaining_days as u32 + 1;
        let days_in_months = if Self::is_leap_year(year) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };
        for (i, &dim) in days_in_months.iter().enumerate() {
            if day <= dim {
                month = (i + 1) as u32;
                break;
            }
            day -= dim;
        }

        let day_of_week = Self::day_of_week(year, month, day);
        let day_names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        let month_names = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun",
            "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];

        let mut result = String::new();
        let mut chars = format.chars();
        while let Some(c) = chars.next() {
            match c {
                '%' => {
                    if let Some(f) = chars.next() {
                        match f {
                            'H' => result.push_str(&format!("{:02}", hours)),
                            'M' => result.push_str(&format!("{:02}", minutes)),
                            'S' => result.push_str(&format!("{:02}", seconds)),
                            'I' => {
                                let h12 = if hours == 0 { 12 } else if hours > 12 { hours - 12 } else { hours };
                                result.push_str(&format!("{:02}", h12));
                            }
                            'p' => {
                                let ampm = if hours < 12 { "AM" } else { "PM" };
                                result.push_str(ampm);
                            }
                            'Y' => result.push_str(&format!("{:04}", year)),
                            'm' => result.push_str(&format!("{:02}", month)),
                            'd' => result.push_str(&format!("{:02}", day)),
                            'a' => result.push_str(day_names[(day_of_week) as usize]),
                            'b' => result.push_str(month_names[(month - 1) as usize]),
                            'A' => {
                                let full_days = [
                                    "Monday", "Tuesday", "Wednesday", "Thursday",
                                    "Friday", "Saturday", "Sunday",
                                ];
                                result.push_str(full_days[day_of_week as usize]);
                            }
                            'B' => {
                                let full_months = [
                                    "January", "February", "March", "April", "May", "June",
                                    "July", "August", "September", "October", "November", "December",
                                ];
                                result.push_str(full_months[(month - 1) as usize]);
                            }
                            '%' => result.push('%'),
                            other => {
                                result.push('%');
                                result.push(other);
                            }
                        }
                    }
                }
                _ => result.push(c),
            }
        }
        result
    }

    fn is_leap_year(year: i64) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    fn day_of_week(year: i64, month: u32, day: u32) -> u32 {
        let m = month as i64;
        let d = day as i64;
        let y = year;
        let adjusted_month = if m < 3 { m + 12 } else { m };
        let adjusted_year = if m < 3 { y - 1 } else { y };
        let h = (d
            + ((13 * (adjusted_month + 1)) / 5)
            + adjusted_year
            + (adjusted_year / 4)
            - (adjusted_year / 100)
            + (adjusted_year / 400))
            % 7;
        // Monday=0, Sunday=6
        ((h + 5) % 7) as u32
    }

    pub fn current_hour() -> u32 {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        ((now.as_secs() % 86400) / 3600) as u32
    }

    pub fn current_minute() -> u32 {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        ((now.as_secs() % 3600) / 60) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        let result = ClockService::format_from_epoch(1700000000, "%H:%M:%S");
        assert_eq!(result.len(), 8);
        assert!(result.contains(':'));
    }

    #[test]
    fn test_format_date() {
        let result = ClockService::format_from_epoch(1700000000, "%Y-%m-%d");
        assert!(result.starts_with("20"));
    }

    #[test]
    fn test_leap_year() {
        assert!(ClockService::is_leap_year(2024));
        assert!(!ClockService::is_leap_year(2023));
        assert!(ClockService::is_leap_year(2000));
        assert!(!ClockService::is_leap_year(1900));
    }
}
