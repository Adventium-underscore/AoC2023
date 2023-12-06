use std::{collections::HashSet, fs, time};
use regex::Regex;

fn getCards(data: &String) -> Vec<(usize, usize)> {
    let re = Regex::new(r"\d+").unwrap();
    let mut cards = Vec::<(usize, usize)>::new();
    for line in data.lines() {
        let raw = line.trim().split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning = re.find_iter(raw.0).map(|s| s.as_str().parse::<i32>().unwrap()).collect::<HashSet<i32>>();
        let nums = re.find_iter(raw.1).map(|s| s.as_str().parse::<i32>().unwrap()).collect::<HashSet<i32>>();
        cards.push((winning.intersection(&nums).count(), 1));
    }
    return cards;
}

pub fn main() {
    let start = time::Instant::now();
    let mut cards = getCards(&fs::read_to_string("src/day4/Day 4 Input.txt").unwrap());
    // let mut total = 0;
    // for i in 0..cards.len() {
    //     for x in i..i+cards[i].0 {
    //         cards[x+1].1 += cards[i].1;
    //     }
    //     total += cards[i].1;
    // }
    let total = (0..cards.len()).fold(0, |acc, i| {
        for x in i..i+cards[i].0 {
            cards[x+1].1 += cards[i].1;
        }
        acc + cards[i].1
    });
    let timed = time::Instant::now() - start;
    println!("{total}");
    println!("Time taken: {timed:?}");
}