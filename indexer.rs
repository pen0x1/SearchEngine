use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn load_env_configuration() -> HashMap<String, String> {
    dotenv::dotenv().ok();
    let mut config = HashMap::new();

    if let Ok(data_directory_path) = env::var("DATA_PATH") {
        config.insert("DATA_PATH".to_string(), data_directory_path);
    }

    config
}

fn split_into_tokens<'a>(text: &'a str) -> Vec<Cow<'a, str>> {
    text.split_whitespace()
        .map(|word| Cow::Borrowed(word))
        .collect()
}

fn create_document_index(data_directory_path: &str, document_id: &str) -> io::Result<HashMap<String, Vec<String>>> {
    let document_file_path = format!("{}/{}.txt", data_directory_path, document_id);
    let file = File::open(document_file_path)?;
    let reader = BufReader::new(file);

    // Assuming an average document might contain up to 100 unique tokens for initial hashmap capacity
    let mut index = HashMap::with_capacity(100);

    let document_id_owned = document_id.to_owned();

    for line_result in reader.lines() {
        let line = line_result?;
        let tokens = split_into_tokens(&line);

        for token_cow in tokens {
            index.entry(token_cow.into_owned())
                 .or_insert_with(Vec::new)
                 .push(document_id_owned.clone());
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