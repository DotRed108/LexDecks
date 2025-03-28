use leptos::prelude::*;
use leptos_icons::Icon;
use icondata;

#[component]
pub fn MessageBox() -> impl IntoView {
    //signals and resources
    let subject = RwSignal::new("Welcome to LexDecks! Here are your reviews and lessons for the day.".to_string());
    let urgent = RwSignal::new(false);
    let message = RwSignal::new("gay".to_string());
    let display_message = RwSignal::new(false);

    //styles
    let styles: &str = "
        :root {
            --mb-icon-width: 4ch;
            --mb-icon-padding: 0.75ch;
            --mb-left-padding: max(1.5vw, 1.5ch);
            --mb-right-padding: calc(var(--mb-left-padding) + var(--mb-icon-width) + var(--mb-icon-padding) + 1%);
            --mb-top-padding: 2.5vw;
            --mb-bottom-padding: var(--mb-top-padding);
        }
        .message-box {
            box-shadow: var(--box-shadow-light);
            color: var(--default-text-color);
            text-shadow: var(--text-shadow-dark);
            margin-top: var(--default-div-margin);
            position: relative;
            border-radius: var(--default-border-radius);
            padding-top: var(--mb-top-padding);
            padding-bottom: var(--mb-bottom-padding);
            padding-right: var(--mb-right-padding);
            padding-left: var(--mb-left-padding);
        }
        .message-box-icon {
            color: var(--default-text-color);
            text-shadow: var(--text-shadow-dark);
            position: absolute;
            top: 50%; 
            right: var(--mb-left-padding); 
            transform: translateY(-50%);
            padding: var(--mb-icon-padding);
            font-weight: bold;
            font-size: 2ch;
            border-radius: var(--default-border-radius);
            height: var(--mb-icon-width);
            width: var(--mb-icon-width);
        }
        .message-box-icon:hover {
            background-color: rgba(var(--winter2-rgb), 0.5);
            cursor: pointer;
        }
        .message-box {
            background-color: var(--mint);
        }
        .urgent-message {
            background-color: var(--red);
        }
        .urgent-message .message-box-icon:hover {
            background-color: rgba(var(--midnight-black-rgb), 0.5) !important;
        }
        .flip-icon {
            transform: rotate(0.5turn) translateY(50%);
        }
        .wabba {
            --wabba-top-pad: calc(var(--mb-top-padding)/3);
            box-shadow: var(--box-shadow-light);
            color: var(--default-text-color);
            text-shadow: var(--text-shadow-dark);
            background-color: var(--mint);
            position: relative;
            width: inherit;
            padding-top: var(--wabba-top-pad);
            padding-bottom: var(--mb-bottom-padding);
            padding-left: var(--mb-left-padding);
            border-top-left-radius: 0;
            border-top-right-radius: 0;
            border-bottom-left-radius: var(--default-border-radius);
            border-bottom-right-radius: var(--default-border-radius);
        }
        .wabba::before {
            --this-height: 1em;
            border-bottom: thick double var(--white);
            content: \" \";
            position: absolute;
            top: 0;
            left: 0;
            width: calc(100% - var(--mb-right-padding));
            height: var(--this-height);
            transform: translateY(calc(-1 * (var(--this-height) + (var(--mb-bottom-padding) / 4))));
        }
        .wabba-merge {
            border-bottom-right-radius: 0;
            border-bottom-left-radius: 0;
        }
    ";

    // closures
    let is_urgent = move || {urgent.get()};
    let toggle_message = move |_| {display_message.set(!display_message.get())};
    let flip_icon = move || {
        match display_message.get() {
        true => "message-box-icon flip-icon",
        false => "message-box-icon",
    }};

    view! {
        <style>
        {styles}
        </style>
        <Show when=move || { !subject.get().is_empty() }>
            <div class="message-box" class:wabba-merge={move || display_message.get()} class:urgent-message=is_urgent>
                <p>{subject.get()}</p>
                <Show when=move || { !message.get().is_empty() }>
                    <Icon icon={icondata::LuArrowDownFromLine} {..} class=flip_icon on:click=toggle_message/>
                </Show>
            </div>
            <Show when=move || { display_message.get() }>
                <div class="wabba">{message.get()}</div>
            </Show>
        </Show>
    }
}