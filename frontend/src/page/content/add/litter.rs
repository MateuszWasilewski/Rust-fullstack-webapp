use yew::{html, Html, Component, Event};
use web_sys::MouseEvent;

use common::litter::LitterData;
use crate::common::input::{get_text_value, InputForm};

pub struct AddLitter {
    new_litter: LitterData,
    respose: String
}

pub enum LitterMsg {
    SetLitterId(String),
    SetFather(String),
    SetMother(String),
    Submit,
    ResponseSuccess,
    ResponseFailure(String)
}


impl AddLitter {
    fn get_litter(&self) -> Option<LitterData> {
        if self.new_litter.id == "" {
            return None
        }
        if self.new_litter.id_mother == "" {
            return None
        }
        if self.new_litter.id_father == "" {
            return None
        }

        Some(self.new_litter.clone())
    }
}

impl Component for AddLitter {
    type Message = LitterMsg;
    type Properties= ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        AddLitter {
            new_litter: LitterData {
                id: String::new(),
                id_mother: String::new(),
                id_father: String::new()
            },
            respose: String::new()
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_update = false;
        match msg {
            LitterMsg::SetLitterId(text) => self.new_litter.id = text,
            LitterMsg::SetFather(text) => self.new_litter.id_father = text,
            LitterMsg::SetMother(text) => self.new_litter.id_mother = text,
            LitterMsg::Submit => {
                ctx.link().send_future({
                    let litter = self.get_litter().unwrap();

                    async move {
                        let response = backend_api::litter::post_litter(&litter).await;
                        match response {
                            Ok(_) => LitterMsg::ResponseSuccess,
                            Err(err) => LitterMsg::ResponseFailure(err.to_string())
                        }
                    }
                })
            },
            LitterMsg::ResponseSuccess => {
                self.respose = "Litter has been successfully inserted".into();
                should_update = true;
            },
            LitterMsg::ResponseFailure(error) => {
                self.respose = error;
                should_update = true;
            }
        }

        should_update
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let link = ctx.link();

        let on_id = link.callback(|input: Event| { LitterMsg::SetLitterId(get_text_value(input)) } );
        let on_father = link.callback(|input: Event| { LitterMsg::SetFather(get_text_value(input)) } );
        let on_mother = link.callback(|input: Event| { LitterMsg::SetMother(get_text_value(input)) });
        let submit = link.callback(|_: MouseEvent| { LitterMsg::Submit });

        html! {
            <>
            <h1>{"Dodaj Miot"}</h1>
            <InputForm action={on_id} id="id" text="Id miotu" />
            <InputForm action={on_mother} id="id_mother" text="Id matki" />
            <InputForm action={on_father} id="id_father" text="Id ojca" />
            <div class="col-auto">
                <button type="submit" onclick={submit} class="btn btn-primary mb-3">{"Dodaj miot"} </button>
            </div>
            <a>{&self.respose}</a>
            </>
        }
    }
}