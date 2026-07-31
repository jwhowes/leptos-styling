use leptos::{mount, prelude::*};

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Hello, world!"</h1>
    }
}

fn main() {
    mount::mount_to_body(App);
}
