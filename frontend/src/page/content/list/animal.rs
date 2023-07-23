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

fn animal_to_row(cx: Scope, animal: AnimalData) -> RowView {
    vec![
        get_animal_link(cx, animal.id).into_view(cx),
        get_optional_litter_link(cx, animal.litter).into_view(cx),
        animal.fenotyp.into_view(cx),
        animal.status.into_view(cx),
        get_optional_animal_link(cx, animal.father).into_view(cx),
        get_optional_animal_link(cx, animal.mother).into_view(cx),
    ]
}

impl Listable for AnimalListData {
    fn get_column_tags(&self, cx: Scope) -> ReadSignal<RowView> {
        let (result, _) = create_signal(
            cx,
            vec![
                "id osobnika".into_view(cx),
                "nr miotu".into_view(cx),
                "fenotyp".into_view(cx),
                "status".into_view(cx),
                "ojciec".into_view(cx),
                "matka".into_view(cx),
            ],
        );
        return result;
    }

    fn get_rows(&self, cx: Scope) -> ReadSignal<Vec<RowView>> {
        let animals_resource = create_resource(cx, || (), |_| async move { fetch_data().await });
        let (read, write) = create_signal(cx, vec![]);
        create_effect(cx, move |_| {
            write.set(
                animals_resource
                    .read(cx)
                    .unwrap_or_default()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|animal| animal_to_row(cx, animal))
                    .collect(),
            )
        });
        return read;
    }
}

#[component]
pub fn AnimalList(cx: Scope) -> impl IntoView {
    let data = AnimalListData {};

    view!(cx,
        <List data={data} />
    )
}
