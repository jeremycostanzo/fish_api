use serde::{Deserialize, Serialize};
use std::io::{Seek, SeekFrom};

use rand::prelude::*;

#[derive(Debug, Deserialize)]
struct CsvFish {
    // ISSCAAP: String,
    // TAXOCODE: String,
    // 3A_CODE: String,
    #[serde(rename = "Scientific_name")]
    scientific_name: Option<String>,
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

#[derive(Serialize)]
pub struct Fish {
    #[serde(rename = "Scientific_name")]
    scientific_name: Option<String>,
    #[serde(rename = "English_name")]
    pub english_name: String,
}

pub struct FishDataset {
    fish_names: Vec<Fish>,
    data_file: std::fs::File,
}

impl FishDataset {
    pub fn from_file(file: std::fs::File) -> std::result::Result<Self, csv::Error> {
        let mut random_fishes = Vec::new();
        let mut seen_fishes = std::collections::HashSet::new();

        for fish in csv::Reader::from_reader(&file).deserialize::<CsvFish>() {
            let fish = fish?;
            if let Some(name) = fish.english_name {
                if seen_fishes.insert(name.clone()) {
                    random_fishes.push(Fish {
                        english_name: name,
                        scientific_name: fish.scientific_name,
                    })
                }
            }
        }

        let mut rng = rand::thread_rng();

        random_fishes.shuffle(&mut rng);

        Ok(FishDataset {
            fish_names: random_fishes,
            data_file: file,
        })
    }

    pub fn overwrite(&mut self) {
        // We want to clear the file's contents before writing into it
        self.data_file
            .set_len(0)
            .expect("File not open for writing");
        let new_position = &self.data_file.seek(SeekFrom::Start(0)).unwrap();
        println!("new cursor position: {}", new_position);
        let mut writer = csv::Writer::from_writer(&self.data_file);

        for row in self.fish_names.iter() {
            writer.serialize(row).expect("Csv serialization failure")
        }
    }

    pub fn random(&mut self) -> Option<Fish> {
        self.fish_names.pop()
    }

    pub fn remaining(&self) -> usize {
        self.fish_names.len()
    }
}

#[cfg(test)]
mod tests {
    const TEST_FILE_NAME: &str = "dataset.csv";
    use crate::FishDataset;

    fn create_dataset() -> FishDataset {
        let file = std::fs::File::open(TEST_FILE_NAME)
            .unwrap_or_else(|_| panic!("could not find {}", TEST_FILE_NAME));
        FishDataset::from_file(file).expect("incorrect csv format")
    }
    #[test]
    fn read_from_file() {
        let mut dataset = create_dataset();
        if dataset.random().is_none() {
            panic!("the file should not be empty and there is no fish returned")
        }
    }

    #[test]
    fn fishes_appear_only_once() {
        let mut dataset = create_dataset();

        let mut seen_names = std::collections::HashSet::new();
        loop {
            let fish = dataset.random();
            if let Some(fish) = fish {
                if !seen_names.insert(fish.english_name.clone()) {
                    panic!("{} already exists in the set", fish.english_name)
                }
            } else {
                return;
            }
        }
    }

    #[test]
    fn empty_english_names_are_ignored() {
        let mut dataset = create_dataset();

        loop {
            let fish = dataset.random();
            if let Some(fish) = fish {
                assert_ne!(fish.english_name, "")
            } else {
                return;
            }
        }
    }
}
