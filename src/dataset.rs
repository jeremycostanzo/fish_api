use serde::Deserialize;

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

pub fn read(
    file: &std::fs::File,
) -> std::result::Result<std::vec::Vec<std::string::String>, csv::Error> {
    let mut fishes = Vec::new();

    for fish in csv::Reader::from_reader(file).deserialize::<Fish>() {
        if let Some(name) = fish?.english_name {
            fishes.push(name)
        }
    }

    Ok(fishes)
}
