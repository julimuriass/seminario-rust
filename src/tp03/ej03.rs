use serde::{Serialize, Deserialize};
#[derive(Clone, Serialize, Deserialize)]
pub struct Fecha {
    pub dia: u32,
    pub mes: u32,
    pub año: u32,
}

impl Fecha {
    pub fn new (dia: u32, mes: u32, año: u32) -> Fecha {
        Fecha {
            dia,
            mes,
            año,
        }
    }

    fn es_bisiesto (&self) -> bool {
        // A year is a leap year if:
        // 1. It is divisible by 4, and
        // 2. It is not divisible by 100, unless it is also divisible by 400.
        self.año % 4 == 0 && self.año % 100 != 0 || self.año % 400 == 0
    }

    fn es_fecha_valida (&self) -> bool {
        let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if self.mes < 1 || self.mes > 12 { 
            return false;
        }

        let mut max_days = days_in_month[self.mes as usize];

        if self.mes == 2 && self.es_bisiesto() {
            max_days = 29;
        }

        self.dia >= 1 && self.dia <= max_days
    }

    pub fn sumar_dias (&mut self, mut days: u32) {
        let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut days_in_current_month = days_in_month[self.mes as usize];

        if self.mes == 2 && self.es_bisiesto() {
            days_in_current_month = 29;
        }

        //Calculate the remaining days in the current month.
        let mut remaining_days_in_month = days_in_current_month - self.dia;

        while days > remaining_days_in_month {
            days -= remaining_days_in_month + 1; //+1 ensures that the current day is included when transitioning to the next month.

            self.dia = 1; //Move to the next month.
            self.mes += 1;

            if self.mes > 12 {
                self.año += 1;
                self.mes = 1;
            }

            //Recalculate the days in the current month.
            days_in_current_month = days_in_month[self.mes as usize];
            if self.mes == 2 && self.es_bisiesto() {
                days_in_current_month = 29;
            } 

            remaining_days_in_month = days_in_current_month - self.dia;
        }
        self.dia += days;  
    }

    fn restar_dias (&mut self, mut days: u32) {
        let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut days_in_current_month = days_in_month[self.mes as usize];

        if self.mes == 2 && self.es_bisiesto() {
            days_in_current_month = 29;
        }

        while days >= self.dia { // While the number of days to subtract is greater than or equal to the current day.
            days -= self.dia; 

            self.mes -= 1;
            if self.mes < 1 {
                self.año -= 1;
                self.mes = 12;
            }   

            // Recalculate the days in the current month.
            days_in_current_month = days_in_month[self.mes as usize];
            if self.mes == 2 && self.es_bisiesto() {
                days_in_current_month = 29;
            } 

            self.dia = days_in_current_month; // Move to the end of the previous month.
        }

        self.dia -= days; // Subtract the remaining days.

    }

    pub fn es_mayor (&self, other: &Fecha) -> bool { //Retorna true si la fecha que recibí es mayor.

        //First check the year.
        if other.año < self.año {
            return true;
        } else if other.año > self.año {
            return false;
        }

        //Check month.
        if other.mes < self.mes {
            return true;
        } else if other.mes > self.mes {
            return false;
        }

        //If months are equal, compare day.
        other.dia < self.dia
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_es_fecha_valida() {
        let mut date1 = Fecha {
            dia: 12,
            mes: 5,
            año: 2006,
        };
        assert_eq!(date1.es_fecha_valida(), true);

        let mut date2 = Fecha::new(21, 13, 2024);
        assert_eq!(date2.es_fecha_valida(), false);
    }

    #[test]
    fn test_es_bisiesto() {
        let mut date1 = Fecha {
            dia: 12,
            mes: 5,
            año: 2006,
        };
        assert_eq!(date1.es_bisiesto(), false);

        let mut date2 = Fecha::new(21, 12, 2024);
        assert_eq!(date2.es_bisiesto(), true);
    }

    #[test]
    fn test_sumar_dias() {
        let mut date1 = Fecha {
            dia: 1,
            mes: 5,
            año: 2006,
        };

        date1.sumar_dias(0);
        assert_eq!(date1.dia, 1);

        date1.sumar_dias(31); //Se pasa de mes.
        assert_eq!(date1.dia, 1);
        assert_eq!(date1.mes, 6);

        date1.sumar_dias(250); //se pasa de año. (Quedaría en Feb 6 2007).
        assert_eq!(date1.dia, 6);
        assert_eq!(date1.mes, 2);
        assert_eq!(date1.año, 2007);
    }

    #[test]
    fn test_restar_dias() {
        let mut date1 = Fecha {
            dia: 10,
            mes: 5,
            año: 2006,
        };

        date1.restar_dias(5); //Solo cambia el día.
        assert_eq!(date1.dia, 5);

        date1.restar_dias(10); //Quedaría en 25/04/2006
        assert_eq!(date1.dia, 25);
        assert_eq!(date1.mes, 4);
        assert_eq!(date1.año, 2006);

        date1.restar_dias(200); //07/10/2005
        assert_eq!(date1.dia, 7);
        assert_eq!(date1.mes, 10);
        assert_eq!(date1.año, 2005);
    }

    #[test]
    fn test_es_mayor() {
        let date1= Fecha::new(12, 5, 2006);
        let date2= Fecha::new(31, 12, 2006);
        let date3= Fecha::new(1, 1, 2005);

        assert_eq!(date1.es_mayor(&date2), false);
        assert_eq!(date1.es_mayor(&date3), true);
    }


}
