use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use itertools::Itertools;

fn main() {
    println!("Processing hidden-message-data.input:");
    hidden_message_harness("hidden-message-data.input");
}

// side-effecting
fn hidden_message_harness(fname: &str) {
    let fpath = Path::new(fname);
    let file = match File::open(&fpath) {
        Err(why) => panic!("couldn't open {}, {:?}", fpath.display(), why),
        Ok(file) => file,
    };

    // expect two lines, first the data and second the target kmer
    let mut rdr = io::BufReader::new(file);

    loop {
        match read_two_lines(&mut rdr) {
            Some((seq, kmer)) => {
                let maxlen = if seq.len() > 7 { 7 } else { seq.len() };
                println!("given {} looking for {} found {} instances", &seq[0..maxlen], kmer, hidden_message(&seq, &kmer));
            },
            _ => {
                println!("failed to read two lines from input");
                break;
            }
        }
    }


    
}

fn read_two_lines(rdr: &mut io::BufReader<File>) -> Option<(String, String)> {
    let mut first = String::new();
    let mut second = String::new();

    let r1 = rdr.read_line(&mut first); 
    let r2 = rdr.read_line(&mut second);

    match (r1, r2) {
        (Ok(n1), Ok(n2)) if n1 > 0 && n2 > 0 => {
            Some((first[0..(first.len()-1)].to_string(), second[0..(second.len()-1)].to_string()))
        }
        _ => None
    }
        
    


}

fn hidden_message(seq: &str, kmer: &str) -> u32 {
    let mut count = 0;
    for i in 0..=(seq.len() - kmer.len()) {
        let window = &seq[i..(i+kmer.len())];
        if *window == *kmer { count += 1; }
    }
    
    count
}

