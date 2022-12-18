use common::litter::LitterData;
use wasm_bindgen::JsCast;
use yew::{html, Html, Component, Event};
use web_sys::{HtmlInputElement, MouseEvent};

pub struct AddLitter {
    id: String,
    mother: String,
    father: String,
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

fn get_text_value(input: Event) -> String {
    let target = input.target().unwrap();
    let element = target.unchecked_into::<HtmlInputElement>();
    element.value()
}

impl Component for AddLitter {
    type Message = LitterMsg;
    type Properties= ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        AddLitter {
            id: String::new(),
            mother: String::new(),
            father: String::new(),
            respose: String::new()
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_update = false;
        match msg {
            LitterMsg::SetLitterId(text) => self.id = text,
            LitterMsg::SetFather(text) => self.father = text,
            LitterMsg::SetMother(text) => self.mother = text,
            LitterMsg::Submit => {
                ctx.link().send_future({
                    let litter = LitterData {
                        id: self.id.clone(),
                        id_mother: self.mother.clone(),
                        id_father: self.father.clone()
                    };

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

        let on_id = link.callback(|input: Event| {
            LitterMsg::SetLitterId(get_text_value(input))
        } );
        
        let on_father = link.callback(|input: Event| {
            LitterMsg::SetFather(get_text_value(input))
        } );
        
        let on_mother = link.callback(|input: Event| {
            LitterMsg::SetMother(get_text_value(input))
        } );

        let submit = link.callback(|_: MouseEvent| {
            LitterMsg::Submit
        });

        html! {
            <>
            <h1>{"Dodaj Miot"}</h1>
            <div class="row g-3 align-items-center">
                <div class="col-auto">
                    <label for="ID" class="col-form-label">{"Id miotu"}</label>
                </div>
                <div class="col-auto">
                    <input type={"text"} id={"ID"} class="form-control" onchange={on_id}/>
                </div>
            </div>
            <div class="row g-3 align-items-center">
                <div class="col-auto">
                    <label for="ID" class="col-form-label">{"Id matki"}</label>
                </div>
                <div class="col-auto">
                    <input type={"text"} id={"ID"} class="form-control" onchange={on_father}/>
                </div>
            </div>
            <div class="row g-3 align-items-center">
                <div class="col-auto">
                    <label for="ID" class="col-form-label">{"Id ojca"}</label>
                </div>
                <div class="col-auto">
                    <input type={"text"} id={"ID"} class="form-control" onchange={on_mother}/>
                </div>
            </div>
            <div class="col-auto">
                <button type="submit" onclick={submit} class="btn btn-primary mb-3">{"Dodaj miot"} </button>
            </div>
            <a>{&self.respose}</a>
            </>
        }
    }
}