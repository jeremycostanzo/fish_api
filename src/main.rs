mod dataset;
mod images;

use actix_web::{web, App, HttpServer, Responder};
use dataset::FishDataset;
use serde::Serialize;
use std::sync::Mutex;

struct AppState {
    fish_dataset: Mutex<FishDataset>,
}

#[derive(Serialize)]
struct FishData {
    english_name: String,
    image_url: Option<String>,
}

async fn serve_fish(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let mut dataset = data.fish_dataset.lock().unwrap();
    println!("remaining fishes: {}", dataset.remaining());
    match dataset.random() {
        Some(english_name) => {
            let image_url = images::get_url(&english_name).await;
            if let Err(error) = &image_url {
                eprintln!("Could not download image: {}", error)
            }
            Ok(web::Json(Some(FishData {
                english_name,
                image_url: image_url.ok().flatten(),
            })))
        }
        None => Ok(web::Json(None)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("dataset.csv")?;

    let random_fishes = web::Data::new(AppState {
        fish_dataset: Mutex::new(FishDataset::from_file(&file)?),
    });

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
