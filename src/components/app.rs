use crate::converter::convert_arabic_to_roman_number::convert_arabic_to_roman_number;
use crate::converter::convert_roman_to_arabic_number::convert_roman_to_arabic_number;
use crate::converter::roman_numerals::ROMAN_NUMERALS;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (arabic, set_arabic) = signal(0);
    let (roman, set_roman) = signal(String::new());

    let update_roman = move |value: i32| {
        let new_roman = convert_arabic_to_roman_number(value)
            .map_err(|err| {
                error!("{}", err);
            })
            .unwrap_or("".to_string());
        set_roman(new_roman);
    };

    let update_arabic = move |value: String| {
        let new_arabic = convert_roman_to_arabic_number(value)
            .map_err(|err| {
                error!("{}", err);
            })
            .unwrap_or(0);
        set_arabic(new_arabic);
        new_arabic
    };

    let on_arabic_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev).parse().unwrap_or(0);
        set_arabic(value);
        update_roman(value);
    };

    let on_roman_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        update_roman(update_arabic(value));
    };

    let table_content = ROMAN_NUMERALS
        .into_iter()
        .map(|n| {
            view! { <tr>
                <td>{n.numeral}</td>
                <td>{n.value}</td>
            </tr>}
        })
        .collect_view();

    view! {
        <div class="container">
            <h1>Roman to arabic number converter</h1>

            <label class="label">
                Arabic number
                <input type="number"
                    class="input"
                    on:input=on_arabic_input
                    prop:value=arabic
                    placeholder="Arabic number"
                />
            </label>

            <label class="label">
                Roman number
                <input type="text"
                    class="input"
                    on:input=on_roman_input
                    prop:value=roman
                    placeholder="Roman number"
                />
            </label>

            <h2>List of valid roman numbers</h2>
            <table>
            <thead>
                <tr>
                    <th>Roman</th>
                    <th>Arabic</th>
                </tr>
            </thead>
            <tbody>
                {table_content}
            </tbody>
            </table>
        </div>
    }
}
