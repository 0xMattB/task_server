/*************************************************************************
    "task_server"
    date/date.rs
    10/17/23
    © Matthew Bentivegna
*************************************************************************/
use std::{
    fmt,
    str::FromStr,
    error::Error,
};
use serde::{
    Serialize,
    Deserialize
};
use chrono::{
    NaiveDate,
    NaiveTime,
    NaiveDateTime,
    Datelike
};

/*----------------------------------------------------------------------*/
#[derive(Debug, Clone, PartialEq)]
pub struct DateInvalidYear;

impl Error for DateInvalidYear {}

impl fmt::Display for DateInvalidYear {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "date: invalid year")
    }
}

/*----------------------------------------------------------------------*/
#[derive(Debug, Clone, PartialEq)]
pub struct DateInvalidMonth;

impl Error for DateInvalidMonth {}

impl fmt::Display for DateInvalidMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "date: invalid month")
    }
}

/*----------------------------------------------------------------------*/
#[derive(Debug, Clone, PartialEq)]
pub struct DateInvalidDay;

impl Error for DateInvalidDay {}

impl fmt::Display for DateInvalidDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "date: invalid day")
    }
}

/*----------------------------------------------------------------------*/
#[derive(Debug, Clone, PartialEq)]
pub struct DateParseError;

impl Error for DateParseError {}

impl fmt::Display for DateParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "date: error parsing string")
    }
}

/************************************************************************/
#[derive(Debug, Copy, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

#[allow(dead_code)]
impl Date {
    const VALID_YEAR_MIN: i32 = 1970;
    const VALID_YEAR_MAX: i32 = 2100;
    
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, Box<dyn Error>> {
        let year = Self::validate_year(year)?;
        let month = Self::validate_month(month)?;
        let day = Self::validate_day(day, month, Self::is_leap_year(year))?;
        
        Ok(
            Self {
                year,
                month,
                day
            }
        )
    }
    
    pub fn year(&self) -> i32 {
        self.year
    }
    
    pub fn month(&self) -> u32 {
        self.month
    }
    
    pub fn day(&self) -> u32 {
        self.day
    }
    
    pub fn set_year(&mut self, year: i32) -> Result<(), DateInvalidYear> {
        match Self::validate_year(year) {
            Ok(_) => {
                self.year = year;
                Ok(())
            },
            Err(e) => Err(e)
        }
    }
    
    pub fn set_month(&mut self, month: u32) -> Result<(), DateInvalidMonth> {
        match Self::validate_month(month) {
            Ok(_) => {
                self.month = month;
                Ok(())
            },
            Err(e) => Err(e)
        }
    }
    
    pub fn set_day(&mut self, day: u32, month: u32, leap: bool) -> Result<(), DateInvalidDay> {
        match Self::validate_day(day, month, leap) {
            Ok(_) => {
                self.day = day;
                Ok(())
            },
            Err(e) => Err(e)
        }
    }
    
    
    pub fn today() -> Self {
        // note: time-zones are not implemented when determining the current date:
        // https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.from_local
        
        let current_date = chrono::Utc::now();
        let year = current_date.year();
        let month = current_date.month();
        let day = current_date.day();
        
        Self {
            year,
            month,
            day,
        }
    }
    
    pub fn difference(date1: &Date, date2: &Date) -> i64 {
        let d1 = NaiveDate::from_ymd_opt(date1.year(), date1.month(), date1.day()).unwrap();
        let d2 = NaiveDate::from_ymd_opt(date2.year(), date2.month(), date2.day()).unwrap();
        
        let t1 = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let t2 = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        
        let dt1 = NaiveDateTime::new(d1, t1);
        let dt2 = NaiveDateTime::new(d2, t2);
        
        (dt1 - dt2).num_days()
    }
    
    pub fn is_today_or_later(date: &Date) -> bool {
        let today = Self::today();
        
        if Self::difference(&date, &today) >= 0 {
            true
        } else {
            false
        }
    }
    
    fn validate_year(year: i32) -> Result<i32, DateInvalidYear> {
        match year {
            Self::VALID_YEAR_MIN..=Self::VALID_YEAR_MAX => Ok(year),
            _ => Err(DateInvalidYear),
        }
    }
    
    fn validate_month(month: u32) -> Result<u32, DateInvalidMonth> {
        match month {
            1..=12 => Ok(month),
            _ => Err(DateInvalidMonth),
        }
    }
    
    fn validate_day(day: u32, month: u32, leap: bool) -> Result<u32, DateInvalidDay> {
        let days_per_month: [u32; 12] = [
            31,
            if leap { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];
        
        if Self::validate_month(month) == Ok(month) && day > 0 && day <= days_per_month[month as usize - 1] {
            Ok(day)
        } else {
            Err(DateInvalidDay)
        }
    }
    
    fn is_leap_year(year: i32) -> bool {
        if year % 400 == 0 {
            true
        } else if year % 100 == 0 {
            false
        } else if year % 4 == 0 {
            true
        } else {
            false
        }
    }
}

/*----------------------------------------------------------------------*/
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}/{:02}/{:04}", self.month(), self.day(), self.year())
    }
}

/*----------------------------------------------------------------------*/
impl FromStr for Date {
    type Err = DateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("/").collect();
        
