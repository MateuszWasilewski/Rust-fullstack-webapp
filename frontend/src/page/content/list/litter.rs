use crate::{
    common::table::{List, Listable, RowView},
    page::navigation::links::{get_animal_link, get_litter_link},
};
use backend_api::LitterData;
use leptos::*;

struct LitterListData {}

fn litter_to_row_view(cx: Scope, litter: LitterData) -> RowView {
    vec![
        get_litter_link(cx, litter.id).into_view(cx),
        get_animal_link(cx, litter.id_father).into_view(cx),
        get_animal_link(cx, litter.id_mother).into_view(cx),
    ]
}

impl Listable for LitterListData {
    fn get_column_tags(&self, cx: Scope) -> ReadSignal<RowView> {
        let (read, _) = create_signal(
            cx,
            vec![
                "id miotu".into_view(cx),
                "id ojca".into_view(cx),
                "id matki".into_view(cx),
            ],
        );
        return read;
    }

    fn get_rows(&self, cx: Scope) -> ReadSignal<Vec<RowView>> {
        let (read, write) = create_signal(cx, vec![]);
        let resource = create_resource(
            cx,
            || (),
            |_| async move { backend_api::get::litter_list().await.ok() },
        );
        create_effect(cx, move |_| {
            write.set(
                resource
                    .read(cx)
                    .unwrap_or_default()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|litter| litter_to_row_view(cx, litter))
                    .collect(),
            );
        });
        return read;
    }
}

#[component]
pub fn LitterList(cx: Scope) -> impl IntoView {
    let data = LitterListData {};

    view!(cx,
        <List data={data} />
    )
}
