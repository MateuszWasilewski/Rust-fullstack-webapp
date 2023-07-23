use crate::common::table::{List, Listable, RowView};
use leptos::*;

fn phenotype_to_row(cx: Scope, phenotype: backend_api::Phenotype) -> RowView {
    vec![
        phenotype.variant.clone().into_view(cx),
        phenotype.phenotype.clone().into_view(cx),
    ]
}

struct PhenotypeListData {}

impl Listable for PhenotypeListData {
    fn get_column_tags(&self, cx: Scope) -> ReadSignal<RowView> {
        let (read, _) = create_signal(cx, vec!["wariant".into_view(cx), "fenotyp".into_view(cx)]);
        return read;
    }

    fn get_rows(&self, cx: Scope) -> ReadSignal<Vec<RowView>> {
        let resource = create_resource(
            cx,
            || (),
            |_| async move { backend_api::get::phenotypes().await.ok() },
        );
        let (read, write) = create_signal(cx, vec![]);
        create_effect(cx, move |_| {
            write.set(
                resource
                    .read(cx)
                    .unwrap_or_default()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|phenotype| phenotype_to_row(cx, phenotype))
                    .collect(),
            );
        });
        return read;
    }
}
#[component]
pub fn PhenotypeList(cx: Scope) -> impl IntoView {
    let data = PhenotypeListData {};

    view!(cx,
        <List data={data}/>
    )
}
