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
        let mut seen_fishes = std::collections::HashSet::new();

        for fish in csv::Reader::from_reader(file).deserialize::<Fish>() {
            if let Some(name) = fish?.english_name {
                if seen_fishes.insert(name.clone()) {
                    random_fishes.push(name)
                }
            }
        }

        let mut rng = rand::thread_rng();

        random_fishes.shuffle(&mut rng);

        Ok(FishDataset {
            fish_names: random_fishes,
        })
    }
    pub fn random(&mut self) -> Option<String> {
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
        FishDataset::from_file(&file).expect("incorrect csv format")
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
            let name = dataset.random();
            if let Some(name) = name {
                if !seen_names.insert(name.clone()) {
                    panic!("{} already exists in the set", name)
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
            let name = dataset.random();
            if let Some(name) = name {
                assert_ne!(name, "")
            } else {
                return;
            }
        }
    }
}
