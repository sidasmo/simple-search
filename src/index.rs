use crate::query::process_posting_lists;
use crate::score::scorer;
use crate::tokenizer::tokenize_text;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct InvertedIndex {
    inverted_lists: HashMap<String, Vec<Posting>>,
    pub number_of_documents: u32,
    pub number_of_terms: u32,
    active: bool,
}

#[derive(Debug)]
pub struct Document {
    doc_id: u32,
    postings: Vec<Posting>,
    document_length: u32,
}


// Implement doc-store and let Posting be a tuple of doc_id and score
#[derive(Debug, Clone)]
pub struct Posting {
    pub doc_id: u32,
    pub dl: usize,
    pub term_frequency: u32,
    pub score: f32,
}

#[derive(Debug)]
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
    pub fn new(doc_id: u32, dl: usize) -> Self {
        Posting {
            doc_id,
            dl,
            term_frequency: 1,
            score: 0.0,
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
    pub fn run_scorer(&mut self) {
        for mut posting_list in &mut self.inverted_lists {
            scorer(
                self.number_of_documents as f32,
                self.number_of_terms as f32,
                &mut posting_list.1,
            );
        }
    }

    pub fn set_active(mut self) {
        self.active = true
    }

    pub fn set_index_inactive(mut self) {
        self.active = false
    }

    pub fn query_processing(&mut self, query: &str) -> Option<Vec<Posting>> {
        // tokenize the query in the same way you tokenize the documents
        if query.is_empty() {
            return None;
        };
        let tokenized_query = tokenize_text(query);

        let mut matched: Vec<Vec<Posting>> = Vec::new();
        for term in tokenized_query {
            matched.push(self.inverted_lists[&term].clone());
        }

        Some(process_posting_lists(matched, false))
    }

    pub fn import_document(&mut self, text: &str) {
        self.number_of_documents += 1;
        let tokenized_text = tokenize_text(text);
        let dl = tokenized_text.len();
        for term in tokenized_text {
            self.number_of_terms += 1;
            self.import_posting(self.number_of_documents, dl, term);
        }
    }

    fn import_posting(&mut self, doc_id: u32, dl: usize, term: String) {
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
                posting_list.push(Posting::new(doc_id, dl));
            }
        // Case 2: Term not in index.
        } else {
            let mut posting_list = Vec::new();
            posting_list.push(Posting::new(doc_id, dl));
            self.inverted_lists.insert(term, posting_list);
        }
    }
}
