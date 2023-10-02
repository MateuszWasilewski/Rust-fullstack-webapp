use crate::common::table::{List, Listable, RowView};
use leptos::*;

fn phenotype_to_row(phenotype: backend_api::Phenotype) -> RowView {
    vec![
        phenotype.variant.clone().into_view(),
        phenotype.phenotype.clone().into_view(),
    ]
}

struct PhenotypeListData {}

impl Listable for PhenotypeListData {
    fn get_column_tags(&self) -> ReadSignal<RowView> {
        let (read, _) = create_signal(vec!["wariant".into_view(), "fenotyp".into_view()]);
        return read;
    }

    fn get_rows(&self) -> ReadSignal<Vec<RowView>> {
        let resource = create_resource(
            || (),
            |_| async move { backend_api::get::phenotypes().await.ok() },
        );
        let (read, write) = create_signal(vec![]);
        create_effect(move |_| {
            write.set(
                resource
                    .get()
                    .unwrap_or_default()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|phenotype| phenotype_to_row(phenotype))
                    .collect(),
            );
        });
        return read;
    }
}
#[component]
pub fn PhenotypeList() -> impl IntoView {
    let data = PhenotypeListData {};

    view!(
        <List data={data}/>
    )
}
