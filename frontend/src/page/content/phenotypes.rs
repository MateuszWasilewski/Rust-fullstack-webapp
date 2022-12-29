use yew::{html, Html, Component};
use crate::common::table::{RowProps, TableWithTags};
use anyhow::Result;

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

pub struct Phenotypes {
    table: Html
}

impl Component for Phenotypes {
    type Message = Result<Html>;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future(async move {
            let genes = backend_api::get_genes().await?;
            let phenotypes = backend_api::get_phenotypes_full().await?;
            let phenotypes_list: Vec<RowProps> = phenotypes
                .iter()
                .map(|phenotype| phenotype_list(&genes, phenotype))
                .collect();

            Ok(html! {
                <TableWithTags tags={gene_tags()} data={phenotypes_list} />
            })
        });
        Phenotypes { table: html!{} }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let new_table = match msg {
            Ok(data) => data,
            Err(err) => html! {
                <>
                    <h4> {"Received error"}</h4>
                    {err}
                </>
            },
        };
        if new_table != self.table {
            self.table = new_table;
            true
        }
        else {
            false
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
            <>
                <h1>{"Lista Fenotypów"} </h1>
                { self.table.clone() }
            </>
        }
    }
}
