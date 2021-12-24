mod dataset;

use actix_web::{web, App, HttpServer};
use rand::prelude::*;
use std::sync::Mutex;

struct AppState {
    fishes: Mutex<Vec<String>>,
}

async fn index(data: web::Data<AppState>) -> String {
    let mut fishes = data.fishes.lock().unwrap();
    match fishes.pop() {
        Some(fish) => fish,
        None => "No more fishes".to_owned(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("dataset.txt")?;
    let mut rng = rand::thread_rng();

    let mut random_fishes = dataset::read(&file)?;
    random_fishes.shuffle(&mut rng);

    let random_fishes = web::Data::new(AppState {
        fishes: Mutex::new(random_fishes),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            // Note: using app_data instead of data
            .app_data(random_fishes.clone()) // <- register the created data
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
