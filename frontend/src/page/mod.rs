use yew::{Component, Html, html, Context};

mod navbar;
mod footer;
mod content;

pub struct App {
}

pub enum Msg {
    Update,
    //GoToMouse{id: u64}
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App { }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::Update => {
                true
            }
            //Msg::GoToMouse { id } => {
            //    self.mouse_id = id;
            //    true
            //}
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <navbar::Navbar />
                
                <div class="container">
                    <content::AnimalList />
                </div>

                //<footer::Footer />
            </>
        }     
    }
}