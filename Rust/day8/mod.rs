use std::{fs, time, collections::HashMap};
use num::integer;

fn getInput(data: &String) -> (HashMap<&str, (&str, &str)>, &str, Vec<&str>) {
    let p: &[_] = &['(', ')', '\n', '\r'];
    let (instructions, raw) = data.split_once("\r\n\r\n").unwrap();
    let mut network = HashMap::<&str, (&str, &str)>::new();
    let mut starts = Vec::<&str>::new();
    for line in raw.lines() {
        let (node, rawLR) = line.split_once(" = ").unwrap();
        if node.chars().last().unwrap() == 'A' {
            starts.push(node);
        }
        let lr = rawLR.trim_matches(p).split_once(", ").unwrap();
        network.insert(node, lr);
    }
    return (network, instructions, starts);
}

fn traverseNetwork(network: &HashMap<&str, (&str, &str)>, instructions: &str, start: &str) -> u64 {
    let mut current = start;
    let mut i = 0;
    for dir in instructions.chars().into_iter().cycle() {
        if dir == 'L' {
            current = network.get(current).unwrap().0;
        } else {
            current = network.get(current).unwrap().1;
        }
        i += 1;
        if current.chars().last().unwrap() == 'Z' {
            break;
        }
    }
    return i;
}

pub fn main() {
    let start = time::Instant::now();
    let data = fs::read_to_string("src/day8/Day 8 Input.txt").unwrap();
    let (network, instructions, starts) = getInput(&data);
    let total = starts.iter().map(|s| traverseNetwork(&network, instructions, s))
                      .reduce(|acc, n| integer::lcm(n, acc)).unwrap();
    let timed = time::Instant::now() - start;
    println!("{total}");
    println!("Time taken: {timed:?}");
}