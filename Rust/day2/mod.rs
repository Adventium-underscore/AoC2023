use std::{fs, collections::HashMap, cmp};
use regex::Regex;

fn splitHand(hand: &str, re: &Regex) -> (i32, i32, i32) {
    let mut colors = (0, 0, 0);
    for h in re.captures_iter(hand) {
        let n = h.get(1).unwrap().as_str().parse::<i32>().unwrap();
        match h.get(2).unwrap().as_str() {
            "red" => colors.0 = cmp::max(colors.0, n),
            "green" => colors.1 = cmp::max(colors.1, n),
            "blue" => colors.2 = cmp::max(colors.2, n),
            _ => (),
        }
    }
    return colors;
}

pub fn main() {
    let re = Regex::new(r"(\d+) (\w+)").unwrap();
    let mut games = HashMap::<i32, (i32, i32, i32)>::new();
    
    let data = fs::read_to_string("src/day2/Day 2 Input.txt").expect("oop");
    for game in data.lines() {
        let g = game.strip_prefix("Game ").unwrap().split_once(": ").unwrap();
        let gameID = g.0.parse::<i32>().unwrap();
        let colors = splitHand(g.1, &re);
        games.insert(gameID, colors);
    }
    let total:i32 = games.into_iter().map(|g| g.1.0*g.1.1*g.1.2).sum();
    println!("{total}");
}