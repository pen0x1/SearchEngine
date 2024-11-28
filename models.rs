use std::collections::HashMap;
use dotenv::dotenv;
use std::env;

fn init_env() {
    dotenv().ok();
}

#[derive(Debug, Clone)]
struct Document {
    id: String,
    title: String,
    content: String,
    metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct SearchResult {
    document_id: String,
    score: f32,
    metadata: HashMap<String, String>,
}

struct Index {
    inverted_index: HashMap<String, Vec<String>>,
}

impl Index {
    fn new() -> Index {
        Index {
            inverted_index: HashMap::new(),
        }
    }

    fn add_document(&mut self, doc: Document) {
        let doc_id = doc.id.clone();
        let text = doc.title + " " + &doc.content;
        for word in text.split_whitespace() {
            self.inverted_index
                .entry(word.to_string().to_lowercase())
                .or_default()
                .push(doc_id.clone());
        }
    }

    fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results: Vec<SearchResult> = Vec::new();
        for word in query.split_whitespace() {
            if let Some(doc_ids) = self.inverted_index.get(&word.to_lowercase()) {
                for id in doc_ids {
                    results.push(SearchResult {
                        document_id: id.clone(),
                        score: 1.0,
                        metadata: HashMap::new(),
                    });
                }
            }
        }

        results
    }
}

fn main() {
    init_env();

    let mut index = Index::new();
    let doc1 = Document {
        id: "1".to_string(),
        title: "The Title".to_string(),
        content: "The quick brown fox jumps over the lazy dog".to_string(),
        metadata: HashMap::new(),
    };

    index.add_document(doc1);

    let search_results = index.search("quick fox");
    println!("{:?}", search_results);
}