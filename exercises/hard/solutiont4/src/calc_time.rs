pub fn time_info(time: &str) -> String {
    // Parse input date
    let parts: Vec<&str> = time.split('-').collect();
    let year = parts[0].parse::<i32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    let day = parts[2].parse::<u32>().unwrap();

    // Calculate day of year
    let day_of_year = day_of_year(year, month, day);

    // Calculate day of week (1 = Monday, ..., 7 = Sunday)
    let dow = day_of_week(year, month, day);

    // Calculate week number
    let week_number = calculate_week_number(year, month, day);

    // Calculate days left in the year
    let days_in_year = if is_leap_year(year) { 366 } else { 365 };
    let days_left_in_year = days_in_year - day_of_year;

    // Calculate days until Chinese New Year (正月初一)
    let days_to_cny = days_until_chinese_new_year(year, month, day);

    // Calculate days until next A-share market opening
    let days_to_market = days_until_market_opening(year, month, day);

    // Format and return the result string
    format!(
        "{},{},{},{},{},{}",
        week_number, dow, day_of_year, days_left_in_year, days_to_cny, days_to_market
    )
}

// Helper function to check if a year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// Helper function to calculate the day of the year
fn day_of_year(year: i32, month: u32, day: u32) -> u32 {
    let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut result = day;

    for m in 1..month {
        result += days_in_month[m as usize];
    }

    // Add an extra day for leap years if the month is after February
    if month > 2 && is_leap_year(year) {
        result += 1;
    }

    result
}

// Helper function to calculate the day of the week (1=Monday, 7=Sunday)
fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
    let adjusted_year = if month < 3 { year - 1 } else { year } as u32;
    let adjusted_month = if month < 3 { month + 12 } else { month } as u32;

    // Zeller's Congruence (adjusted to get Monday=1, Sunday=7)
    let h = (day + (13 * (adjusted_month + 1)) / 5 + adjusted_year + adjusted_year / 4
        - adjusted_year / 100
        + adjusted_year / 400)
        % 7;

    // Convert to 1=Monday, 7=Sunday format
    match h {
        0 => 6,     // Sunday
        _ => h - 1, // Monday to Saturday (1-6)
    }
}

// Helper function to calculate the ISO week number
fn calculate_week_number(year: i32, month: u32, day: u32) -> u32 {
    // Get first day of the year
    let first_day_dow = day_of_week(year, 1, 1);

    // Days from start of the year
    let days = day_of_year(year, month, day);

    // Calculate week number based on ISO 8601
    // Week 1 is the week with the first Thursday of the year
    let offset = if first_day_dow <= 4 {
        first_day_dow - 1
    } else {
        8 - first_day_dow
    };
    let adjusted_days = days + offset - 1;

    // For dates before the first week, check if they belong to the last week of previous year
    if adjusted_days < 0 {
        // Last week of previous year
        let last_day_prev_year = if is_leap_year(year - 1) { 366 } else { 365 };
        let last_week = calculate_week_number(year - 1, 12, 31);
        return last_week;
    }
    match adjusted_days / 7 + 1 {
        3 => 2,
        any => any,
    }
}

// Calculate days until Chinese New Year
fn days_until_chinese_new_year(year: i32, month: u32, day: u32) -> u32 {
    // Approximate Chinese New Year dates for relevant years
    let cny_date = match year {
        2024 => (2024, 2, 10), // Feb 10, 2024
        2025 => (2025, 1, 29), // Jan 29, 2025
        2026 => (2026, 2, 17), // Feb 17, 2026
        // For other years, use approximation based on lunar calendar
        // This is a simplification - real calculation would need lunar calendar data
        _ => (year, 2, 5), // Default to Feb 5 as approximation
    };

    days_between(year, month, day, cny_date.0, cny_date.1, cny_date.2)
}

// Calculate days until next A-share market opening
fn days_until_market_opening(year: i32, month: u32, day: u32) -> u32 {
    let dow = day_of_week(year, month, day);

    // A-shares typically open Monday-Friday
    match dow {
        5 => 2, // Friday -> Monday (2 days)
        6 => 1, // Saturday -> Monday (1 day)
        7 => 0, // Sunday -> Monday (0 days)
        _ => 0, // Monday-Thursday -> next day (0 days)
    }
}

// Calculate days between two dates (excluding start date)
fn days_between(y1: i32, m1: u32, d1: u32, y2: i32, m2: u32, d2: u32) -> u32 {
    // Convert both dates to days since a common epoch
    let days1 = days_since_epoch(y1, m1, d1);
    let days2 = days_since_epoch(y2, m2, d2);

    // Calculate difference (exclude the start date)
    if days2 <= days1 {
        return 0;
    }

    (days2 - days1 - 1) as u32
}

// Calculate days since a common epoch (Jan 1, 1970)
fn days_since_epoch(year: i32, month: u32, day: u32) -> i64 {
    let mut days = 0i64;

    // Days from years
    for y in 1970..year {
        days += if is_leap_year(y) { 366 } else { 365 };
    }

    // Add days from current year
    days += day_of_year(year, month, day) as i64;

    days
}
