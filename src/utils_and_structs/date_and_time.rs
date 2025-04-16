use partial_derive::Partial;
use strum::Display;

use crate::utils_and_structs::proceed;

#[derive(Clone, Copy)]
pub struct ThreeCalendarMonths {
    pub last_month: [Date; Date::MAX_CALENDAR_DATES],
    pub this_month: [Date; Date::MAX_CALENDAR_DATES],
    pub next_month: [Date; Date::MAX_CALENDAR_DATES],

    pub currently_displayed: CalendarState,
    pub currently_displayed_month: Month,
}

impl ThreeCalendarMonths {
    pub fn sync_month_with_state(&mut self) {
        match self.currently_displayed {
            CalendarState::LastMonth => self.currently_displayed_month = self.last_month[14].month,
            CalendarState::ThisMonth => self.currently_displayed_month = self.this_month[14].month,
            CalendarState::NextMonth => self.currently_displayed_month = self.next_month[14].month,
        }
    }

    pub fn next_state(&self) -> CalendarState {
        if self.currently_displayed == CalendarState::LastMonth {
            return CalendarState::ThisMonth;
        } else if self.currently_displayed == CalendarState::ThisMonth {
            return CalendarState::NextMonth;
        } else if self.currently_displayed == CalendarState::NextMonth {
            return CalendarState::LastMonth;
        }
        self.currently_displayed
    }

    pub fn previous_state(&self) -> CalendarState {
        if self.currently_displayed == CalendarState::LastMonth {
            return CalendarState::NextMonth;
        } else if self.currently_displayed == CalendarState::ThisMonth {
            return CalendarState::LastMonth;
        } else if self.currently_displayed == CalendarState::NextMonth {
            return CalendarState::ThisMonth;
        }
        self.currently_displayed
    }
}

#[derive(Clone, Copy, PartialEq, Display)]
pub enum CalendarState {
    LastMonth,
    ThisMonth,
    NextMonth,
}

#[derive(Partial)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    pub month: Month,
    pub day: usize,
    pub year: usize,
    pub day_of_week: usize,
}

impl PartialDate {
    pub fn day_and_month(day: usize, month: Month) -> PartialDate {
        PartialDate { month: Some(month), day: Some(day), year: None, day_of_week: None }
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::UNIX_EPOCH
    }
}

impl From<PartialDate> for Date {
    fn from(value: PartialDate) -> Self {
        Self { 
            month: value.month.unwrap_or(Month::new(15, 0)), 
            day: value.day.unwrap_or_default(), 
            year: value.year.unwrap_or_default(), 
            day_of_week: value.day_of_week.unwrap_or(9),
        }
    }
}

impl Date {
    pub fn to_month_and_day(&self) -> PartialDate {
        PartialDate { month: Some(self.month), day: Some(self.day), year: None, day_of_week: None }
    }

    pub fn get_advance_by(&self, day_count: usize) -> Date {
        let mut date = self.clone();
        
        date.day_of_week += day_count % 7;
        if date.day_of_week > 7 {
            date.day_of_week -= 7;
        }
        
        let mut remaining_days = day_count;

        let mut this_month_days = date.month.days(date.year);

        while remaining_days > 0 {
            if remaining_days > this_month_days - date.day {
                remaining_days -= this_month_days - date.day;
                date.month = date.get_next_month();
                if date.month.index == 1 {date.year += 1};
                this_month_days = date.month.days(date.year);
                date.day = 0;
            } else if remaining_days <= this_month_days - date.day {
                date.day += remaining_days;
                remaining_days = 0;
            }
        }
        
        return date
    }

    pub fn advance_by(&mut self, day_count: usize) {
        *self = self.get_advance_by(day_count);
    }

    pub fn get_retrogress_by(&self, day_count: usize) -> Date {
        let mut date = self.clone();
        
        let mut day_of_week = date.day_of_week as i64;
        day_of_week -= day_count as i64 % 7;
        if day_of_week < 0 {
            date.day_of_week = (day_of_week + 7) as usize;
        } else if day_of_week == 0 {
            date.day_of_week = 7;
        } else {
            date.day_of_week = day_of_week as usize;
        }

        let mut remaining_days = day_count;

        while remaining_days > 0 {
            if remaining_days > date.day {
                date.month = date.get_last_month();
                if date.month.index == 12 {date.year -= 1};
                remaining_days -= date.day;
                date.day = date.month.days(date.year);
            } else if remaining_days == date.day {
                date.month = date.get_last_month();
                if date.month.index == 12 {date.year -= 1};
                date.day = date.month.days(date.year);
                remaining_days = 0;
            } else if remaining_days < date.day {
                date.day -= remaining_days;
                remaining_days = 0;
            }
        }

        date
    }

