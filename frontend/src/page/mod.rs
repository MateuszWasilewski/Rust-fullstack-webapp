use yew::{Component, Html, html, Context, classes};

mod navbar;
mod footer;

pub struct App {
    counter: u32
}

pub enum Msg {
    Update
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App { counter: 0}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::Update => {
                self.counter += 1;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("main-page")}>
                <navbar::Navbar />
                { self.counter }

                <div class={classes!("content")}>
                    <h2>{ "More text" }</h2>

                </div>

                <footer::Footer />
            </div>
        }
    }
}