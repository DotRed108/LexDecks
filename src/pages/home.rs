use leptos::prelude::*;
use leptos_use::utils::Pausable;
use leptos_use::{use_interval_fn, use_timestamp};
use leptos::web_sys::js_sys;

use crate::components::message_box::MessageBox;
use crate::components::study_window::{StudyWindow, StudyType};
use crate::utils_and_structs::shared_truth::CALENDAR_BG;
use crate::utils_and_structs::ui::Color;

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <MessageBox/>
        <div class="content-flex home-container">
            <div class="multi-or-one-grid">
                <StudyWindow study_type=StudyType::Lesson/>
                <StudyWindow study_type=StudyType::Review/>
            </div>
            <div class="ka chow giver black-bg">
            </div>
            <Calendar/>
        </div>
    }
}

#[component]
pub fn Calendar() -> impl IntoView {
    let utc_date_on_init = Date::now();
    let todays_date = RwSignal::new(utc_date_on_init);
    let dates = RwSignal::new(utc_date_on_init.get_3_calendar_months());
    
    let call_effect = RwSignal::new(0);
    Effect::new(move |last_run| {
        call_effect.get();
        let date_on_last_run = match last_run {
            Some(date_on_last_run) => date_on_last_run,
            None => utc_date_on_init,
        };

        let tz_offset = js_sys::Date::new_0().get_timezone_offset();
        let offset_in_seconds = (tz_offset * 60.0) as u64;
        let current_date = Date::now_with_time_zone_offset(offset_in_seconds);

        if date_on_last_run != current_date {
            todays_date.set(current_date);
        }
        if date_on_last_run.month != current_date.month {
            dates.set(current_date.get_3_calendar_months());
        }

        current_date
    });

    #[allow(unused)]
    let Pausable {pause, resume, is_active} = use_interval_fn(move || 
    {
        if call_effect.get_untracked() == 0 {
            call_effect.update_untracked(|k: &mut i32| {*k = 1});
        } else {
            call_effect.set(call_effect.get() + 1)
        }
    }, 300000);
    
    let output_dates = move |date: Date, which_month: CalendarState| {
        let display = if date == Date::NO_DISPLAY {"none"} else {"inherit"};
        let classes = format!("calendar-day {} {} {}", date.day, date.day_of_week, date.month.to_string());
        
        let dont_display = move || {
            if dates.get().currently_displayed == which_month {
                false
            } else {
                true
            }
        };

        let focus_month = move || {
            if dates.get().currently_displayed_month == date.month {
                true
            } else {
                false
            }
        };
        
        view! {
            <li class=classes style:display=display class:no-display=dont_display style:opacity="30%" class:opaque=focus_month>
                {date.day.to_string()}
            </li>
        }
    };


    let last_month = {move |date: Date|  output_dates(date, CalendarState::LastMonth)};
    let this_month = {move |date: Date|  output_dates(date, CalendarState::ThisMonth)};
    let next_month = {move |date: Date|  output_dates(date, CalendarState::NextMonth)};

    let write_styles = move || {
        let three_months = dates.get();

        format!("
        .no-display {{
            display: none !important;
        }}
        .opaque {{
            opacity: 100% !important;
        }}
        .calendar-container {{
            --calendar-padding: 2.5ch;
            background-color: transparent;
            display: flex;
            flex-direction: column;
            align-items: center;
            padding-bottom: var(--calendar-padding);
            padding-left: var(--calendar-padding);
            padding-right: var(--calendar-padding);
            gap: var(--calendar-padding);
            background-image: url({CALENDAR_BG});
            background-size: 100%;
        }}
        .calendar {{
            --calendar-item-height: 2.5em;
            width: 100%;
            background-color: transparent;
            display: grid;
            grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
            row-gap: calc(var(--calendar-padding)/2);
            list-style-type: none;
            justify-items: center;
        }}
        .calendar-day {{
            --border-width: 2px;
            background-color: {clndr_day_color};
            border-color: {cldr_day_border_color};
            border-style: solid;
            border-width: var(--border-width);
            height: var(--calendar-item-height);
            width: var(--calendar-item-height);
            border-radius: 50%;
            text-align: center;
            line-height: calc(var(--calendar-item-height) - var(--border-width) * 2);
            color: {clndr_day_text_color};
        }}
        .calendar-label {{
            background-color: {clndr_label_color};
            height: var(--calendar-item-height);
            width: var(--calendar-item-height);
            border-radius: 50%;
            text-align: center;
            line-height: var(--calendar-item-height);
            font-weight: 600;
            color: {label_color};
        }}
        ", clndr_day_color = Color::OffWhite.hex(),
        clndr_label_color=Color::DarkSlate.hex(),
        label_color=Color::White.hex(),
        cldr_day_border_color=Color::Winter2.hex(),
        clndr_day_text_color=Color::MidnightBlack.hex(),
    )};

    let create_days_of_week = |day_of_week: u32| {
        let heading = match day_of_week {
            1 => "Su",
            2 => "Mo",
            3 => "Tu",
            4 => "We",
            5 => "Th",
            6 => "Fr",
            7 => "Sa",
            8 => "ND",
            _ => "Err",
        };
        view! {
            <li class="calendar-label">
                {heading}
            </li>
        }
    };

    let goto_next_month = move |_| {
        let mut current_3_calendar = dates.get();
        current_3_calendar.currently_displayed = current_3_calendar.next_state();
        current_3_calendar.sync_month_with_state();
        dates.set(current_3_calendar);
    };

    let goto_last_month = move |_| {
        let mut current_3_calendar = dates.get();
        current_3_calendar.currently_displayed = current_3_calendar.previous_state();
        current_3_calendar.sync_month_with_state();
        dates.set(current_3_calendar);
    };

    view! {
        <style>
           {move || write_styles}
        </style>
        <div class="calendar-container">
            <div style:display="flex" style:flex-direction="row" style:align-items="center">
                <div on:click=goto_last_month class="previous_month">"previous"</div>
                <h2 style:font-size="2.2em">{move || dates.get().currently_displayed_month.to_string()}</h2>
                <div on:click=goto_next_month class="future_month">"future"</div>
            </div>
            <ol class="calendar">
                {move || (1..=7_u32).map(create_days_of_week).into_iter().collect::<Vec<_>>()}
                {move || dates.get().last_month.map(last_month).into_iter().collect::<Vec<_>>()}
                {move || dates.get().this_month.map(this_month).into_iter().collect::<Vec<_>>()}
                {move || dates.get().next_month.map(next_month).into_iter().collect::<Vec<_>>()}
            </ol>
        </div>
    }
}

