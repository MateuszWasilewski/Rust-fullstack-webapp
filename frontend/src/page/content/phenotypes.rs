use crate::common::table::{RowProps, TableWithTags};
use common::Phenotype;
use yew::{html, Html, function_component, platform::spawn_local};
use yewdux::{store::Store, prelude::{Dispatch, use_store_value}};

fn gene_tags() -> RowProps {
    vec!["wariant".into(), "fenotyp".into(), "genotyp".into()]
}

#[derive(PartialEq, Default, Store, Clone)]
struct State {
    phenotypes: Option<Vec<Phenotype>>,
    gene_names: Option<Vec<String>>
}

async fn fetch_data() {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();
    if let None = state.phenotypes {
        backend_api::get::phenotypes_full().await.map(|phenotypes| {
            dispatch.reduce_mut(|state| state.phenotypes = Some(phenotypes) )
        }).ok();
    }
    if let None = state.gene_names {
        backend_api::get::genes().await.map(|gene_names| {
            dispatch.reduce_mut(|state| state.gene_names = Some(gene_names) )
        }).ok();
    }
}

#[function_component]
fn Page() -> Html {
    let state = use_store_value::<State>();
    let phenotypes = state.phenotypes.clone().unwrap_or_default();
    let gene_names = state.gene_names.clone().unwrap_or_default();

    let phenotypes_list: Vec<RowProps> = phenotypes
        .iter()
        .map(|phenotype| phenotype_list(&gene_names, phenotype))
        .collect();

    html! {
        <TableWithTags tags={gene_tags()} data={phenotypes_list} />
    }
}

#[function_component]
pub fn Phenotypes() -> Html {
    spawn_local(fetch_data());

    html!{
        <Page />
    }
}

fn phenotype_list(genes: &Vec<String>, phenotype: &common::Phenotype) -> RowProps {
    let mut fields: Vec<Html> = vec![];
    fields.push(phenotype.variant.clone().into());
    fields.push(phenotype.phenotype.clone().into());
    fields.push(phenotype.genes.get_genotype(genes).into());

    fields
}