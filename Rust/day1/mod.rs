use std::{fs, collections::HashMap, time};
use regex::Regex;

pub fn main() {
    let start = time::Instant::now();
    let first = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let last = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let toInt = HashMap::<&str, i32>::from([("one",1), ("two",2), ("three",3), ("four",4), ("five",5), ("six",6), ("seven",7), ("eight",8), ("nine",9)]);
    
    let data = fs::read_to_string("src/Day1/Day 1 Input.txt").expect("oop");
    let mut calibration = 0;
    for line in data.lines() {
        let Some(tensR) = first.captures(line) else {
            println!("no match!");
            return;
        };
        let Some(onesR) = last.captures(line) else {
            println!("no match!");
            return;
        };
        calibration += 10*(if tensR[0].len() == 1 {tensR[0].parse::<i32>().unwrap()} else {toInt[&tensR[0]]})
                         + if onesR[1].len() == 1 {onesR[1].parse::<i32>().unwrap()} else {toInt[&onesR[1]]};
    }
    let timed = time::Instant::now() - start;
    println!("{calibration:?}");
    println!("Time taken: {timed:?}");
}