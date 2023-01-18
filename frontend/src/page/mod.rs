use yew::{function_component, html, Html};
use yew_router::BrowserRouter;

mod content;
mod navbar;

use content::Content;
use navbar::Navbar;

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
