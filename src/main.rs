use crate::app::App;
use leptos::view;

mod app;
mod convert_arabic_to_roman_number;
mod convert_roman_to_arabic_number;

fn main() {
    leptos::mount::mount_to_body(|| view! { <App/> })
}
