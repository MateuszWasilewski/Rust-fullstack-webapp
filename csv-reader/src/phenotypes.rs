use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;

#[allow(unused_variables)]

static FILE_PATH: &str = "db/Fenotypy-genotypy.csv";


#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct PhenotypeFull {
    WARIANT: String,
    FENOTYP: String,
    ASIP1: Option<String>,
    ASIP2: Option<String>,
    TYRP1: Option<String>,
    TYRP2: Option<String>,
    TYR1: Option<String>,
    TYR2: Option<String>,
    MYO5A1: Option<String>,
    MYO5A2: Option<String>,
    OCA1: Option<String>,
    OCA2: Option<String>,
    MLPH1: Option<String>,
    MLPH2: Option<String>,
    PMEL1: Option<String>,
    PMEL2: Option<String>,
    ATP7A1: Option<String>,
    ATP7A2: Option<String>,
    KIT1: Option<String>,
    KIT2: Option<String>,
    MC1R1: Option<String>,
    MC1R2: Option<String>,
    AP3B1: Option<String>,
    AP3B2: Option<String>,
    U1: Option<String>,
    U2: Option<String>,
    RW1: Option<String>,
    RW2: Option<String>,
    SPL1: Option<String>,
    SPL2: Option<String>,
    RN1: Option<String>,
    RN2: Option<String>,
    FZ1: Option<String>,
    FZ2: Option<String>
}

pub async fn run_phenotypes() {
  let text = fs::read_to_string(FILE_PATH)
    .expect("Should be able to read file");

  let mut reader = csv::Reader::from_reader(text.as_bytes());
  let mut row_id: u32 = 2;

  let pool = db::connect_db().await.expect("Failed to connect DB");

  for record in reader.deserialize() {
      let record: PhenotypeFull = record.expect("Failed to unwrap row");

      println!(
          "Row {} | {:#?}",
          row_id,
          record
      );
      row_id += 1;

      let phenotype = common::SimplePhenotype {
        phenotype: record.FENOTYP.clone(),
        variant: record.WARIANT.clone()
      };
      // Let's assume it worked or phenotype was already in DB.
      let _ = db::insert::phenotype(&phenotype, &pool).await;
      
      let genes_json = serde_json::to_value(&record).unwrap();
      let mut genes_opt: HashMap<String, Option<String>> = serde_json::from_value(genes_json).unwrap();
      genes_opt.remove("WARIANT");
      genes_opt.remove("FENOTYPE");
      let mut genes: HashMap<String, String> = HashMap::new();

      for (key, value) in genes_opt {
        if let Some(gene) = value {
          genes.insert(key, gene);
        }
      }

      let phenotype = common::Phenotype {
        phenotype: record.FENOTYP,
        variant: record.WARIANT,
        genes: genes
      };
      db::insert::genes(&phenotype, &pool).await.unwrap();
  }

  //let none: Option<()> = None;
  //none.expect("Fail after reading from cvs");
}