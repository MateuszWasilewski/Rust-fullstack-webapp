use yew::{function_component, html, Html, Properties};

use common::Animal as AnimalStruct;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub animal_id: String
}


fn get_animal_page(animal: AnimalStruct) -> Html {
    let photos = animal.photos.into_iter().map( |photo| {
        let image_path = format!("/photo/{}", photo.path);
        html! {
            <img src={image_path} class="col-md-6 float-md-end mb-3 ms-md-3" alt="..."/>
        }
    }).collect::<Html>();

    html! {
        <>
            <h2> { format!("Information about: {}", animal.id)} </h2>
            <div class="clearfix">
                { photos }
                <p>
                    { "A paragraph of placeholder text. We re using it here to show the use of the clearfix class. We re adding quite a few meaningless phrases here to demonstrate how the columns interact here with the floated image." }
                </p>
                
                <p>
                    { "As you can see the paragraphs gracefully wrap around the floated image. Now imagine how this would look with some actual content in here, rather than just this boring placeholder text that goes on and on, but actually conveys no tangible information at. It simply takes up space and should not really be read." }
                </p>
                
                <p>
                    { "And yet, here you are, still persevering in reading this placeholder text, hoping for some more insights, or some hidden easter egg of content. A joke, perhaps. Unfortunately, theres none of that here." }
                </p>
                
            </div>
        </>
    }
}

#[function_component(Animal)]
pub fn get_animal_page(props: &Props) -> Html {
    let animal = backend_api::get_animal_by_id(&props.animal_id);

    match animal {
        Some(animal) => {
            get_animal_page(animal)
        },
        None => html! {
            <h2> { format!("Animal with given id {} could not be found", &props.animal_id)} </h2>
        }
    }
}