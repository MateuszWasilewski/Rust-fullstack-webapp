use common::Phenotype;
use rocket::serde::json::Json;


#[get("/genes-list")]
pub fn genes_list() -> Json<Vec<String>>{
    let result = vec![
        "ASIP".to_owned(),
        "ASIP2".to_owned(),
        "TYRP1".to_owned(),
        "TYRP2"	.to_owned(),
        "TYR".to_owned(),
        "TYR2".to_owned(),
        "MYO5A".to_owned(),
        "MYO5A2".to_owned(),
        "OCA2".to_owned(),
        "OCA3".to_owned(),
    ];
    Json(result)
}

#[get("/phenotype-list")]
pub fn get_phenotypes() -> Json<Vec<Phenotype>> {
    Json(vec! [
        Phenotype {
            phenotype: "agouti".to_owned(),
            variant: "ticked".to_owned(),
            genes: [
                ("ASIP".to_owned(), "A".to_owned()),
                ("TYRP1".to_owned(), "B".to_owned()),
                ("TYR".to_owned(), "C".to_owned()),
                ("MYO5A".to_owned(), "D".to_owned()),
                ("OCA2".to_owned(), "P".to_owned())
            ].into_iter().collect()
        },
        Phenotype {
            phenotype: "agouti white-bellied".to_owned(),
            variant: "ticked".to_owned(),
            genes: [
                ("ASIP".to_owned(), "Aw".to_owned()),
                ("TYRP1".to_owned(), "B".to_owned()),
                ("TYR".to_owned(), "C".to_owned()),
                ("MYO5A".to_owned(), "D".to_owned()),
                ("OCA2".to_owned(), "P".to_owned())
            ].into_iter().collect()
        },
        Phenotype {
            phenotype: "agouti tan".to_owned(),
            variant: "ticked".to_owned(),
            genes: [
                ("ASIP".to_owned(), "A".to_owned()),
                ("ASIP2".to_owned(), "at".to_owned()),
                ("TYRP1".to_owned(), "B".to_owned()),
                ("TYR".to_owned(), "C".to_owned()),
                ("MYO5A".to_owned(), "D".to_owned()),
                ("OCA2".to_owned(), "P".to_owned())
            ].into_iter().collect()
        },
        Phenotype {
            phenotype: "argente".to_owned(),
            variant: "ticked".to_owned(),
            genes: [
                ("ASIP".to_owned(), "A".to_owned()),
                ("TYRP1".to_owned(), "B".to_owned()),
                ("TYR".to_owned(), "C".to_owned()),
                ("MYO5A".to_owned(), "D".to_owned()),
                ("OCA2".to_owned(), "p".to_owned()),
                ("OCA3".to_owned(), "p".to_owned())
            ].into_iter().collect()
        },
        Phenotype {
            phenotype: "argente tan".to_owned(),
            variant: "ticked".to_owned(),
            genes: [
                ("ASIP".to_owned(), "A".to_owned()),
                ("ASIP2".to_owned(), "at".to_owned()),
                ("TYRP1".to_owned(), "B".to_owned()),
                ("TYR".to_owned(), "C".to_owned()),
                ("MYO5A".to_owned(), "D".to_owned()),
                ("OCA2".to_owned(), "p".to_owned()),
                ("OCA3".to_owned(), "p".to_owned())
            ].into_iter().collect()
        },
        Phenotype {
            phenotype: "argente tan".to_owned(),
            variant: "ticked".to_owned(),
            genes: [
                ("ASIP".to_owned(), "Aw".to_owned()),
                ("ASIP2".to_owned(), "at".to_owned()),
                ("TYRP1".to_owned(), "B".to_owned()),
                ("TYR".to_owned(), "C".to_owned()),
                ("MYO5A".to_owned(), "D".to_owned()),
                ("OCA2".to_owned(), "p".to_owned()),
                ("OCA3".to_owned(), "p".to_owned())
            ].into_iter().collect()
        },
        Phenotype {
            phenotype: "cynamon agouti".to_owned(),
            variant: "ticked".to_owned(),
            genes: [
                ("ASIP".to_owned(), "A".to_owned()),
                ("TYRP1".to_owned(), "b".to_owned()),
                ("TYRP2".to_owned(), "b".to_owned()),
                ("TYR".to_owned(), "C".to_owned()),
                ("MYO5A".to_owned(), "D".to_owned()),
                ("OCA2".to_owned(), "P".to_owned()),
            ].into_iter().collect()
        },
        Phenotype {
            phenotype: "cynamon tan".to_owned(),
            variant: "ticked".to_owned(),
            genes: [
                ("ASIP".to_owned(), "A".to_owned()),
                ("ASIP2".to_owned(), "at".to_owned()),
                ("TYRP1".to_owned(), "b".to_owned()),
                ("TYRP2".to_owned(), "b".to_owned()),
                ("TYR".to_owned(), "C".to_owned()),
                ("MYO5A".to_owned(), "D".to_owned()),
                ("OCA2".to_owned(), "P".to_owned()),
            ].into_iter().collect()
        },
        Phenotype {
            phenotype: "blue agouti".to_owned(),
            variant: "ticked".to_owned(),
            genes: [
                ("ASIP".to_owned(), "A".to_owned()),
                ("TYRP1".to_owned(), "B".to_owned()),
                ("TYR".to_owned(), "C".to_owned()),
                ("MYO5A".to_owned(), "d".to_owned()),
                ("MYO5A2".to_owned(), "d".to_owned()),
                ("OCA2".to_owned(), "P".to_owned()),
            ].into_iter().collect()
        },
        Phenotype {
            phenotype: "blue agouti tan".to_owned(),
            variant: "ticked".to_owned(),
            genes: [
                ("ASIP".to_owned(), "A".to_owned()),
                ("ASIP2".to_owned(), "at".to_owned()),
                ("TYRP1".to_owned(), "B".to_owned()),
                ("TYR".to_owned(), "C".to_owned()),
                ("MYO5A".to_owned(), "D".to_owned()),
                ("OCA2".to_owned(), "P".to_owned()),
            ].into_iter().collect()
        },
    ])
}