    pub fn retrogress_by(&mut self, day_count: usize) {
        *self = self.get_retrogress_by(day_count);
    }

    pub fn get_last_month(&self) -> Month {
        let mut last_month_index = self.month.index as i32 - 1 as i32;
        if last_month_index < 1 {
            last_month_index = 12;
        }
        Date::MONTHS[(last_month_index - 1) as usize]
    }

    pub fn get_next_month(&self) -> Month {
        let mut next_month_index = self.month.index + 1;
        if next_month_index > 12 {
            next_month_index = 1;
        }
        Date::MONTHS[(next_month_index - 1) as usize]
    }

    pub fn get_first_day_of_month(&self) -> Date {
        self.get_retrogress_by(self.day - 1)
    }

    pub fn get_first_day_of_last_month(&self) -> Date {
        self.get_retrogress_by(self.day).get_first_day_of_month()
    }

    pub fn get_first_day_of_next_month(&self) -> Date {
        let number_of_days_to_advance = self.month.days(self.year) - self.day + 1;
        self.get_advance_by(number_of_days_to_advance).get_first_day_of_month()
    }

    pub fn is_before(&self, other_date: Date) -> bool {
        if self.year < other_date.year {
            return true;
        }

        if self.month.index < other_date.month.index {
            return true;
        }

        if self.day < other_date.day {
            return true;
        }

        false
    }

    pub const MONTHS: [Month; 12] = 
    [
        Month { index: 1, days: 31 },
        Month { index: 2, days: 28 },
        Month { index: 3, days: 31 },
        Month { index: 4, days: 30 },
        Month { index: 5, days: 31 },
        Month { index: 6, days: 30 },
        Month { index: 7, days: 31 },
        Month { index: 8, days: 31 },
        Month { index: 9, days: 30 },
        Month { index: 10, days: 31 },
        Month { index: 11, days: 30 },
        Month { index: 12, days: 31 },
    ];

    pub const JAN: Month = Date::MONTHS[0];
    pub const FEB: Month = Date::MONTHS[1];
    pub const MAR: Month = Date::MONTHS[2];
    pub const APR: Month = Date::MONTHS[3];
    pub const MAY: Month = Date::MONTHS[4];
    pub const JUN: Month = Date::MONTHS[5];
    pub const JUL: Month = Date::MONTHS[6];
    pub const AUG: Month = Date::MONTHS[7];
    pub const SEP: Month = Date::MONTHS[8];
    pub const OCT: Month = Date::MONTHS[9];
    pub const NOV: Month = Date::MONTHS[10];
    pub const DEC: Month = Date::MONTHS[11];

    pub const NO_DISPLAY: Date = Date {month: Month { index: 13, days: 45 }, day: 0, year: 0, day_of_week: 8};

    pub const UNIX_EPOCH: Date = Date {month: Date::MONTHS[0], day: 1, year: 1970, day_of_week: 5}; 

    pub const MAX_CALENDAR_DATES: usize = 42;

    pub const SECONDS_IN_DAY: u64 =  86400;
    pub const SECONDS_IN_HOUR: u64 = 3600;
    pub const SECONDS_IN_MINUTE: u64 = 60;

    pub fn now() -> Date {
        let epoch = Date::UNIX_EPOCH;

        let days_seconds = 86400;
        let current_time = current_time_in_seconds();

        let days_to_advance = current_time / days_seconds;

        epoch.get_advance_by(days_to_advance as usize)
    }

    pub fn now_with_time_zone_offset(offset_in_seconds: u64) -> Date {
        let epoch = Date::UNIX_EPOCH;

        let days_seconds = 86400;
        let current_time = current_time_in_seconds();
        let current_time = (current_time - offset_in_seconds).max(0);

        let days_to_advance = current_time / days_seconds;

        epoch.get_advance_by(days_to_advance as usize)
    }

    pub fn get_day_of_week(&self) -> String {
        let day = match self.day_of_week {
            1 => "Sunday",
            2 => "Monday",
            3 => "Tuesday",
            4 => "Wednesday",
            5 => "Thursday",
            6 => "Friday",
            7 => "Saturday",
            8 => "NoDisplay",
            9 => "Unknown",
            _ => "Error",
        };

        day.to_string()
    }

