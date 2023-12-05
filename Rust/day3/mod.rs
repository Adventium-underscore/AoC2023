use std::{fs, collections::HashSet};
use regex::Regex;

fn createSchematic(data: &String) -> Vec<String> {
    let mut schematic = data.lines().map(|s| ".".to_owned() + s.trim() + ".").collect::<Vec<String>>();
    schematic.insert(0, ".".repeat(schematic[0].len()));
    schematic.push(".".repeat(schematic[0].len()));
    return schematic;
}

fn getNumber(line: &String, x: &usize, re: &Regex) -> Option<i32> {
    for m in re.find_iter(line) {
        if m.range().contains(&x) {
            return Some(m.as_str().parse().unwrap());
        }
    }
    return None;
}

fn findRatio(schematic: &Vec<String>, x: &usize, y: &usize, re: &Regex) -> i32 {
    let mut nums = HashSet::<i32>::new();
    for yi in y-1..y+2 {
        nums.extend((x-1..x+2).filter(|xi| schematic[yi].chars().nth(*xi).unwrap().is_numeric())
                              .map(|xi| getNumber(&schematic[yi], &xi, re).unwrap()));
    }
    if nums.len() == 2 {
        let mut ni = nums.iter();
        return ni.next().unwrap() * ni.next().unwrap();
    }
    return 0
}

pub fn main() {
    let re = Regex::new(r"\d+").unwrap();
    let schematic = createSchematic(&fs::read_to_string("src/day3/Day 3 Input.txt").expect("oop"));
    let mut total = 0;
    for (y, line) in schematic.iter().enumerate() {
        total += line.match_indices('*').map(|m| findRatio(&schematic, &m.0, &y, &re)).sum::<i32>();
    }
    println!("{total}");
}