use crate::convert_arabic_to_roman_number::convert_arabic_to_roman_number;
use crate::convert_roman_to_arabic_number::convert_roman_to_arabic_number;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (arabic, set_arabic) = signal(0);
    let (roman, set_roman) = signal(String::new());

    let on_arabic_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        let parsed_value = value.parse().unwrap_or(0);
        set_arabic(parsed_value);
        let new_roman = convert_arabic_to_roman_number(parsed_value)
            .map_err(|err| {
                error!("{}", err);
            })
            .unwrap_or("".to_string());
        set_roman(new_roman);
    };

    let on_roman_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        set_roman(value.clone());
        let new_arabic = convert_roman_to_arabic_number(value.clone())
            .map_err(|err| {
                error!("{}", err);
            })
            .unwrap_or(0);
        set_arabic(new_arabic);
    };

    view! {
        <div class="container">
            <label class="label">
                Arabic number
                <input type="number"
                    class="input"
                    on:input=on_arabic_input
                    prop:value=arabic
                    placeholder="Arabic"
                />
            </label>

            <label class="label">
                Roman number
                <input type="text"
                    class="input"
                    on:input=on_roman_input
                    prop:value=roman
                    placeholder="Roman"
                />
            </label>
        </div>
    }
}
