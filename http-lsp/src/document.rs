use dashmap::DashMap;
use tree_sitter::Tree;

use crate::parser::{self, HttpRequest};

#[derive(Debug, Clone)]
pub struct CachedResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub duration_ms: u128,
}

pub struct Document {
    pub content: String,
    pub tree: Option<Tree>,
}

impl Document {
    pub fn new(content: String) -> Self {
        let tree = parser::parse(&content);
        Self { content, tree }
    }

    pub fn update(&mut self, content: String) {
        self.tree = parser::parse(&content);
        self.content = content;
    }

    pub fn requests(&self) -> Vec<HttpRequest> {
        match &self.tree {
            Some(tree) => parser::find_requests(tree, &self.content),
            None => vec![],
        }
    }
}

pub struct DocumentManager {
    documents: DashMap<String, Document>,
    responses: DashMap<(String, u32), CachedResponse>,
}

impl DocumentManager {
    pub fn new() -> Self {
        Self {
            documents: DashMap::new(),
            responses: DashMap::new(),
        }
    }

    pub fn open(&self, uri: String, content: String) {
        self.documents.insert(uri, Document::new(content));
    }

    pub fn update(&self, uri: &str, content: String) {
        if let Some(mut doc) = self.documents.get_mut(uri) {
            doc.update(content);
        }
    }

    pub fn close(&self, uri: &str) {
        self.documents.remove(uri);
        self.responses.retain(|(u, _), _| u != uri);
    }

    pub fn get_requests(&self, uri: &str) -> Vec<HttpRequest> {
        self.documents
            .get(uri)
            .map(|doc| doc.requests())
            .unwrap_or_default()
    }

    pub fn get_content(&self, uri: &str) -> Option<String> {
        self.documents.get(uri).map(|doc| doc.content.clone())
    }

    pub fn cache_response(&self, uri: &str, line: u32, response: CachedResponse) {
        self.responses.insert((uri.to_string(), line), response);
    }

    pub fn get_cached_response(&self, uri: &str, line: u32) -> Option<CachedResponse> {
        self.responses.get(&(uri.to_string(), line)).map(|r| r.clone())
    }
}

impl Default for DocumentManager {
    fn default() -> Self {
        Self::new()
    }
}
