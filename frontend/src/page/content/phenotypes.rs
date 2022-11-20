use yew::{function_component, html, Html};


fn get_table_tags(tags: &Vec<String>) -> Html {
    let tags = tags.iter().map(|tag| {
        html! {
            <div class="col">
                {tag}
            </div>
        }
    } ).collect::<Html>();

    html! {
        <div class="row fst-italic">
            {tags}
        </div>
    }
}

fn get_table_fields(fields: &Vec<String>) -> Html {
    let html_fields = fields.iter().map(|field| {
        html! {
            <div class="col">
                {field}
            </div>
        }
    }).collect::<Html>();

    html! {
        <div class="row border-top">
            { html_fields }
        </div>
    }
}

fn gene_tags() -> Html {
    let tags = vec![
        "variant".to_owned(),
        "phenotype".to_owned(),
        "genotyp".to_owned()
    ];
    get_table_tags(&tags)
}

fn phenotype_list(genes: &Vec<String>, phenotype: &common::Phenotype) -> Html {
    let mut fields = vec! [];
    fields.push(phenotype.variant.clone());
    fields.push(phenotype.phenotype.clone());

    let mut genotype = String::new();
    for gene in genes {
        let gene_str = match phenotype.genes.get(gene) {
            Some(gene_value) => gene_value,
            None => "."
        };
        genotype.push_str(gene_str);
    }
    fields.push(genotype);

    get_table_fields(&fields)
}

#[function_component(Phenotypes)]
pub fn get_phenotypes() -> Html {
    let genes = backend_api::get_genes();
    let phenotypes = backend_api::get_phenotypes();

    let phenotypes_list = phenotypes.iter().map(|phenotype| phenotype_list(&genes, phenotype)).collect::<Html>();
    html! {
        <>
            <h1>{"Phenotypes List"} </h1>
            { gene_tags() }
            { phenotypes_list }
        </>
    }
}