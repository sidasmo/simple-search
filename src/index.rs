use crate::query::process_posting_lists;
use crate::tokenizer::tokenize_text;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct InvertedIndex {
    inverted_lists: HashMap<String, Vec<Posting>>,
    number_of_documents: u32,
    number_of_terms: u32,
    active: bool,
}

#[derive(Debug, Clone)]
pub struct Document {
    doc_id: u32,
    postings: Vec<Posting>,
    document_length: u32,
}

#[derive(Debug, Clone)]
pub struct Posting {
    pub doc_id: u32,
    pub term_frequency: u32,
    pub score: f32,
    pub positions: Vec<f32>,
    pub scored: bool,
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    doc_id: u32,
    score: f32,
    positions: Vec<f32>,
}

impl QueryResult {
    pub fn new(doc_id: u32, score: f32, positions: Vec<f32>) -> Self {
        QueryResult {
            doc_id,
            score,
            positions,
        }
    }
}

impl Posting {
    pub fn new(doc_id: u32) -> Self {
        Posting {
            doc_id,
            term_frequency: 1,
            score: 0.0,
            positions: vec![],
            scored: false,
        }
    }
}

impl InvertedIndex {
    pub fn new() -> Self {
        let lists = HashMap::new();
        InvertedIndex {
            inverted_lists: lists,
            number_of_documents: 0_u32,
            number_of_terms: 0_u32,
            active: true,
        }
    }
    pub fn set_active(mut self) {
        self.active = true
    }

    pub fn set_index_inactive(mut self) {
        self.active = false
    }

    pub fn query_processing(self, query: &str) -> Option<Vec<Posting>> {
        // tokenize the query in the same way you tokenize the documents
        if query.is_empty() {
            return None;
        };
        let tokenized_query = tokenize_text(query);

        let mut matched: Vec<Vec<Posting>> = Vec::new();
        for term in tokenized_query {
            matched.push(self.inverted_lists[&term].clone());
        }

        //todo: intersect and merge functions for posting_lists
        Some(process_posting_lists(matched))
    }

    pub fn import_document(&mut self, doc_id: u32, text: &str) {
        let tokenized_text = tokenize_text(text);
        for term in tokenized_text {
            self.import_posting(doc_id, term.clone());
        }
    }

    fn import_posting(&mut self, doc_id: u32, term: String) {
        // Case 1: Term in index.
        if let Some(posting_list) = self.inverted_lists.get_mut(&term) {
            // Case 1A: doc_id is in posting list for term, so increment term frequency.
            if let Some(posting) = posting_list
                .iter_mut()
                .find(|posting| posting.doc_id == doc_id)
            {
                posting.term_frequency += 1;
            // Case 1B: doc_id is not in posting list, add posting.
            } else {
                posting_list.push(Posting::new(doc_id));
            }
        // Case 2: Term not in index.
        } else {
            let mut posting_list = Vec::new();
            posting_list.push(Posting::new(doc_id));
            self.inverted_lists.insert(term, posting_list);
        }
    }
}
