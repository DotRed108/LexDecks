use leptos::either::Either;
use leptos::{prelude::*, svg};
use leptos::web_sys::js_sys;
use leptos_icons::Icon;
use leptos_use::{use_interval_fn, utils::Pausable};

use crate::utils_and_structs::date_and_time::ThreeCalendarMonths;
use crate::utils_and_structs::front_utils::get_fake_review_schedule;
use crate::utils_and_structs::{shared_truth::CALENDAR_BG, database_types::DeckId, date_and_time::{CalendarState, Date}, ui::{Color, Shadow}};

#[component]
pub fn Calendar(current_deck: RwSignal<DeckId>) -> impl IntoView {
    let utc_date_on_init = Date::now();
    let todays_date = RwSignal::new(utc_date_on_init);
    let dates = RwSignal::new(utc_date_on_init.get_3_calendar_months());
    let selected_date = RwSignal::new(utc_date_on_init);
    let item_height = 2.5;
    

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
        view! {
            <CalendarItem dates item_type=CalendarItemType::Day(date, selected_date, current_deck, which_month, item_height)/>
        }
    };

    let create_days_of_week = move |day_of_week: u32| {
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
            <CalendarItem dates item_type=CalendarItemType::Label(heading)/>
        }
    };


    let last_month = {move |date: Date|  output_dates(date, CalendarState::LastMonth)};
    let this_month = {move |date: Date|  output_dates(date, CalendarState::ThisMonth)};
    let next_month = {move |date: Date|  output_dates(date, CalendarState::NextMonth)};

    let mut styles = format!(
        ".calendar-container {{
            --calendar-item-height: {item_height}em;
            --calendar-padding: 2.5ch;
            background-color: transparent;
            display: flex;
            flex-direction: column;
            align-items: center;
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
        }}\n", winter3 = Color::Winter3.hex(), dark_shadow=Shadow::dark().css()
    );
    let item_styles = 
        format!("
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
        }}\n", 
        dark_slate=Color::DarkSlate.hex(),
        white=Color::White.hex(),
        winter4=Color::Winter4.hex(),
        midnight=Color::MidnightBlack.hex(),
        winter2=Color::Winter2.hex(),
        winter3=Color::Winter3.hex(),
    );
    styles.push_str(&item_styles);

    let left_icon: NodeRef<svg::Svg> = NodeRef::new();
    let right_icon: NodeRef<svg::Svg> = NodeRef::new();

    let hide_button = move |calendar_state| {
        if calendar_state == CalendarState::LastMonth {
            let _ = left_icon.get().unwrap().set_attribute("visibility", "hidden");
        } else if calendar_state == CalendarState::NextMonth {
            let _ = right_icon.get().unwrap().set_attribute("visibility", "hidden");
        } else {
            let _ = left_icon.get().unwrap().remove_attribute("visibility");
            let _ = right_icon.get().unwrap().remove_attribute("visibility");
        }
    };

    let goto_next_month = move |_| {
        let mut current_3_calendar = dates.get();
        current_3_calendar.currently_displayed = current_3_calendar.next_state();
        current_3_calendar.sync_month_with_state();
        dates.set(current_3_calendar);
        hide_button(current_3_calendar.currently_displayed);
    };

    let goto_last_month = move |_| {
        let mut current_3_calendar = dates.get();
        current_3_calendar.currently_displayed = current_3_calendar.previous_state();
        current_3_calendar.sync_month_with_state();
        dates.set(current_3_calendar);
        hide_button(current_3_calendar.currently_displayed);
    };

    let get_selected_date_review_count = move |selected_date: Date, todays_date: Date, deck_id: DeckId| {
        let (schedule, _) = get_fake_review_schedule(deck_id);
        let reviews_on_selected_date = *schedule.get(&selected_date.to_month_and_day()).unwrap_or(&0);

        let mut what_day = "that day";
        let tense = if selected_date.is_before(todays_date) {
            "completed"
        } else if selected_date == todays_date {
            what_day = "today";
            "have"
        } else {
            "have"
        };

        let review_count = if reviews_on_selected_date == 0 {
            "no".to_string()
        } else {
            reviews_on_selected_date.to_string()
        };

        format!("You {tense} {review_count} reviews {what_day}.")
    };

    view! {
        <style>
           {styles}
        </style>
        <div class="calendar-container">
            <div style:user-select="none" style:width="100%" style:display="flex" style:flex-direction="row" style:align-items="center" style:justify-content="space-around">
                <Icon icon={icondata::LuArrowLeftCircle} {..}  class="calendar-button" node_ref=left_icon on:click=goto_last_month/>
                <h2 style:font-size="var(--calendar-item-height)">{move || dates.get().currently_displayed_month.to_string()}</h2>
                <Icon icon={icondata::LuArrowRightCircle} {..} class="calendar-button" node_ref=right_icon on:click=goto_next_month/>
            </div>
            <ol class="calendar">
                {move || (1..=7_u32).map(create_days_of_week).into_iter().collect::<Vec<_>>()}
                {move || dates.get().last_month.map(last_month).into_iter().collect::<Vec<_>>()}
                {move || dates.get().this_month.map(this_month).into_iter().collect::<Vec<_>>()}
                {move || dates.get().next_month.map(next_month).into_iter().collect::<Vec<_>>()}
            </ol>
            <div>
                {move || get_selected_date_review_count(selected_date.get(), todays_date.get(), current_deck.get())}
            </div>
        </div>
    }
}

