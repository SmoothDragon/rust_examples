use std::io::{ BufRead, BufReader };
use std::collections::HashMap;
use itertools::Itertools;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut best = HashMap::<char, u8>::new();
    let filename = "WOW24.txt";

    let reader = BufReader::new(std::fs::File::open(filename).expect("open failed"));
    // let mut char_counts = HashMap::<char, u8>::new();


    for line in reader.lines().flatten() {
        let mut char_counts = HashMap::<char, u8>::new();
        // char_counts.clear();
        for ch in line.chars() {
            char_counts.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
        }

        for (key, value) in char_counts.into_iter() {
            let _ = *best.entry(key).and_modify(|max| *max = std::cmp::max(*max, value)).or_insert(value);
        }

    }

    println!("Maximum times a character is used in a word:");
    for ch in 'A'..='Z' {
        print!("{}:{} ", ch, best[&ch]);
    }
    println!();

    // Inverse of Hashmap
    let mut invmap = HashMap::<u8, Vec<char>>::new();
    for (key, value) in best.into_iter() {
        let _ = *invmap.entry(value).and_modify(|v| v.push(key)).or_insert(vec![key]);
    }
    println!("\nSorted by counts:");
    for key in invmap.keys().sorted() {
        println!("{}:{:?}", key, invmap[key]);
    }
    Ok(())
}
