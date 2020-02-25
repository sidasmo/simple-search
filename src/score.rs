use crate::index::{Posting};

fn bm25(m_idf : f32, tf: f32, b: f32, k: f32, dl: f32, avdl: f32) -> f32 {
    m_idf * ((tf * (k + 1_f32)) / (tf + k * (1_f32 - b + (b * dl / avdl))))
}
fn idf(num_of_docs : f32 , posting_list_length : f32) -> f32{
  (num_of_docs/posting_list_length).ln()
}
pub fn scorer(num_of_terms: f32, num_of_docs: f32, postings: &mut Vec<Posting>) {
   let m_idf = idf(num_of_docs, postings.len() as f32); 
   for mut posting in postings {
        posting.score = bm25(
            m_idf,
            posting.term_frequency as f32,
            0.75,
            1.2,
            posting.dl as f32,
            num_of_terms / num_of_docs,
        );
    }
}
