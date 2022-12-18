use common::Animal;
use yew::{html, Html, Callback, MouseEvent, function_component, Properties, AttrValue, Component};
use yew_router::prelude::*;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Routes {
    #[at("/")]
    Home,
    #[at("/animal-list")]
    AnimalList,
    #[at("/animal/:id")]
    GoToAnimal {id: String},
    #[at("/phenotypes")]
    Phenotypes,
    #[at("/litter-list")]
    Litters
}

#[derive(PartialEq, Properties, Clone)]
pub struct LinkProps {
    pub target: Routes,
    pub link_name: String
}

#[function_component(Link)]
pub fn get_link(props: &LinkProps) -> Html {
    let navigator = use_navigator().unwrap();
    let route = props.target.clone();
    let onclick = Callback::from(move |_: MouseEvent| navigator.push(&route));
    html! {
        <a class="nav-link active" href="javascript:void(0);" onclick={onclick}>{props.link_name.to_owned()}</a>
    }
}

#[derive(PartialEq, Properties)]
pub struct AnimalLinkProps {
    pub id: AttrValue
}

pub struct AnimalLink {
    animal_id: AttrValue,
    animal: Option<Animal>
}

impl AnimalLink {
    fn fetch_animal(&self, ctx: &yew::Context<Self>) {
        let props = ctx.props();
        ctx.link().send_future({
            let id = props.id.clone();
            async move {
                let animal = backend_api::get_animal_by_id(&id).await;
                animal
            }
        });
    }
}

impl Component for AnimalLink {
    type Message = Result<Animal>;
    type Properties = AnimalLinkProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let animal = AnimalLink { 
            animal_id: ctx.props().id.clone(), 
            animal: None 
        };
        animal.fetch_animal(ctx);
        animal
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        self.animal = msg.ok();
        true
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        self.animal_id = ctx.props().id.clone();
        self.fetch_animal(ctx);
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        match self.animal {
            Some(_) => {
                let onclick = {
                    let navigator = ctx.link().navigator().unwrap();
                    let id = self.animal_id.clone();
                    Callback::from(move |_: MouseEvent| {
                    navigator.push(&Routes::GoToAnimal { id: id.to_string() })
                })};
                html! {
                    <a class="nav-link active text-primary" href="javascript:void(0);" onclick={onclick}>{&self.animal_id}</a>
                }
            }
            None => {
                html! {
                    {&self.animal_id}
                }
            }
        }
    }

}