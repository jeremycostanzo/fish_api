mod dataset;
mod images;

use actix_web::{web, App, HttpServer, Responder};
use dataset::FishDataset;
use serde::Serialize;
use std::{fs::OpenOptions, sync::Mutex, time::Instant};

struct AppState {
    fish_dataset: Mutex<FishDataset>,
}

#[derive(Serialize)]
struct FishData {
    english_name: String,
    image_url: Option<String>,
}

async fn serve_fish(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let mut dataset = data.fish_dataset.lock().expect("Could not lock dataset");
    println!("remaining fishes: {}", dataset.remaining());
    let json = match dataset.random() {
        Some(fish) => {
            let image_url = images::get_url(&fish.english_name).await;
            if let Err(error) = &image_url {
                eprintln!("Could not download image: {}", error)
            }
            Ok(web::Json(Some(FishData {
                english_name: fish.english_name,
                image_url: image_url.ok().flatten(),
            })))
        }
        None => Ok(web::Json(None)),
    };
    dataset.overwrite();
    json
}

const DATASET_FILE_NAME: &str = "dataset.csv";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let start_time = Instant::now();

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DATASET_FILE_NAME)
        .unwrap_or_else(|_| panic!("Could not open file {}", DATASET_FILE_NAME));

    let random_fishes = web::Data::new(AppState {
        fish_dataset: Mutex::new(FishDataset::from_file(file)?),
    });

    println!("Fishes read in {}ms", start_time.elapsed().as_millis());

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            // Note: using app_data instead of data
            .app_data(random_fishes.clone()) // <- register the created data
            .route("/", web::get().to(serve_fish))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
