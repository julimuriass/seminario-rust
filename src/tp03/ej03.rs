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

    fn es_bisiesto (&self) -> bool {
        // A year is a leap year if:
        // 1. It is divisible by 4, and
        // 2. It is not divisible by 100, unless it is also divisible by 400.
        self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0
    }

    fn es_fecha_valida (&self) -> bool {
        let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if self.month < 1 || self.month > 12 { 
            return false;
        }

        let mut max_days = days_in_month[self.month as usize];

        if self.month == 2 && self.es_bisiesto() {
            max_days = 29;
        }

        self.day >= 1 && self.day <= max_days
    }

    fn sumar_dias (&mut self, mut days: u32) {
        let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut days_in_current_month = days_in_month[self.month as usize];

        if self.month == 2 && self.es_bisiesto() {
            days_in_current_month = 29;
        }

        //Calculate the remaining days in the current month.
        let mut remaining_days_in_month = days_in_current_month - self.day;

        while days > remaining_days_in_month {
            days -= remaining_days_in_month + 1; //+1 ensures that the current day is included when transitioning to the next month.

            self.day = 1; //Move to the next month.
            self.month += 1;

            if self.month > 12 {
                self.year += 1;
                self.month = 1;
            }

            //Recalculate the days in the current month.
            days_in_current_month = days_in_month[self.month as usize];
            if self.month == 2 && self.es_bisiesto() {
                days_in_current_month = 29;
            } 

            remaining_days_in_month = days_in_current_month - self.day;
        }
        self.day += days;  
    }

    fn restar_dias (&mut self, mut days: u32) {
        let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut days_in_current_month = days_in_month[self.month as usize];

        if self.month == 2 && self.es_bisiesto() {
            days_in_current_month = 29;
        }

        while days > self.day { //While the numbers of days I have to subtract is greater than the days I already have.
            days -= self.day + 1; //+1 ensures that the current day is included when transitioning to the previous month.

            self.month -= 1;
            if self.month < 1 {
                self.year -= 1;
                self.month = 12;
            }   

            //Recalculate the days in the current month.
            days_in_current_month = days_in_month[self.month as usize];
            if self.month == 2 && self.es_bisiesto() {
                days_in_current_month = 29;
            } 

            self.day = days_in_current_month; //Move to the end of the previous month.

        }

        self.day -= days;  
    }

    fn es_mayor (&self, other: &Date) -> bool {

        //First check the year.
        if other.year > self.year {
            return true;
        } else {
            return false;
        }

        //Check month.
        if other.month > self.month {
            return true;
        } else {
            return false;
        }

        //If months are equal, compare day.
        other.day > self.day
    }
}

//# [should_panic]
# [test]
fn tester() {
    let mut date1 = Date {
        day: 12,
        month: 5,
        year: 2006,
    };

    let mut date2 = Date::new(21, 13, 2024);


    //assert_eq!(date2.es_fecha_valida(), false);
    //assert_eq!(date1.es_bisiesto(), false);

    date1.sumar_dias(50);
    //assert_eq!(date1.month, 7); //yesss

    date1.restar_dias(50);
    println!("{}",date1.day);
    assert_eq!(date1.day, 12); //why is this failing???

    //assert_eq!(date1.es_mayor(&date2), true);  //yess
}