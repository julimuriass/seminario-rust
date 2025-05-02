struct Date {
    day: u32,
    month: u32,
    year: u32,
}

impl Date {

    fn new (day: u32, month: u32, year: u32) -> Date {
        Date {
            day,
            month,
            year,
        }
    }

    fn is_leap_year (&self) -> bool {
        // A year is a leap year if:
        // 1. It is divisible by 4, and
        // 2. It is not divisible by 100, unless it is also divisible by 400.
        ((self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0)
    }

    fn is_valid_date (&self) -> bool {
        let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if self.month < 1 || self.month > 12 { 
            false
        }

        let mut max_days = days_in_month[self.month as usize];

        if self.month == 2 && self.is_leap_year {
            max_days = 29;
        }

        self.day >= 1 && self.day <= max_days
    }

    fn sum_days (&self, days: u32) {
        
    }
}