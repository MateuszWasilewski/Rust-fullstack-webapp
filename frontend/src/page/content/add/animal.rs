use yew::{Component, html, Event, MouseEvent, Callback};

use crate::common::input::{get_text_value, InputForm, DropdownForm};
use common::animal::{AnimalData, Gender};

pub struct AddAnimal {
    id: Option<String>,
    litter_id: Option<String>,
    gender: Option<Gender>,
    status: Option<String>,
    phenotype: Option<String>,
    litter_list: Vec<String>,
    phenotype_list: Vec<String>,
    response: String,
}

pub enum Msg {
    SetId(Option<String>),
    SetLitter(Option<String>),
    SetPhenotype(Option<String>),
    SetStatus(Option<String>),
    ReceiveLitterList(Vec<String>),
    ReceivePhenotypeList(Vec<String>),
    SetGender(Option<Gender>),
    Submit,
    Response(String)
}

impl Component for AddAnimal {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future(async move {
            let litters = backend_api::get_litter_list().await.unwrap_or(vec![]);
            let litters: Vec<String> = litters.into_iter().map(|litter| {
                litter.id
            }).collect();
            Msg::ReceiveLitterList(litters)
        });
        ctx.link().send_future(async move {
            let litters = backend_api::get_phenotypes().await.unwrap_or(vec![]);
            let litters: Vec<String> = litters.into_iter().map(|phenotype| {
                phenotype.phenotype
            }).collect();
            Msg::ReceivePhenotypeList(litters)
        });

        AddAnimal {
            id: None,
            litter_id: None,
            gender: None,
            status: None,
            litter_list: vec![],
            phenotype_list: vec![],
            response: String::new(),
            phenotype: None,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_update = false;
        match msg {
            Msg::SetId(id) => self.id = id,
            Msg::SetLitter(id) => self.litter_id = id,
            Msg::SetGender(gender) => self.gender = gender,
            Msg::SetStatus(status) => self.status = status,
            Msg::ReceiveLitterList(list) => {
                self.litter_list = list;
                should_update = true
            }
            Msg::ReceivePhenotypeList(list) => {
                self.phenotype_list = list;
                should_update = true
            },
            Msg::Submit => {
                ctx.link().send_future({
                    let animal = AnimalData {
                        id: self.id.clone().unwrap(), // TODO
                        fenotyp: self.phenotype.clone().unwrap(), // TODO
                        gender: self.gender.clone().unwrap(), // TODO
                        status: self.status.clone(),
                        litter: self.litter_id.clone(),
                        mother: None, // Not needed for submitting data
                        father: None // Not needed for submitting data
                    };
                    async move {
                        let response = backend_api::animal::post_animal(&animal).await;
                        Msg::Response(match &response {
                            Ok(_) => "Success!!".into(),
                            Err(error) => error.to_string(),
                        })
                    }
                })
            },
            Msg::SetPhenotype(phenotype) => self.phenotype = phenotype,
            Msg::Response(text) => {
                self.response = text;
                should_update = true;
            }
        }

        should_update
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let link = ctx.link();

        let on_id = link.callback(|input: Event| {
            let text = get_text_value(input);
            if text == "" {
                return Msg::SetId(None)
            }
            return Msg::SetId(Some(text))
        } );

        let default_litter = 
        ("Wybierz nr miotu".into(), link.callback(|_: MouseEvent| Msg::SetLitter(None)));
        let click_litter: Vec<(String, Callback<MouseEvent>)> = self.litter_list.iter().map(|id| {
            let id = id.clone();
            (id.clone(), ctx.link().callback(move |_: MouseEvent| Msg::SetLitter(Some(id.clone()))))
        }).collect();

        let default_phenotype = 
        ("Wybierz fenotyp myszy".into(), link.callback(|_: MouseEvent| Msg::SetPhenotype(None)));
        let options_phenotype: Vec<(String, Callback<MouseEvent>)> = self.phenotype_list.iter().map(|id| {
            let id = id.clone();
            (id.clone(), ctx.link().callback(move |_: MouseEvent| Msg::SetPhenotype(Some(id.clone()))))
        }).collect();

        let default_gender = 
        ("Wybierz płeć myszy".into(), link.callback(|_: MouseEvent| Msg::SetGender(None)));
        let options_gender = vec![
            ("Samiec".into(), link.callback(|_: MouseEvent| Msg::SetGender(Some(Gender::Male)))),
            ("Samica".into(), link.callback(|_: MouseEvent| Msg::SetGender(Some(Gender::Female)))),
        ];

        let on_status = link.callback(|input: Event| {
            let text = get_text_value(input);
            if text == "" {
                return Msg::SetStatus(None)
            }
            return Msg::SetStatus(Some(text))
        } );

        let submit = link.callback(|_: MouseEvent| { Msg::Submit });

        html! {
            <>
            <h1>{"Dodaj mysz"}</h1>
            <InputForm action={on_id} id="id" text="Id myszy"/>
            <DropdownForm id="litter" text="Wybierz miot" options={click_litter} default={default_litter}/>
            <DropdownForm id="phenotype" text="Wybierz Fenotyp" options={options_phenotype} default={default_phenotype}/>
            <DropdownForm id="gender" text="Wybierz Płeć" options={options_gender} default={default_gender}/>
            <InputForm action={on_status} id="status" text="Status"/>
            <button type="submit" onclick={submit} class="btn btn-primary mb-3">{"Dodaj Mysz"} </button>
            <a>{&self.response}</a>
            </>
        }
    }
}