#[component]
pub fn CalendarItem(dates: RwSignal<ThreeCalendarMonths>, item_type: CalendarItemType) -> impl IntoView {
    let classes = |date: Date| format!("calendar-day {} {} {}", date.get_day_of_week(), date.day, date.month.to_string());
    let bg_color = Color::OffWhite;

    let apply_heat = move |deck_id: DeckId, date: Date, item_height: f64| {
        let (schedule, highest_reviews) = get_fake_review_schedule(deck_id);
        let review_count = *schedule.get(&date.to_month_and_day()).unwrap_or(&0);

        let mut box_shadow = Shadow::dark();
        if review_count > 0 {
            let fill_unit_css = "em";
            let max_fill_size = item_height / 2.0;
            let fill_floor = max_fill_size * 0.1;
            let max_fill_size = max_fill_size - fill_floor;
            let prcnt_filled = review_count as f64 / highest_reviews as f64;
            let amount_of_fill = max_fill_size - fill_floor - prcnt_filled * max_fill_size;


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

    let focus_month = move |dates: ThreeCalendarMonths, date: Date| {
        if dates.currently_displayed_month == date.month {
            "100%"
        } else {
            "30%"
        }
    };

    let display = move |dates: ThreeCalendarMonths, which_month: CalendarState, date: Date| {
        if date == Date::NO_DISPLAY {return "none";};
        if dates.currently_displayed == which_month {
            return "block";
        } else {
            return "none";
        }
    };

    view! {
        {match item_type {
            CalendarItemType::Label(heading) => Either::Left(view! {<li class="calendar-label">{heading}</li>}),
            CalendarItemType::Day(date, selected_date, current_deck, which_month, item_height) => Either::Right(view! {
                <li class=classes(date) class=("selected", move || selected_date.get() == date) 
                style=("display", move || {display(dates.get(), which_month, date)})  style=("opacity", move || {focus_month(dates.get(), date)}) 
                style=("box-shadow", move || {apply_heat(current_deck.get(), date, item_height)}) style:background-color=bg_color.hex() 
                on:click=move |_| selected_date.set(date)>
                    {date.day.to_string()}
                </li>
            })
        }}
    }
}

#[derive(Clone)]
pub enum CalendarItemType {
    Label(&'static str),
    Day(Date /*date which item represents on calendar*/, RwSignal<Date> /*Calendar date selected*/, RwSignal<DeckId> /*id of deck selected*/, CalendarState /*which calendar block this item belongs in*/, f64 /*item_height*/),
}