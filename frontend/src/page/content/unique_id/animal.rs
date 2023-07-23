use backend_api::{AnimalFull, Gender};
use leptos::*;
use leptos_router::*;

use crate::page::navigation::links::get_animal_link;

#[derive(Params, PartialEq)]
struct AnimalParams {
    id: Option<String>,
}

#[component]
pub fn Animal(cx: Scope) -> impl IntoView {
    let params = use_params::<AnimalParams>(cx);
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
    let animal_data = create_resource(cx, id, |id| async move {
        backend_api::get::full_animal_by_id(&id).await.ok()
    });

    let animal_view = move || get_animal_view(cx, animal_data.read(cx).unwrap_or_default());

    view!(
        cx,
        <h2> {top_msg} </h2>
        <div class="clearfix">
            {animal_view}
            // <button type="button" class="btn btn-primary btn-sm" {onclick}>{"Usuń mysz"}</button>
        </div>
    )
}

struct RowData {
    tag: View,
    value: View,
}

impl RowData {
    fn new(cx: Scope, tag: impl IntoView, value: impl IntoView) -> RowData {
        RowData {
            tag: tag.into_view(cx),
            value: value.into_view(cx),
        }
    }
}

fn get_animal_view(cx: Scope, animal: Option<AnimalFull>) -> impl IntoView {
    animal.map(|animal| {
        let photos = animal
            .photos
            .iter()
            .map(|photo| {
                let image_path = format!("/{}", photo.path);
                view! (cx,
                    <img src={image_path} class="col-md-6 float-md-end mb-3 ms-md-3" alt="..."/>
                )
            })
            .collect_view(cx);
        let mut data = vec![
            RowData::new(cx, "id", get_animal_link(cx, animal.id)),
            RowData::new(
                cx,
                "płeć",
                match animal.gender {
                    Gender::Male => "M",
                    Gender::Female => "F",
                },
            ),
            RowData::new(cx, "fenotyp", animal.fenotyp.unwrap_or("nieznany".into())),
            RowData::new(
                cx,
                "kolor oka",
                animal.eye_color.unwrap_or("nieznany".into()),
            ),
            RowData::new(cx, "włos", animal.hair.unwrap_or("nieznany".into())),
        ];
        if let Some(litter) = &animal.litter {
            data.push(RowData::new(cx, "nr miotu", litter));
        }
        if let Some(father) = &animal.father {
            data.push(RowData::new(
                cx,
                "ojciec",
                get_animal_link(cx, father.clone()),
            ));
        }
        if let Some(mother) = &animal.mother {
            data.push(RowData::new(
                cx,
                "matka",
                get_animal_link(cx, mother.clone()),
            ));
        }

        let data = data
            .into_iter()
            .map(|row_data| {
                view! (cx,
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
            .collect_view(cx);

        view!(
            cx,
            { photos }
            { data }
        )
    })
}
