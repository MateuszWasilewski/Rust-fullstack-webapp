use frontend_routing::Routes;
use web_sys::MouseEvent;
use yew::platform::spawn_local;
use yew::{html, Html, Properties, function_component, Callback};

use common::AnimalFull;
use common::animal::Gender;
use yew_router::prelude::use_navigator;
use yewdux::prelude::{Dispatch, use_store_value};
use yewdux::store::Store;

use frontend_routing::links::AnimalLink;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub animal_id: String
}

fn get_animal_link(id: &str) -> Html {
    html! {
        <AnimalLink id={id.to_owned()} />
    }
}

#[function_component]
fn AnimalPage() -> Html {
    let navigator = use_navigator().unwrap();
    let state = use_store_value::<State>();
    if let None = state.animal {
        return html!{}
    }

    let animal = state.animal.clone().unwrap();
    let all_genes = state.genes.as_ref();

    let photos = animal.photos.iter().map( |photo| {
        let image_path = format!("/{}", photo.path);
        html! {
            <img src={image_path} class="col-md-6 float-md-end mb-3 ms-md-3" alt="..."/>
        }
    }).collect::<Html>();

    let onclick = Callback::from( {
        let id = animal.id.clone();
        move |_: MouseEvent| {
            let id = id.clone();
            let navigator = navigator.clone();
            spawn_local( async move {
                match backend_api::delete::animal(&id).await {
                    Ok(_) => navigator.push(&Routes::Home),
                    Err(_) => navigator.push(&Routes::ServerError)
                }
            })
        }
    });

    let mut data = vec![
        (html!{"id"}, get_animal_link(&animal.id)),
        (html!{"płeć"}, html!{
            match animal.gender {
                Gender::Male => "M",
                Gender::Female => "F"
            }
        }),
        (html!{"fenotyp"}, html!{ &animal.fenotyp }),
        (html!{"kolor oka"}, animal.eye_color.clone().unwrap_or("nieznany".into()).into()),
        (html!{"włos"}, animal.hair.clone().unwrap_or("nieznany".into()).into()),
    ];

    if let Some(litter) = &animal.litter {
        data.push(("nr miotu".into(), html!{litter}));
    }
    if let Some(father) = &animal.father {
        data.push(("ojciec".into(), get_animal_link(father)));
    }
    if let Some(mother) = &animal.mother {
        data.push(("matka".into(), get_animal_link(mother)));
    }
    animal.genes.iter().for_each(|genes| {
        data.push(("możliwy genotyp".into(), genes.get_genotype(all_genes).into()))
    });
    
    let data = data.into_iter().map(|(name, value)| {
        html! {
            <div class="row border-top">
                <div class="col">
                    {name}
                </div>
                <div class="col">
                    {value}
                </div>
            </div>
        }
    }).collect::<Html>();

    html! {
        <>
            <h2> { format!("Informacje o myszy: {}", animal.id)} </h2>
            <div class="clearfix">
                { photos }
                { data }
                <button type="button" class="btn btn-primary btn-sm" {onclick}>{"Usuń mysz"}</button>
            </div>
        </>
    }
}

#[derive(PartialEq, Default, Store, Clone)]
struct State {
    id: String,
    genes: Vec<String>,
    animal: Option<AnimalFull>
}

async fn fetch_animal(dispatch: Dispatch<State>) {
    let id = &dispatch.get().id;
    let animal = backend_api::get::animal_by_id(id).await;
    if let Ok(animal) = animal {
        dispatch.reduce_mut(|state| state.animal = Some(animal));
    }
}

async fn fetch_genes(dispatch : Dispatch<State>) {
    let genes = backend_api::get::genes().await;
    if let Ok(genes) = genes {
        dispatch.reduce_mut(|state| state.genes = genes);
    }
}

#[function_component]
pub fn Animal(props: &Props) -> Html {
    let dispatch = Dispatch::<State>::new();
    dispatch.reduce_mut(|state| state.id = props.animal_id.clone());
    spawn_local(fetch_animal(dispatch.clone()));
    spawn_local(fetch_genes(dispatch.clone()));

    html!{
        <AnimalPage />
    }
}
