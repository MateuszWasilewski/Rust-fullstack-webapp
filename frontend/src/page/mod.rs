use leptos::*;
use leptos_router::Router;

mod content;
mod navigation;

pub use navigation::paths;

use content::PageContent;
use navigation::navbar::Navbar;

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view!(cx,
    <a>"Some text"</a>
      )
}

#[component]
fn AnimalList(cx: Scope) -> impl IntoView {
    view!(cx,
    <a>"Some Animal text"</a>
      )
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Navbar/>
            <PageContent/>
        </Router>
    }
}
