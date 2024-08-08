use components::app::App;
use leptos::view;

mod components;
mod converter;

fn main() {
    leptos::mount::mount_to_body(|| view! { <App/> })
}
