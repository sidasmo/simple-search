use crate::tokenizer::tokenize_text;
use std::cmp::Ordering;
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
    doc_id: u32,
    term_frequency: u32,
    score: f32,
    positions: Vec<f32>,
    scored: bool,
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
            //todo: intersect and merge functions for posting_lists
        }
        Some(intersect_posting_lists(matched))
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

fn intersect_posting_lists(matched: Vec<Vec<Posting>>) -> Vec<Posting> {
    let mut result = matched[0].clone();
    if matched.len() > 1 {
        for element in matched {
            result = intersect(result.clone(), element.clone());
        }
    } else {
        result.extend(matched[0].clone());
    }
    fn intersect(l1: Vec<Posting>, l2: Vec<Posting>) -> Vec<Posting> {
        let (short, long): (Vec<Posting>, Vec<Posting>) = if l1.len() < l2.len() {
            (l1, l2)
        } else {
            (l2, l1)
        };
        let mut results = Vec::new();
        let mut i_s = 0;
        let mut i_l = 0;
        //todo: implement galopping search for faster intersection
        while i_s < short.len() && i_l < long.len() {
            match short[i_s].doc_id.cmp(&long[i_l].doc_id) {
                Ordering::Greater => i_l += 1,
                Ordering::Less => i_s += 1,
                Ordering::Equal => {
                    results.push(combine_postings(short[i_s].clone(), long[i_l].clone()));
                    i_s += 1;
                    i_l += 1;
                }
            }
        }
        results
    }
    result
}

fn combine_postings(post1: Posting, post2: Posting) -> Posting {
    Posting {
        doc_id: post1.doc_id,
        term_frequency: post1.term_frequency + post2.term_frequency,
        score: post1.score + post2.score,
        //todo: combine positionsvector
        positions: post1.positions,
        scored: post1.scored,
    }
}

// todo:
// fn mergePostingList(list1 : Vec<Posting>, list2 : Vec<Posting>) -> Vec<Posting>{

// }
