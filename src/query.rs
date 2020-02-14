use crate::index::Posting;
use std::cmp::Ordering;

pub fn process_posting_lists(matched: Vec<Vec<Posting>>) -> Vec<Posting> {
    let mut result = matched[0].clone();
    if matched.len() > 1 {
        for i in 1..matched.len() {
            result = intersect_posting_lists(result.clone(), matched[i].clone());
        }
    }
    result
}

fn intersect_posting_lists(l1: Vec<Posting>, l2: Vec<Posting>) -> Vec<Posting> {
    let (short, long): (Vec<Posting>, Vec<Posting>) = if l1.len() < l2.len() {
        (l1, l2)
    } else {
        (l2, l1)
    };
    let mut results = Vec::new();
    println!("short: {:?}\n , long : {:?}\n", short, long);
    for posting in short {
        println!("Post: {:?}\n , long : {:?}\n", posting, long);
        match horse(posting, &long) {
            Ok(res) => results.push(res),
            Err(_e) => break,
        }
    }
    results
}

fn merge_postings(mut post1: Posting, mut post2: Posting) -> Posting {
    println!("Post1: {:?}\nPost2 : {:?}\n", post1, post2);
    post1.positions.append(&mut post2.positions);
    Posting {
        doc_id: post1.doc_id,
        term_frequency: post1.term_frequency + post2.term_frequency,
        score: post1.score + post2.score,
        positions: post1.positions,
        scored: post1.scored,
    }
}

fn binary_search(
    mut l: usize,
    mut r: usize,
    posting: Posting,
    list: &[Posting],
) -> Result<Posting, &'static str> {
    while l <= r {
        let m = (l + r) / 2_usize;
        match posting.doc_id.cmp(&list[m].doc_id) {
            Ordering::Greater => l = m + 1_usize,
            Ordering::Less => r = m - 1_usize,
            Ordering::Equal => {
                return Ok(merge_postings(posting, list[m].clone()));
            }
        }
    }
    Err("Number not found")
}

fn horse(posting: Posting, list: &[Posting]) -> Result<Posting, &'static str> {
    let mut step = 1;
    if list.is_empty() {
        return Err("list length = 0");
    }
    if list[0].doc_id == posting.doc_id {
        return Ok(merge_postings(posting, list[0].clone()));
    }
    while step < list.len() && list[step].doc_id < posting.doc_id {
        step *= 2;
    }
    if step > list.len() {
        step = list.len();
    }
    match binary_search(step / 2, step, posting, list) {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}

// todo:
// fn mergePostingList(list1 : Vec<Posting>, list2 : Vec<Posting>) -> Vec<Posting>{

// }
