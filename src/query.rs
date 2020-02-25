use crate::index::Posting;
use std::cmp;
use std::cmp::Ordering;

// TODO: avoid clones
pub fn process_posting_lists(matched: Vec<Vec<Posting>>, conjunction: bool) -> Vec<Posting> {
    let mut result = matched[0].clone();
    if matched.len() > 1 {
        for post in matched {
            if conjunction {
              result = intersect_posting_lists(&result, &post);
            }
            else {
              result = merge_posting_lists(&result, &post);
            }
        }
    }
    result.sort_by(|a,b| b.score.partial_cmp(&a.score).unwrap());
    if result.len() > 10 {
      result[1..10].to_vec()
    }
    else{
      result
    } 
    
}

fn intersect_posting_lists(l1: &[Posting], l2: &[Posting]) -> Vec<Posting> {
    let (short, long): (&[Posting], &[Posting]) = if l1.len() < l2.len() {
        (l1, l2)
    } else {
        (l2, l1)
    };
    let mut results = Vec::new();
    for posting in short {
        match horse(posting, &long) {
            Some(res) => results.push(res),
            None => break,
        }
    }
    results
}

fn merge_postings(post1: &Posting, post2: &Posting) -> Posting {
    if post1.dl != post2.dl {
        println!("WARNING! Different Document length");
    }
    Posting {
        doc_id: post1.doc_id,
        dl: post1.dl,
        term_frequency: post1.term_frequency + post2.term_frequency,
        score: post1.score + post2.score,
    }
}

fn binary_search(
    mut l: usize,
    mut r: usize,
    posting: &Posting,
    list: &[Posting],
) -> Option<Posting> {
    while l <= r {
        let m = (l + r) / 2_usize;
        match posting.doc_id.cmp(&list[m].doc_id) {
            Ordering::Greater => l = m + 1_usize,
            Ordering::Less => r = m - 1_usize,
            Ordering::Equal => {
                return Some(merge_postings(posting, &list[m]));
            }
        }
    }
None
}

fn horse(posting: &Posting, list: &[Posting]) -> Option<Posting> {
        let mut step = 1_usize;
        while step<list.len() && list[step].doc_id < posting.doc_id {
            step *=2
        }
    
        binary_search(step / 2, cmp::min(step +1, list.len()), posting, list) 
    
}


fn merge_posting_lists(l1 : &[Posting], l2 : &[Posting]) -> Vec<Posting>{
  // Zipper Algorithm
  let (short, long): (&[Posting], &[Posting]) = if l1.len() < l2.len() {
    (l1, l2)
} else {
    (l2, l1)
};   
    let mut results = Vec::new();
    let mut i_s = 0;
    let mut i_l = 0;
    while i_s < short.len() && i_l < long.len() {
        match short[i_s].doc_id.cmp(&long[i_l].doc_id) {
            Ordering::Greater => {
              results.push(long[i_l].clone());
              i_l += 1
            },
            Ordering::Less => {
              results.push(short[i_s].clone());
              i_s += 1
            },
            Ordering::Equal => {
                let merged = merge_postings(&short[i_s], &long[i_l]);
                results.push(merged);
                i_s += 1;
                i_l += 1;
            }
        }
    }
    while i_s < short.len() {
      results.push(short[i_s].clone());
      i_s += 1;
    }
    while i_l < long.len() {
      results.push(long[i_l].clone());
      i_l += 1;
    }
    results
}
