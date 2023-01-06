use yew::{function_component, html, Html};
use yew_router::BrowserRouter;

mod content;
mod navbar;
mod routes;

use content::Content;
use navbar::Navbar;
pub use routes::Routes;

#[function_component(App)]
pub fn get_app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Navbar />
                <Content />
            </BrowserRouter>
        </>
    }
}
