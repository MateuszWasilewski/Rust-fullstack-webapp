use serde::{Deserialize, Serialize};
use std::fs;

static FILE_PATH: &str = "files/db/baza-myszy.csv";
static NEW_FILE_PATH: &str = "files/db/nowa-baza-myszy.csv";

#[derive(Deserialize, Serialize, Debug)]
struct AnimalFull {
  nr_miotu: String,
  //number_osobnika: String,
  nazwa: String,
}

pub async fn run_animals() {
  let text = fs::read_to_string(FILE_PATH)
    .expect("Should be able to read file");

  let mut reader = csv::Reader::from_reader(text.as_bytes());
  let mut writer = csv::WriterBuilder::default()
    .has_headers(true)
    .from_path(NEW_FILE_PATH)
    .unwrap();
  let mut row_id: u32 = 0;

  //let pool = db::connect_db().await.expect("Failed to connect DB");

  for record in reader.records() {
    row_id += 1;
    let record = record.unwrap();

    println!(
      "Row {} | {:#?}",
      row_id,
      &record[3]
    );
    if !record[3].is_empty() {
      writer.write_record(&record).unwrap();
    }
  }
  writer.flush().unwrap();
}