use std::collections::HashMap;
use std::env;

const DEFAULT_INDEX_FILE: &str = "data/index.json";

#[derive(Debug, serde::Deserialize)]
struct Document {
    id: u32,
    content: String,
}

fn main() {
    dotenv::dotenv().ok();
    let query = env::var("SEARCH_QUERY").expect("Expected a SEARCH_QUERY in the environment");

    let index_file = env::var("INDEX_FILE").unwrap_or_else(|_| String::from(DEFAULT_INDEX_FILE));
    let documents = load_documents(&index_file).expect("Failed to load documents");

    let search_results = search_documents(&query, documents);
    println!("{:#?}", search_results);
}

fn load_documents(file_path: &str) -> Result<Vec<Document>, Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string(file_path)?;
    let documents: Vec<Document> = serde_json::from_str(&file_content)?;
    Ok(documents)
}

fn search_documents(query: &str, documents: Vec<Document>) -> Vec<Document> {
    documents
        .into_iter()
        .filter(|doc| doc.content.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}