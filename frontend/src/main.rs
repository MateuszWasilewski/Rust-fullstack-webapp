use leptos::view;

mod common;
mod page;

use page::App;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> });
}
