use dashmap::DashMap;
use tree_sitter::Tree;

use crate::parser::{self, HttpRequest};

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
}

impl DocumentManager {
    pub fn new() -> Self {
        Self {
            documents: DashMap::new(),
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
}

impl Default for DocumentManager {
    fn default() -> Self {
        Self::new()
    }
}
