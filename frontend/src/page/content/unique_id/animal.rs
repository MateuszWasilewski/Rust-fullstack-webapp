use backend_api::{AnimalFull, Gender};
use leptos::*;
use leptos_router::*;

use crate::page::navigation::links::get_animal_link;

#[derive(Params, PartialEq)]
struct AnimalParams {
    id: Option<String>,
}

#[component]
pub fn Animal() -> impl IntoView {
    let params = use_params::<AnimalParams>();
    let id = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.id.clone())
                .unwrap_or_default()
                .unwrap_or_default()
        })
    };
    let top_msg = move || format!("Informacje o myszy {}", id());
    let animal_data = create_resource(id, |id| async move {
        backend_api::get::full_animal_by_id(&id).await.ok()
    });

    let animal_view = move || get_animal_view(animal_data.get().unwrap_or_default());

    view! {
        <h2> {top_msg} </h2>
        <div class="clearfix">
            {animal_view}
            // <button type="button" class="btn btn-primary btn-sm" {onclick}>{"Usuń mysz"}</button>
        </div>
    }
}

struct RowData {
    tag: View,
    value: View,
}

impl RowData {
    fn new(tag: impl IntoView, value: impl IntoView) -> RowData {
        RowData {
            tag: tag.into_view(),
            value: value.into_view(),
        }
    }
}

fn get_animal_view(animal: Option<AnimalFull>) -> impl IntoView {
    animal.map(|animal| {
        let photos = animal
            .photos
            .iter()
            .map(|photo| {
                let image_path = format!("/{}", photo.path);
                view! (
                    <img src={image_path} class="col-md-6 float-md-end mb-3 ms-md-3" alt="..."/>
                )
            })
            .collect_view();
        let mut data = vec![
            RowData::new("id", get_animal_link(animal.id)),
            RowData::new(
                "płeć",
                match animal.gender {
                    Gender::Male => "M",
                    Gender::Female => "F",
                },
            ),
            RowData::new("fenotyp", animal.fenotyp.unwrap_or("nieznany".into())),
            RowData::new("kolor oka", animal.eye_color.unwrap_or("nieznany".into())),
            RowData::new("włos", animal.hair.unwrap_or("nieznany".into())),
        ];
        if let Some(litter) = &animal.litter {
            data.push(RowData::new("nr miotu", litter));
        }
        if let Some(father) = &animal.father {
            data.push(RowData::new("ojciec", get_animal_link(father.clone())));
        }
        if let Some(mother) = &animal.mother {
            data.push(RowData::new("matka", get_animal_link(mother.clone())));
        }

        let data = data
            .into_iter()
            .map(|row_data| {
                view! (
                    <div class="row border-top">
                        <div class="col">
                            {row_data.tag}
                        </div>
                        <div class="col">
                            {row_data.value}
                        </div>
                    </div>
                )
            })
            .collect_view();

        view!(
            { photos }
            { data }
        )
    })
}
