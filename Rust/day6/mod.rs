use std::{fs, time};

pub fn main() {
    let start = time::Instant::now();
    let raw = fs::read_to_string("src/day6/Day 6 Input.txt").unwrap();
    let mut data = raw.lines();
    let time = data.next().unwrap().split_once(':').unwrap().1.replace(' ', "").parse::<f64>().unwrap();
    let distance = data.next().unwrap().split_once(':').unwrap().1.replace(' ', "").parse::<f64>().unwrap();
    let lb = ((-time + (time * time - 4.0 * distance).sqrt()) / -2.0).ceil();
    let wins = time - lb - lb + 1.0;
    let timed = time::Instant::now() - start;
    println!("{wins}");
    println!("Time taken: {timed:?}");
}