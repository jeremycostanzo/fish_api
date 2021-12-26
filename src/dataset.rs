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

#[cfg(test)]
mod tests {
    const TEST_FILE_NAME: &str = "test_rows.csv";
    const MAX_ITERATIONS: usize = 50;
    use crate::FishDataset;
    fn the_file_is_too_long() {
        panic!(
            "The test file is too large, it should contain at most {} entries",
            MAX_ITERATIONS
        )
    }

    fn create_dataset() -> FishDataset {
        let file = std::fs::File::open(TEST_FILE_NAME)
            .unwrap_or_else(|_| panic!("could not find {}", TEST_FILE_NAME));
        FishDataset::from_file(&file).expect("incorrect csv format")
    }
    #[test]
    fn read_from_file() {
        let mut dataset = create_dataset();
        assert_eq!(Some("Blue sucker".to_owned()), dataset.random_fish());
    }

    #[test]
    fn fishes_appear_only_once() {
        let mut dataset = create_dataset();

        let mut seen_names = std::collections::HashSet::new();
        for _ in 1..=MAX_ITERATIONS {
            let name = dataset.random_fish();
            if let Some(name) = name {
                assert!(seen_names.insert(name))
            } else {
                return;
            }
        }
        the_file_is_too_long();
    }

    #[test]
    fn empty_english_names_are_ignored() {
        let mut dataset = create_dataset();

        for _ in 1..=MAX_ITERATIONS {
            let name = dataset.random_fish();
            if let Some(name) = name {
                assert_ne!(name, "")
            } else {
                return;
            }
        }
        the_file_is_too_long();
    }
}
