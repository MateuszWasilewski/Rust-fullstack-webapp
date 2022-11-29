use yew::{html, function_component, Html};
use yew_router::BrowserRouter;

mod navbar;
mod content;
mod routes;

use navbar::Navbar;
use content::Content;
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