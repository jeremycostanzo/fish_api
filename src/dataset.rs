use serde::Deserialize;

use rand::prelude::*;

#[derive(Debug, Deserialize)]
struct Fish {
    // ISSCAAP: String,
    // TAXOCODE: String,
    // 3A_CODE: String,
    // Scientific_name: String,
    // Fishes can have a scientific name but no english name for instance, we want to skip them
    #[serde(rename = "English_name")]
    english_name: Option<String>,
    // French_name: String,
    // Spanish_name: String,
    // Arabic_name: String,
    // Chinese_name: String,
    // Russian_name: String,
    // Author: String,
    // Family: String,
    // Order: String,
    // Stats_data : String,
}

pub struct FishDataset {
    fish_names: Vec<String>,
}

impl FishDataset {
    pub fn from_file(file: &std::fs::File) -> std::result::Result<Self, csv::Error> {
        let mut random_fishes = Vec::new();

        for fish in csv::Reader::from_reader(file).deserialize::<Fish>() {
            if let Some(name) = fish?.english_name {
                random_fishes.push(name)
            }
        }

        let mut rng = rand::thread_rng();

        random_fishes.shuffle(&mut rng);

        Ok(FishDataset {
            fish_names: random_fishes,
        })
    }
    pub fn random_fish(&mut self) -> Option<String> {
        self.fish_names.pop()
    }
}
