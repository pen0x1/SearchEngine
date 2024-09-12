use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn load_configuration() -> HashMap<String, String> {
    dotenv::dotenv().ok();
    let mut config = HashMap::new();

    if let Ok(data_path) = env::var("DATA_PATH") {
        config.insert("DATA_PATH".to_string(), data_path);
    }

    config
}

fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| word.to_lowercase())
        .collect()
}

fn index_document(data_path: &str, doc_id: &str) -> io::Result<HashMap<String, Vec<String>>> {
    let file_path = format!("{}/{}.txt", data_path, doc_id);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut index = HashMap::new();
    
    for line in reader.lines() {
        let line = line?;
        let tokens = tokenize(&line);

        for token in tokens {
            index.entry(token).or_insert_with(Vec::new).push(doc_id.to_string());
        }
    }

    Ok(index)
}

fn main() -> io::Result<()> {
    let config = load_configuration();
    let data_path = config.get("DATA_PATH").expect("DATA_PATH not configured");

    let indexed_data = index_document(data_path, "example_doc")?;

    for (token, doc_ids) in indexed_data {
        println!("Token: {}, Document IDs: {:?}", token, doc_ids);
    }

    Ok(())
}