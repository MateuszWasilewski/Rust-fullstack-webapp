use yew::{Component, Html, html, Context, classes};
use super::super::log;

pub struct Navbar {
}

pub enum Msg {
    PlusOne
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Navbar { }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PlusOne => {
                match ctx.link().get_parent() {
                    Some(parent) => {
                        let parent_link = parent.clone();
                        parent_link.downcast::<super::App>().send_message(super::Msg::Update);
                    }
                    None => log("Navbar has no parent component")
                }
            }
        }
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let onclick = _ctx.link().callback(|_| Msg::PlusOne);

        html! {
            <div onclick={onclick} class={classes!("header")} >
                <h3>{ "Baza danych myszy" }</h3>
            </div>
        }
    }
}