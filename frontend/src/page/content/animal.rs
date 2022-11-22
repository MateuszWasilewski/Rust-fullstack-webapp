use yew::{function_component, html, Html, Properties};

use common::Animal as AnimalStruct;
use common::animal::Gender;

use crate::page::routes::get_animal_link;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub animal_id: String
}


fn get_animal_page(animal: &AnimalStruct) -> Html {
    let photos = animal.photos.iter().map( |photo| {
        let image_path = format!("/photo/{}", photo.path);
        html! {
            <img src={image_path} class="col-md-6 float-md-end mb-3 ms-md-3" alt="..."/>
        }
    }).collect::<Html>();

    let mut data = vec![
        (html!{"id"}, get_animal_link(animal.id.as_str())),
        (html!{"płeć"}, html!{
            match animal.gender {
                Gender::Male => "M",
                Gender::Female => "F"
            }
        }),
        (html!{"fenotyp"}, html!{ &animal.fenotyp }),
        //(html!{"kolor oka"}, html! {""}),
        //(html!{"włos"}, html! {""}),
    ];

    if let Some(litter) = &animal.litter {
        let mut bonus_data = vec![
            (html!{"nr miotu"}, html!{ litter.id }),
            (html!{"ojciec"}, get_animal_link(&litter.father)),
            (html!{"matka"}, get_animal_link(&litter.mother)),
            (html!{"data narodzin"}, html!{&litter.birth_date}),
        ];
        data.append(&mut bonus_data);
    }
    
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
            </div>
        </>
    }
}

#[function_component(Animal)]
pub fn get_animal_page(props: &Props) -> Html {
    let animal = backend_api::get_animal_by_id(&props.animal_id);

    match animal {
        Some(animal) => {
            get_animal_page(&animal)
        },
        None => html! {
            <h2> { format!("Animal with given id {} could not be found", &props.animal_id)} </h2>
        }
    }
}