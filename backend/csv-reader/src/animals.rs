use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs};

static FILE_PATH: &str = "files/db/baza-myszy.csv";
static NEW_FILE_PATH: &str = "files/db/nowa-baza-myszy.csv";

#[derive(Deserialize, Serialize, Debug)]
struct AnimalRecord {
    nr_miotu: Option<String>,
    numer_osobnika: String,
    odmiana_barwna: String,
    płeć: String,
    kolor_oka: Option<String>,
    włos: Option<String>,
    ojciec: Option<String>,
    matka: Option<String>
}

use common::{animal::Gender, AnimalData, litter::LitterData};

async fn parse_original() {
    let text = fs::read_to_string(FILE_PATH).expect("Should be able to read file");

    let mut reader = csv::Reader::from_reader(text.as_bytes());
    let mut writer = csv::WriterBuilder::default()
        .has_headers(false)
        .from_path(NEW_FILE_PATH)
        .unwrap();
    let mut row_id: u32 = 0;

    for record in reader.records() {
        row_id += 1;
        let record = record.unwrap();

        println!("Row {} | {:#?}", row_id, &record[3]);
        if !record[3].is_empty() {
            writer.write_record(&record).unwrap();
        }
    }
    writer.flush().unwrap();
}

async fn parse_new() {
    let text = fs::read_to_string(NEW_FILE_PATH).expect("Should be able to read file");
    let original = fs::read_to_string(FILE_PATH).expect("Should be able to read file");

    let mut reader = csv::Reader::from_reader(text.as_bytes());
    let mut old_reader = csv::Reader::from_reader(original.as_bytes());
    reader.set_headers(old_reader.headers().unwrap().clone());

    let pool = db::connect_db().await.expect("Failed to connect DB");
    let fenotypy: HashSet<String> = db::select::phenotype_list(&pool)
        .await
        .unwrap()
        .into_iter()
        .map(|phen| phen.phenotype)
        .collect();

    for record in reader.deserialize() {
        let record: AnimalRecord = record.unwrap();

        //println!("{:?}", record);

        let mut fenotyp = record.odmiana_barwna;
        if !fenotypy.contains(&fenotyp) {
          fenotyp = "nieznany".into();
        }

        let litter = if let (Some(ojciec), Some(matka)) = (record.ojciec, record.matka)  {
            let mut split = record.numer_osobnika.split(".");
            let litter_id = split.next().unwrap();
            let litter = LitterData {
                id: litter_id.into(),
                id_mother: matka,
                id_father: ojciec
            };
            let result = db::insert::litter(&litter, &pool).await;
            if let Err(error) = result {
                println!("{error}");
            }
            Some(litter.id)
        }
        else {
            None
        };

        let animal = AnimalData {
            id: record.numer_osobnika.clone(),
            fenotyp: fenotyp,
            status: None,
            gender: match record.płeć == "M" {
                true => Gender::Male,
                false => Gender::Female,
            },
            litter: litter,
            mother: None,
            father: None,
            eye_color: record.kolor_oka,
            hair: record.włos
        };

        println!("{:?}", animal);

        let result = db::insert::animal(&animal, &pool).await;
        if let Err(error) = result {
            println!("{}", error)
        }
    }
}

pub async fn run_animals() {
    parse_original().await;
    parse_new().await;
}
