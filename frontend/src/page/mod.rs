use leptos::*;
use leptos_router::Router;

mod content;
mod navigation;

pub use navigation::paths;

use content::PageContent;
use navigation::navbar::Navbar;

#[component]
fn Home() -> impl IntoView {
    view!(
    <a>"Some text"</a>
      )
}

#[component]
fn AnimalList() -> impl IntoView {
    view!(
    <a>"Some Animal text"</a>
      )
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Navbar/>
            <PageContent/>
        </Router>
    }
}
