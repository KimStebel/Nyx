use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    let x: u8 = "5".parse().unwrap();
    mount_to_body(|| {
        view! {
            <p>"Hello, world!"</p>
            <p>x</p>
        }
    })
}
