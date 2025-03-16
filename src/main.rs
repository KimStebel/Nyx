mod components;
mod models;

use leptos::prelude::*;
use components::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> })
}
