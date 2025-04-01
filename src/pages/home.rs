use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_use::utils::Pausable;
use leptos_use::{use_interval_fn, use_timestamp};
use leptos::web_sys::js_sys;
use partial_derive::Partial;
use strum::Display;

use crate::components::message_box::MessageBox;
use crate::components::study_window::{StudyWindow, StudyType};
use crate::utils_and_structs::database_types::DeckId;
use crate::utils_and_structs::front_utils::get_fake_review_schedule;
use crate::utils_and_structs::shared_truth::CALENDAR_BG;
use crate::utils_and_structs::ui::{Color, Shadow};

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    let current_deck = RwSignal::new(DeckId::default());
    view! {
        <MessageBox/>
        <div class="content-flex home-container">
            <div class="multi-or-one-grid">
                <StudyWindow study_type=StudyType::Lesson/>
                <StudyWindow study_type=StudyType::Review/>
            </div>
            <div class="ka chow giver black-bg">
            </div>
            <Calendar current_deck/>
        </div>
    }
}

#[component]
pub fn Calendar(current_deck: RwSignal<DeckId>) -> impl IntoView {
    let utc_date_on_init = Date::now();
    let todays_date = RwSignal::new(utc_date_on_init);
    let dates = RwSignal::new(utc_date_on_init.get_3_calendar_months());
    let selected_date = RwSignal::new(utc_date_on_init);
    

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
            selected_date.set(current_date);
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
        let classes = format!("calendar-day {} {} {}", date.get_day_of_week(), date.day, date.month.to_string());

        let bg_color = Color::OffWhite;

        let apply_heat = move |deck_id: DeckId| {
            let (schedule, highest_reviews) = get_fake_review_schedule(deck_id);
            let review_count = *schedule.get(&date.to_month_and_day()).unwrap_or(&0);

            

            let mut box_shadow = Shadow::dark();
            if review_count > 0 {
                let fill_unit_css = "em";
                let max_fill_size = 1.2;
                let prcnt_filled = review_count as f64 / highest_reviews as f64;
                let amount_of_fill = max_fill_size - prcnt_filled * max_fill_size;


                let mut progress_shadow = Shadow::new(bg_color, 0, 0, "2px");
                progress_shadow.inset = true;
                progress_shadow.spread_radius = format!("{amount_of_fill}{fill_unit_css}");

                let mut progress_shadow_bg = Shadow::new(Color::Mint, 0, 0, 0);
                progress_shadow_bg.inset = true;
                progress_shadow_bg.spread_radius = format!("{max_fill_size}{fill_unit_css}");
                
                box_shadow.add_shadow(progress_shadow);
                box_shadow.add_shadow(progress_shadow_bg);
            }

            box_shadow.css()
        };

        let focus_month = move |dates: ThreeCalendarMonths| {
            if dates.currently_displayed_month == date.month {
                "100%"
            } else {
                "30%"
            }
        };

        let display = move |dates: ThreeCalendarMonths| {
            if date == Date::NO_DISPLAY {return "none";};
            if dates.currently_displayed == which_month {
                return "block";
            } else {
                return "none";
            }
        };

        view! {
            <li class=classes class=("selected", move || selected_date.get() == date) 
            style=("display", move || {display(dates.get())})  style=("opacity", move || {focus_month(dates.get())}) 
            style=("box-shadow", move || {apply_heat(current_deck.get())}) style:background-color=bg_color.hex() 
            on:click=move |_| selected_date.set(date)>
                {date.day.to_string()}
            </li>
        }
    };


    let last_month = {move |date: Date|  output_dates(date, CalendarState::LastMonth)};
    let this_month = {move |date: Date|  output_dates(date, CalendarState::ThisMonth)};
    let next_month = {move |date: Date|  output_dates(date, CalendarState::NextMonth)};

    let styles = 
        format!("
        .no-display {{
            display: none !important;
        }}
        .opaque {{
            opacity: 100% !important;
        }}
        .calendar-container {{
            --calendar-item-height: 2.5em;
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
            border-color: {winter2};
            border-style: solid;
            border-width: var(--border-width);
            height: var(--calendar-item-height);
            width: var(--calendar-item-height);
            border-radius: 50%;
            text-align: center;
            line-height: calc(var(--calendar-item-height) - var(--border-width) * 2);
            color: {midnight};
        }}
        .calendar-day:hover {{
            cursor: pointer;
            border-color: {winter3};
        }}
        .selected {{
            border-color: {winter4} !important;
        }}
        .calendar-label {{
            background-color: {dark_slate};
            height: var(--calendar-item-height);
            width: var(--calendar-item-height);
            border-radius: 50%;
            text-align: center;
            line-height: var(--calendar-item-height);
            font-weight: 600;
            color: {white};
        }}
        .calendar-button {{
            color: {winter3};
            text-shadow: {dark_shadow};
            font-weight: bold;
            border-radius: 50%;
            font-size: var(--calendar-item-height);
            -webkit-user-select: none;
            -moz-user-select: none;
            -ms-user-select: none;
        }}
        .calendar-button:hover {{
            background-color: rgba(var(--winter2-rgb), 0.5);
            cursor: pointer;
        }}
        ", dark_slate=Color::DarkSlate.hex(),
        white=Color::White.hex(),
        winter4=Color::Winter4.hex(),
        midnight=Color::MidnightBlack.hex(),
        winter2=Color::Winter2.hex(),
        winter3=Color::Winter3.hex(),
        dark_shadow=Shadow::dark().css(),
    );

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

    let hide_button = move |calendar_state| {
        if calendar_state == CalendarState::LastMonth {
            "hidden"
        } else {
            "block"
        }
    };

    view! {
        <style>
           {styles}
        </style>
        <div class="calendar-container">
            <div style:user-select="none" style:width="100%" style:display="flex" style:flex-direction="row" style:align-items="center" style:justify-content="space-around">
                <Icon icon={icondata::LuArrowLeftCircle} {..}  class="calendar-button" on:click=goto_last_month/>
                <h2 style=("display", move || hide_button(dates.get().currently_displayed)) style:font-size="var(--calendar-item-height)">{move || dates.get().currently_displayed_month.to_string()}</h2>
                <Icon icon={icondata::LuArrowRightCircle} {..} style=("display", move || hide_button(dates.get().currently_displayed)) class="calendar-button" on:click=goto_next_month/>
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
pub struct ThreeCalendarMonths {
    pub last_month: [Date; Date::MAX_CALENDAR_DATES],
    pub this_month: [Date; Date::MAX_CALENDAR_DATES],
    pub next_month: [Date; Date::MAX_CALENDAR_DATES],

    pub currently_displayed: CalendarState,
    pub currently_displayed_month: Month,
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
