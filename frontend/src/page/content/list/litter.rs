use crate::{
    common::table::{List, Listable, RowView},
    page::navigation::links::{get_animal_link, get_litter_link},
};
use backend_api::LitterData;
use leptos::*;

struct LitterListData {}

fn litter_to_row_view(litter: LitterData) -> RowView {
    vec![
        get_litter_link(litter.id).into_view(),
        get_animal_link(litter.id_father).into_view(),
        get_animal_link(litter.id_mother).into_view(),
    ]
}

impl Listable for LitterListData {
    fn get_column_tags(&self) -> ReadSignal<RowView> {
        let (read, _) = create_signal(vec![
            "id miotu".into_view(),
            "id ojca".into_view(),
            "id matki".into_view(),
        ]);
        return read;
    }

    fn get_rows(&self) -> ReadSignal<Vec<RowView>> {
        let (read, write) = create_signal(vec![]);
        let resource = create_resource(
            || (),
            |_| async move { backend_api::get::litter_list().await.ok() },
        );
        create_effect(move |_| {
            write.set(
                resource
                    .get()
                    .unwrap_or_default()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|litter| litter_to_row_view(litter))
                    .collect(),
            );
        });
        return read;
    }
}

#[component]
pub fn LitterList() -> impl IntoView {
    let data = LitterListData {};

    view!(,
        <List data={data} />
    )
}
