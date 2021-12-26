mod dataset;

use actix_web::{web, App, HttpServer};
use dataset::FishDataset;
use std::sync::Mutex;

struct AppState {
    fish_dataset: Mutex<FishDataset>,
}

async fn serve_fish(data: web::Data<AppState>) -> String {
    let mut dataset = data.fish_dataset.lock().unwrap();
    match dataset.random_fish() {
        Some(fish) => fish,
        None => "No more fishes".to_owned(),
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
