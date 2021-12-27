mod dataset;
mod images;

use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer};
use dataset::FishDataset;
use std::sync::Mutex;

struct AppState {
    fish_dataset: Mutex<FishDataset>,
}

async fn serve_fish(data: web::Data<AppState>) -> HttpResponse {
    let mut dataset = data.fish_dataset.lock().unwrap();
    println!("remaining fishes: {}", dataset.remaining());
    match dataset.random() {
        Some(fish) => {
            let image = images::get_url(&fish).await;
            let mut return_string = format!("<div>{}</div>", fish);
            if let Ok(Some(image_url)) = image {
                return_string.push_str(&format!("\n<img src=\"{}\">", image_url));
            }
            HttpResponse::build(StatusCode::OK)
                .content_type("text/html; charset=utf-8")
                .body(return_string)
        }
        None => HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body("No more fishes".to_owned()),
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
