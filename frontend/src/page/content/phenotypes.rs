use yew::{function_component, html, Html};
use crate::common::table::{RowProps, TableWithTags};

fn gene_tags() -> RowProps {
    vec![
        "wariant".into(),
        "fenotyp".into(),
        "genotyp".into()
    ]
}

fn phenotype_list(genes: &Vec<String>, phenotype: &common::Phenotype) -> RowProps {
    let mut fields: Vec<Html> = vec! [];
    fields.push(phenotype.variant.clone().into());
    fields.push(phenotype.phenotype.clone().into());

    let mut genotype = String::new();
    for gene in genes {
        let gene_str = match phenotype.genes.get(gene) {
            Some(gene_value) => gene_value,
            None => "."
        };
        genotype.push_str(&format!(" {gene_str}"));
    }

    fields.push(genotype.into());

    fields
}

#[function_component(Phenotypes)]
pub fn get_phenotypes() -> Html {
    let genes = backend_api::get_genes();
    let phenotypes = backend_api::get_phenotypes();

    let phenotypes_list: Vec<RowProps> = phenotypes.iter().map(|phenotype| 
        phenotype_list(&genes, phenotype)).collect();
    html! {
        <>
            <h1>{"Lista Fenotypów"} </h1>
            <TableWithTags tags={gene_tags()} data={phenotypes_list} />
        </>
    }
}