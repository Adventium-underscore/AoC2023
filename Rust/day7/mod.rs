use std::{fs, time, cmp::Ordering};


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HIGHCARD = 1,
    ONEPAIR = 2,
    TWOPAIR = 3,
    THREEOFAKIND = 4,
    FULLHOUSE = 5,
    FOUROFAKIND = 6,
    FIVEOFAKIND = 7
}
impl HandType {
    fn findType(h: &str) -> Self {
        let mut hand = h.to_string();
        let mut counts = Vec::<usize>::new();
        let mut jokers = 0;
        while !hand.is_empty() {
            let c = hand.chars().nth(0).unwrap();
            if c == 'J' {
                jokers = hand.matches(c).count();
            } else {
                counts.push(hand.matches(c).count())
            }
            hand = hand.replace(c, "");
        }
        if counts.is_empty() {
            counts.push(5);
        } else {
            counts.sort();
            *counts.last_mut().unwrap() += jokers;
        }
        return match counts.as_slice() {
            [1,1,1,1,1] => Self::HIGHCARD,
            [1,1,1,2] => Self::ONEPAIR,
            [1,2,2] => Self::TWOPAIR,
            [1,1,3] => Self::THREEOFAKIND,
            [2,3] => Self::FULLHOUSE,
            [1,4] => Self::FOUROFAKIND,
            [5] => Self::FIVEOFAKIND,
            _ => panic!("Hand type not found!")
        }
    }
}

fn getVal(c: &char) -> i32{
    return match c {
        'A' => 14,
        'K'=> 13,
        'Q'=> 12,
        'T'=> 10,
        '9'=> 9,
        '8'=> 8,
        '7'=> 7,
        '6'=> 6,
        '5'=> 5,
        '4'=> 4,
        '3'=> 3,
        '2'=> 2,
        'J'=> 1,
        _ => -1
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    hand: String,
    handtype: HandType,
    bid: usize
}
impl Hand {
    fn new(h: &str, b: usize) -> Hand {
        return Hand {
            hand: h.to_string(),
            handtype: HandType::findType(h),
            bid: b
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.handtype == other.handtype {
            for (c1, c2) in self.hand.chars().zip(other.hand.chars()) {
                if c1 != c2 {
                    let comp = getVal(&c1).cmp(&getVal(&c2));
                    return comp;
                }
            }
            return Ordering::Equal;
        } else {
            return self.handtype.cmp(&other.handtype);
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn getInput(data: &String) -> Vec<Hand> {
    let mut hands = Vec::<Hand>::new();
    for line in data.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        hands.push(Hand::new(hand, bid.parse().unwrap()))
    }
    return hands;
}

pub fn main() {
    let start = time::Instant::now();
    let mut hands = getInput(&fs::read_to_string("src/day7/Day 7 Input.txt").unwrap());
    hands.sort();
    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (i + 1);
    }
    let timed = time::Instant::now() - start;
    println!("{winnings}");
    println!("Time taken: {timed:?}");
}