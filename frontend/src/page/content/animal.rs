use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub animal_id: String
}


#[function_component(Animal)]
pub fn get_animal_page(props: &Props) -> Html {
    let animal = backend_api::get_animal_by_id(&props.animal_id);

    match animal {
        Some(animal) => {
            html! {
                <h2> { format!("Information about: {}", animal.id)} </h2>
            }
        },
        None => html! {
            <h2> { format!("Animal with given id {} could not be found", &props.animal_id)} </h2>
        }
    }

}