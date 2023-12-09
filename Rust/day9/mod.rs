use std::{fs, time};

fn getInput(data: &String) -> Vec<Vec<i32>> {
    return data.lines().map(
        |s| s.split(' ').map(
            |n| n.parse().unwrap()
        ).collect::<Vec<i32>>()
    ).collect();
}

fn findDiffList(history: &Vec<i32>) -> Vec<i32> {
    let mut dl = Vec::<i32>::new();
    for i in 1..history.len() {
        dl.push(history[i] - history[i - 1])
    }
    return dl;
}

fn isZeroes(difflist: &Vec<i32>) -> bool {
    for diff in difflist {
        if *diff != 0 {
            return false;
        }
    }
    return true;
}

fn findNextVal(history: &mut Vec<i32>) -> i32 {
    history.reverse();
    let mut nextVal = *history.last().unwrap();
    let mut dl = findDiffList(&history);
    while !isZeroes(&dl) {
        nextVal += dl.last().unwrap();
        dl = findDiffList(&dl);
    }
    return nextVal;
}

pub fn main() {
    let start = time::Instant::now();
    let oasis = getInput(&fs::read_to_string("src/day9/Day 9 Input.txt").unwrap());
    let total = oasis.into_iter().fold(0, |acc, mut h| acc + findNextVal(&mut h));
    let timed = time::Instant::now() - start;
    println!("{total}");
    println!("Time taken: {timed:?}");
}