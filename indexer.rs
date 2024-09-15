use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Load environment configuration, specifically looking for a data path.
fn load_env_configuration() -> HashMap<String, String> {
    dotenv::dotenv().ok(); // Load .env file, if present.
    let mut config = HashMap::new();

    if let Ok(data_directory_path) = env::var("DATA_PATH") {
        config.insert("DATA_PATH".to_string(), data_directory_path);
    }

    config
}

// Splits a string into lowercase words.
fn split_into_tokens(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| word.to_lowercase())
        .collect()
}

// Creates an inverted index for a single document located at the specified path.
fn create_document_index(data_directory_path: &str, document_id: &str) -> io::Result<HashMap<String, Vec<String>>> {
    let document_file_path = format!("{}/{}.txt", data_directory_path, document_id);
    let file = File::open(document_file_path)?;
    let reader = BufReader::new(file);

    let mut index = HashMap::new();
    
    for line in reader.lines() {
        let line = line?;
        let tokens = split_into_tokens(&line);

        for token in tokens {
            index.entry(token).or_insert_with(Vec::new).push(document_id.to_string());
        }
    }

    Ok(index)
}

fn main() -> io::Result<()> {
    let config = load_env_configuration();
    let data_path = config.get("DATA_PATH").expect("DATA_PATH not configured");

    let document_index = create_document_index(data_path, "example_doc")?;

    for (token, document_ids) in document_index {
        println!("Token: {}, Document IDs: {:?}", token, document_ids);
    }

    Ok(())
}