    pub fn get_calendar_grid_of_month(&self) -> [Date; Date::MAX_CALENDAR_DATES] {
        let mut date = self.get_first_day_of_month();
        date.retrogress_by(date.day_of_week - 1);
        
        let mut dates = [Date::NO_DISPLAY; 42];
        for (i, _) in dates.into_iter().enumerate() {
            let working_date = date.get_advance_by(i);
            if working_date.month != self.get_last_month() && working_date.month != self.month {
                if working_date.day_of_week == 1 {
                    break;
                }
            }
            dates[i] = working_date;
        }

        dates
    }

    pub fn get_3_calendar_months(&self) -> ThreeCalendarMonths {
        let month1 = self.get_first_day_of_last_month().get_calendar_grid_of_month();
        let month2 = self.get_calendar_grid_of_month();
        let month3 = self.get_first_day_of_next_month().get_calendar_grid_of_month();

        ThreeCalendarMonths {last_month: month1, this_month: month2, next_month: month3, currently_displayed: CalendarState::ThisMonth, currently_displayed_month: self.month}
    }

    pub fn date_to_secs(&self) -> u64 {
        let mut total_days = self.day as u64 - 1;

        let mut month_index = self.month.index - 1;
        for year in (1970..=self.year).rev() {
            for i in 0..month_index {
                let month = Date::MONTHS[i];
                total_days += month.days(year) as u64;
            }
            month_index = 12;
        }

        let total_seconds = total_days * 86400;
        total_seconds
    }
}

impl ToString for Date {
    fn to_string(&self) -> String {
        if self.day == 0 {
            return "NoDisplay".to_string();
        }

        let day = self.day.to_string();

        let mut ordinal_indicator = "th";
        if day.ends_with('1') {
            ordinal_indicator = "st";
        } else if day.ends_with('2') {
            ordinal_indicator = "nd";
        } else if day.ends_with('3') {
            ordinal_indicator = "rd";
        }

        format!("{day_of_week}, {day}{ordinal_indicator} {month} {year}", day_of_week = self.get_day_of_week(), month = self.month.to_string(), year = self.year.to_string())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Month {
    index: usize,
    days: usize,
}

impl Month {
    pub fn new(month_index: usize, days_in_month: usize) -> Month {
        Month {
            index: month_index,
            days: days_in_month,
        }
    }

    pub fn days(&self, year: usize) -> usize {
        // February 
        if self.index == 2 {
            if year % 400 == 0 {
                return self.days + 1;
            } else if year % 100 == 0 {
                return self.days;
            } else if year % 4 == 0 {
                return self.days + 1;
            }
        }
        return self.days;
    }
}

impl ToString for Month {
    fn to_string(&self) -> String {
        let month = match self.index {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            13 => "NoDisplay",
            _ => "Error",
        };

        month.to_string()
    }
}

pub fn current_time_in_millis() -> u128 {
    #[cfg(not(feature = "ssr"))]
    return web_sys::js_sys::Date::now() as u128;
    #[cfg(feature = "ssr")] {
        let current_date = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => duration.as_millis(),
            Err(_) => panic!("Could Not Get Current Date"),
        };
    
        return current_date
    }
}

pub fn current_time_in_seconds() -> u64 {
    (current_time_in_millis() / 1000) as u64
}

pub fn full_iso_to_secs(iso_str: &str) -> Option<u64> {
    let date_time_split = iso_str.find('T');

    let date_time_split = match date_time_split {
        Some(index) => index,
        None => return None,
    };

    let (date, time) = iso_str.split_at(date_time_split);

    let mut time = time.to_ascii_uppercase();
    let is_utc = time.ends_with('Z');
    if !is_utc {return None;}
    
    let milli_second_index = time.find('.');
    match milli_second_index {
        Some(index) => {let _ = time.split_off(index);},
        None => proceed(),
    }
    
    let time = &time[1..];

    let time_splitter = ':';
    let date_splitter = '-';

    let time_elements = time.split(time_splitter);
    let date_elements = date.split(date_splitter);

    let mut date = Date::default();
    for (index, element) in date_elements.enumerate() {
        let Ok(element) = element.parse() else {return None};
        match index {
            0 => date.year = element,
            1 => date.month = Date::MONTHS[element - 1],
            2 => date.day = element,
            _ => return None,
        }
    }
    let mut seconds = date.date_to_secs();

    for (index, element) in time_elements.enumerate() {
        let Ok(element): Result<u64, _> = element.parse() else {return None};
        match index {
            0 => seconds += element * 3600,
            1 => seconds += element * 60,
            2 => seconds += element,
            _ => return None,
        }
    }

    Some(seconds)
}