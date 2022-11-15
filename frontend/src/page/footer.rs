use yew::{Component, Html, html, classes};


pub struct Footer {

}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Footer {  }
    }

    fn changed(&mut self, _ctx: &yew::Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
            <footer class={classes!("footer")} >
                <h3>{ "Footer" }</h3>
            </footer>
        }
    }
}