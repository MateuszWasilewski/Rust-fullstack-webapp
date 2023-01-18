
use common::Photo;
use glob::glob;

static FILE_PATH: &str = "public/photos/**/*";

pub async fn add_photos() {
  let pool = db::connect_db().await.expect("Failed to connect DB");

  for path in glob(FILE_PATH).unwrap() {
    match path {
      Err(_) => {}
      Ok(path) => {
        if path.is_file() {
          println!("{}", path.display());
          let path_str = path.to_str().unwrap();

          let photo = Photo {
            path: path_str.into(),
            author: None,
          };
          let result = db::insert::photo(&photo, &pool).await;
          if let Err(error) = result {
            println!("{}", error);
          }

          for component in path.components(){
            let part = component.as_os_str();
            let part = part.to_str().unwrap();
            let result = db::insert::link_litter_to_photo(part, &photo.path, &pool).await;
            if let Err(error) = result {
              println!("{}", error);
            }
          }
        }
      }
    }
  }
}