use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use once_cell::sync::Lazy;

const DEFAULT_INDEX_FILE: &str = "data/index.json";

#[derive(Debug, serde::Deserialize, Clone)]
struct Document {
    id: u32,
    content: String,
}

static SEARCH_CACHE: Lazy<Mutex<HashMap<String, Vec<Document>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn main() {
    dotenv::dotenv().ok();
    let query = env::var("SEARCH_QUERY").expect("Expected a SEARCH_QUERY in the environment");

    let index_file = env::var("INDEX_FILE").unwrap_or_else(|_| String::from(DEFAULT_INDEX_FILE));
    let documents = load_documents(&index_file).expect("Failed to load documents");

    let search_results = search_documents_cached(&query, &documents);
    println!("{:#?}", search_results);
}

fn load_documents(file_path: &str) -> Result<Vec<Document>, Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string(file_path)?;
    let documents: Vec<Document> = serde_json::from_str(&file_content)?;
    Ok(documents)
}

fn search_documents_cached(query: &str, documents: &[Document]) -> Vec<Document> {
    let mut cache = SEARCH_CACHE.lock().unwrap();
    if let Some(results) = cache.get(query) {
        results.clone()
    } else {
        let results = documents
            .iter()
            .filter(|doc| doc.content.to_lowercase().contains(&query.to_lowercase()))
            .cloned()
            .collect::<Vec<Document>>();
        cache.insert(query.to_string(), results.clone());
        results
    }
}