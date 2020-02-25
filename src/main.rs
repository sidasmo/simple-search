pub mod index;
pub mod query;
pub mod score;
pub mod tokenizer;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let mut ii = index::InvertedIndex::new();
    const DATASET: &str = "/home/dissi/Projekte/tantivy-ir-evaluation/datasets/movies.txt";
    let f = File::open(DATASET);
    println!("start import");
    match f {
        Ok(f) => {
            let file = BufReader::new(&f);
            for line in file.lines() {
                match line {
                    Ok(line) => {
                        ii.import_document(&line);
            
                    }
                    Err(e) => println!("ERROR. Can't read line {:?}", e),
                }
            }
        }
        Err(e) => println!("ERROR. Can't open file {:?}", e),
    }
    println!("start scoring");
    ii.run_scorer();
    println!("Now it is possible to search for movie reviews");
    loop{
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let answer = ii.query_processing(&line.unwrap());
            match answer {
            Some(docs) => println!("RESULT {:#?}", docs),
            None => println!("we can't found documents who match this query"),
    }
        }
    }
    
}
