use crate::common::table::{List, Listable, RowView};
use crate::page::navigation::links::{
    get_animal_link, get_optional_animal_link, get_optional_litter_link,
};
use backend_api::AnimalData;
use leptos::*;

async fn fetch_data() -> Option<Vec<AnimalData>> {
    backend_api::get::all_animal().await.ok()
}

struct AnimalListData {}

fn animal_to_row(animal: AnimalData) -> RowView {
    vec![
        get_animal_link(animal.id).into_view(),
        get_optional_litter_link(animal.litter).into_view(),
        animal.fenotyp.into_view(),
        animal.status.into_view(),
        get_optional_animal_link(animal.father).into_view(),
        get_optional_animal_link(animal.mother).into_view(),
    ]
}

impl Listable for AnimalListData {
    fn get_column_tags(&self) -> ReadSignal<RowView> {
        let (result, _) = create_signal(vec![
            "id osobnika".into_view(),
            "nr miotu".into_view(),
            "fenotyp".into_view(),
            "status".into_view(),
            "ojciec".into_view(),
            "matka".into_view(),
        ]);
        return result;
    }

    fn get_rows(&self) -> ReadSignal<Vec<RowView>> {
        let animals_resource = create_resource(|| (), |_| async move { fetch_data().await });
        let (read, write) = create_signal(vec![]);
        create_effect(move |_| {
            write.set(
                animals_resource
                    .get()
                    .unwrap_or_default()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|animal| animal_to_row(animal))
                    .collect(),
            )
        });
        return read;
    }
}

#[component]
pub fn AnimalList() -> impl IntoView {
    let data = AnimalListData {};

    view!(
        <List data={data} />
    )
}
