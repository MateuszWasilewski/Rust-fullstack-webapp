use yew::{Component, html, Event, MouseEvent, Callback};

use crate::common::input::{get_text_value, InputForm, DropdownForm};

pub struct AddAnimal {
    id: String,
    litter_id: String,
    litter_list: Vec<String>,
    phenotype_list: Vec<String>
}

pub enum Msg {
    SetId(String),
    SetLitter(String),
    SetPhenotype(String),
    ReceiveLitterList(Vec<String>),
    ReceivePhenotypeList(Vec<String>)
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
            id: String::new(),
            litter_id: String::new(),
            litter_list: vec![],
            phenotype_list: vec![]
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_update = false;
        match msg {
            Msg::SetId(id) => self.id = id,
            Msg::SetLitter(id) => self.litter_id = id,
            Msg::ReceiveLitterList(list) => {
                self.litter_list = list;
                should_update = true
            }
            Msg::ReceivePhenotypeList(list) => {
                self.phenotype_list = list;
                should_update = true
            }
            _ => ()
        }

        should_update
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let link = ctx.link();

        let on_id = link.callback(|input: Event| {
            Msg::SetId(get_text_value(input))
        } );

        let default_litter = 
        ("Wybierz nr miotu".into(), link.callback(|_: MouseEvent| Msg::SetLitter(String::new())));
        let click_litter: Vec<(String, Callback<MouseEvent>)> = self.litter_list.iter().map(|id| {
            let id = id.clone();
            (id.clone(), ctx.link().callback(move |_: MouseEvent| Msg::SetLitter(id.clone())))
        }).collect();

        let default_phenotype = 
        ("Wybierz fenotyp myszy".into(), link.callback(|_: MouseEvent| Msg::SetPhenotype(String::new())));
        let options_phenotype: Vec<(String, Callback<MouseEvent>)> = self.phenotype_list.iter().map(|id| {
            let id = id.clone();
            (id.clone(), ctx.link().callback(move |_: MouseEvent| Msg::SetPhenotype(id.clone())))
        }).collect();

        html! {
            <>
            <h1>{"Dodaj mysz"}</h1>
            <InputForm action={on_id} id="id" text="Id myszy"/>
            <DropdownForm id="litter" text="Wybierz miot" options={click_litter} default={default_litter}/>
            <DropdownForm id="phenotype" text="Wybierz Fenotyp" options={options_phenotype} default={default_phenotype}/>
            </>
        }
    }
}