        if parts.len() == 3 {
            let month = parts[0].parse::<u32>().map_err(|_| DateParseError)?;
            let day = parts[1].parse::<u32>().map_err(|_| DateParseError)?;
            let year = parts[2].parse::<i32>().map_err(|_| DateParseError)?;
            
            if let Ok(date) = Date::new(year, month, day) {
                return Ok(date);
            }
        }
        
        Err(DateParseError)
    }
}

/*----------------------------------------------------------------------*/
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn new_okay() {
        let date = Date::new(2023, 10, 20).unwrap();
        
        assert!(
            date.year() == 2023 &&
            date.month() == 10 &&
            date.day() == 20
        );
    }
    
    #[test]
    fn new_invalid_year() {
        assert!(
            match Date::new(0, 10, 20) {
                Ok(_) => false,
                Err(_) => true,
            }
        )
    }
    
    #[test]
    fn new_invalid_month() {
        assert!(
            match Date::new(2023, 15, 20) {
                Ok(_) => false,
                Err(_) => true,
            }
        )
    }
    
    #[test]
    fn new_invalid_day() {
        assert!(
            match Date::new(2023, 10, 35) {
                Ok(_) => false,
                Err(_) => true,
            }
        )
    }
    
    #[test]
    fn new_valid_leap_year() {
        assert!(
            match Date::new(2024, 2, 29) {
                Ok(_) => true,
                Err(_) => false,
            }
        )
    }
    
    #[test]
    fn new_invalid_leap_year() {
        assert!(
            match Date::new(2023, 2, 29) {
                Ok(_) => false,
                Err(_) => true,
            }
        )
    }
    
    #[test]
    fn to_string() {
        let date = Date::new(2004, 8, 1).unwrap();
        
        assert_eq!(date.to_string(), "08/01/2004");
    }
    
    #[test]
    fn from_string_okay() {
        let date = Date::from_str("4/12/2030").unwrap();
        
        assert!(
            date.year() == 2030 &&
            date.month() == 4 &&
            date.day() == 12
        );
    }
    
    #[test]
    fn from_string_invalid_year() {
        assert!(
            match Date::from_str("12/1/5000") {
                Ok(_) => false,
                Err(_) => true,
            }
        )
    }
    
    #[test]
    fn from_string_invalid_month() {
        assert!(
            match Date::from_str("15/1/2030") {
                Ok(_) => false,
                Err(_) => true,
            }
        )
    }
    
    #[test]
    fn from_string_invalid_day() {
        assert!(
            match Date::from_str("12/0/2030") {
                Ok(_) => false,
                Err(_) => true,
            }
        )
    }
    
    #[test]
    fn from_string_invalid() {
        assert!(
            match Date::from_str("bee/gees") {
                Ok(_) => false,
                Err(_) => true,
            }
        )
    }
    
    #[test]
    fn set_year_valid() {
        let mut date = Date::new(2023, 10, 27).unwrap();
        
        assert!(
            match date.set_year(2025) {
                Ok(_) => true,
                Err(_) => false,
            }
        )
    }
    
    #[test]
    fn set_year_invalid() {
        let mut date = Date::new(2023, 10, 27).unwrap();
        
        assert!(
            match date.set_year(8111) {
                Ok(_) => false,
                Err(_) => true,
            }
        )
    }
    
    #[test]
    fn set_month_valid() {
        let mut date = Date::new(2023, 10, 27).unwrap();
        
        assert!(
            match date.set_month(5) {
                Ok(_) => true,
                Err(_) => false,
            }
        )
    }
    
    #[test]
    fn set_month_invalid() {
        let mut date = Date::new(2023, 10, 27).unwrap();
        
        assert!(
            match date.set_month(80) {
                Ok(_) => false,
                Err(_) => true,
            }
        )
    }
    
    #[test]
    fn set_day_valid() {
        let mut date = Date::new(2023, 10, 27).unwrap();
        
        assert!(
            match date.set_day(19, 10, false) {
                Ok(_) => true,
                Err(_) => false,
            }
        )
    }
    
    #[test]
    fn set_day_invalid() {
        let mut date = Date::new(2023, 10, 27).unwrap();
        
        assert!(
            match date.set_day(50, 10, false) {
                Ok(_) => false,
                Err(_) => true,
            }
        )
    }

    #[test]
    fn difference_negative() {
        let d1 = Date::new(2023, 10, 27).unwrap();
        let d2 = Date::new(2023, 11, 19).unwrap();
        
        assert!(
            Date::difference(&d1, &d2) < 0
        )
    }

    #[test]
    fn difference_zero() {
        let d1 = Date::new(2023, 10, 27).unwrap();
        let d2 = Date::new(2023, 10, 27).unwrap();
        
        assert!(
            Date::difference(&d1, &d2) == 0
        )
    }
    
    #[test]
    fn difference_positive() {
        let d1 = Date::new(2023, 10, 27).unwrap();
        let d2 = Date::new(2023, 11, 19).unwrap();
        
        assert!(
            Date::difference(&d2, &d1) > 0
        )
    }

    #[test]
    fn today_or_later_before() {
        let date = Date::new(2020, 4, 1).unwrap();
        
        assert!(
            !Date::is_today_or_later(&date)
        )
    }

    #[test]
    fn today_or_later_now() {
        let date = Date::today();
        
        assert!(
            Date::is_today_or_later(&date)
        )
    }

    #[test]
    fn today_or_later_after() {
        let date = Date::new(2080, 12, 17).unwrap();
        
        assert!(
            Date::is_today_or_later(&date)
        )
    }
}