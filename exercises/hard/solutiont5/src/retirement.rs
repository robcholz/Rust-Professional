pub fn retire_time(time: &str, tp: &str) -> String {
    // Parse the input date (birth date)
    let parts: Vec<&str> = time.split('-').collect();
    let birth_year: u32 = parts[0].parse().unwrap();
    let birth_month: u32 = parts[1].parse().unwrap();

    // Define base retirement age based on the person type
    let (mut base_retirement_age, mut delay_months_per_year, mut max_retirement_age) = match tp {
        "男职工" => (60, 3, 63),                            // Male workers: retirement at 60, delayed to 63
        "原法定退休年龄55周岁女职工" => (55, 3, 58),             // Female workers with 55 years retirement age
        "原法定退休年龄50周岁女职工" => (50, 6, 55),             // Female workers with 50 years retirement age
        _ => panic!("Unknown person type"),
    };

    // Starting year for delay retirement policy (2025)
    let delay_start_year = 2025;
    let mut total_delay_months = 0;
    let mut current_age = base_retirement_age;

    // Check if the person is subject to the delay policy
    if birth_year + base_retirement_age >= delay_start_year {
        while current_age < max_retirement_age {
            total_delay_months += delay_months_per_year;
            current_age += 1;
        }
    }

    // Calculate the retirement date year and month
    let mut retirement_year = birth_year + base_retirement_age + (total_delay_months / 12);
    let mut retirement_month = birth_month + (total_delay_months % 12);

    // Adjust the retirement year and month if necessary (handling month overflow)
    if retirement_month > 12 {
        retirement_year += retirement_month / 12;
        retirement_month = retirement_month % 12;
    }

    // Calculate retirement age and convert to 2 decimal places
    let retirement_age = base_retirement_age as f32 + total_delay_months as f32 / 12.0;

    // Return the result in the required format
    format!(
        "{:04}-{:02}, {:.2}, {}",
        retirement_year,
        retirement_month,
        retirement_age,
        total_delay_months
    )
}