#[derive(Clone, Copy)]
struct ThreeCalendarMonths {
    last_month: [Date; Date::MAX_CALENDAR_DATES],
    this_month: [Date; Date::MAX_CALENDAR_DATES],
    next_month: [Date; Date::MAX_CALENDAR_DATES],

    currently_displayed: CalendarState,
    currently_displayed_month: Month,
}

impl ThreeCalendarMonths {
    fn sync_month_with_state(&mut self) {
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

#[derive(Clone, Copy, PartialEq)]
enum CalendarState {
    LastMonth,
    ThisMonth,
    NextMonth,
}

#[derive(Clone, Copy, PartialEq)]
struct Date {
    month: Month,
    day: usize,
    year: usize,
    day_of_week: usize,
}

impl Date {
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

    const MONTHS: [Month; 12] = 
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

    const NO_DISPLAY: Date = Date {month: Month { index: 13, days: 45 }, day: 0, year: 0, day_of_week: 8};

    const UNIX_EPOCH: Date = Date {month: Date::MONTHS[0], day: 1, year: 1970, day_of_week: 5}; 

    const MAX_CALENDAR_DATES: usize = 42;

    pub fn now() -> Date {
        let epoch = Date::UNIX_EPOCH;

        let days_seconds = 86400;
        let current_time = use_timestamp().get_untracked() as u64 / 1000;

        let days_to_advance = current_time / days_seconds;

        epoch.get_advance_by(days_to_advance as usize)
    }

    pub fn now_with_time_zone_offset(offset_in_seconds: u64) -> Date {
        let epoch = Date::UNIX_EPOCH;

        let days_seconds = 86400;
        let current_time = use_timestamp().get_untracked() as u64 / 1000;
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

        ThreeCalendarMonths {last_month: month1, this_month: month2, next_month: month3, currently_displayed: CalendarState::ThisMonth,currently_displayed_month: self.month}
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

#[derive(Clone, Copy, PartialEq)]
struct Month {
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
