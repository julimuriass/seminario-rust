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

    fn sum_days (&mut self, mut days: u32) {
        let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut days_in_current_month = days_in_month[self.month as usize];

        if self.month = 2 && self.is_leap_year() {
            days_in_current_month = 29;
        }

        //Calculate the remaining days in the current month.
        let mut remaining_days_in_month = days_in_current_month - self.day;

        while days > remaining_days_in_month {
            days -= remaining_days_in_month + 1; //+1 ensures that the current day is included when transitioning to the next month.

            self.day = 1; //Move to the next month.
            self.month += 1;

            if (self.month > 12) {
                self.year += 1;
                self.month = 1;
            }

            //Recalculate the days in the current month.
            days_in_current_month = days_in_month[self.month as usize];
            if self.month = 2 && self.is_leap_year() {
                days_in_current_month = 29;
            } 

            remaining_days_in_month = days_in_current_month - self.day;
        }
        self.day += days;  
    }

    fn subtract_days (&mut self, mut days: u32) {
        let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut days_in_current_month = days_in_month[self.month as usize];

        if self.month = 2 && self.is_leap_year() {
            days_in_current_month = 29;
        }

        while days > self.days { //While the numbers of days I have to subtract is greater than the days I already have.
            days -= self.days + 1; //+1 ensures that the current day is included when transitioning to the previous month.

            self.month -= 1;
            if (self.month < 1) {
                self.year -= 1;
                self.month = 12;
            }   

            //Recalculate the days in the current month.
            days_in_current_month = days_in_month[self.month as usize];
            if self.month = 2 && self.is_leap_year() {
                days_in_current_month = 29;
            } 

            self.day = days_in_current_month; //Move to the end of the previous month.

        }

        self.day -= days;  
    }

    fn is_greater (&self, other: &Date) -> bool {

        //First check the year.
        if other.year > self.year {
            true
        } else {
            false
        }

        //Check month.
        if other.month > self.month {
            true
        } else {
            false
        }

        //If months are equal, compare day.
        other.day > self.day
    }
}