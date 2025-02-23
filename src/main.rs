use leptos::prelude::*;
use leptos::web_sys::console;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    Effect::new(move |_| {
        let current_value = count.get();
        console::log_1(&format!("count changed: {}", current_value).into());
    });

    view! {
        <button
            class:red=move || count.get() % 2 == 1
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
        <ul>
            <li contenteditable="true">"take the trash out"</li>
            <li>"work out"</li>
        </ul>
    }
}
