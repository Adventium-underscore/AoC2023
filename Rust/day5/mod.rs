use std::{fs, time};

struct SeedGroup {
    lowerBound: i64,
    upperBound: i64
}
impl SeedGroup {
    fn newFromLen(start: i64, len: i64) -> SeedGroup {
        return SeedGroup {
            lowerBound: start,
            upperBound: start + len - 1
        }
    }
    
    fn new(lower: i64, upper: i64) -> SeedGroup {
        return SeedGroup {
            lowerBound: lower,
            upperBound: upper
        }
    }
    
    fn offset(&self, offset: i64) -> SeedGroup {
        return SeedGroup {
            lowerBound: self.lowerBound + offset,
            upperBound: self.upperBound + offset
        }
    }
}

struct Mapping {
    lowerBound: i64,
    upperBound: i64,
    offset: i64
}
impl Mapping {
    fn new(startD: i64, startS: i64, len: i64) -> Mapping {
        return Mapping { 
            lowerBound: startS,
            upperBound: startS + len - 1,
            offset: startD - startS
        }
    }
    
    fn contains(&self, num: i64) -> bool {
        return num >= self.lowerBound && num <= self.upperBound;
    }
}

fn parseInput(data: &String) -> (Vec<SeedGroup>, Vec<Vec<Mapping>>) {
    let (seeddata, mapdata) = data.split_once("\n").unwrap();
    let mut seedgroups = Vec::<SeedGroup>::new();
    let mut mapsets = Vec::<Vec<Mapping>>::new();
    
    let seednums = seeddata.trim().split(' ').skip(1).map(|s| s.parse().unwrap()).collect::<Vec<i64>>();
    for n in (0..seednums.len()).step_by(2) {
        seedgroups.push(SeedGroup::newFromLen(seednums[n], seednums[n+1]));
    }
    
    let mut maps = Vec::<Mapping>::new();
    for line in mapdata.lines().skip(1) {
        if line.trim().is_empty() {
            mapsets.push(maps);
            maps = Vec::<Mapping>::new();
            continue;
        }
        let mapnums = line.split(' ').collect::<Vec<&str>>();
        if mapnums.len() == 3 {
            let mut nums = mapnums.iter().map(|s| s.parse::<i64>().unwrap());
            maps.push(Mapping::new(nums.next().unwrap(), nums.next().unwrap(), nums.next().unwrap()))
        }
    }
    mapsets.push(maps);
    return (seedgroups, mapsets);
}

fn advanceSeeds(maps: &Vec<Mapping>, seedgroups: &mut Vec<SeedGroup>) -> Vec<SeedGroup> {
    let mut newSeeds = Vec::<SeedGroup>::new();
    while !seedgroups.is_empty() {
        let seeds = seedgroups.pop().unwrap();
        let mut added = false;
        for m in maps {
            if m.contains(seeds.lowerBound) {
                if m.contains(seeds.upperBound) {
                    newSeeds.push(seeds.offset(m.offset));
                    added = true;
                } else {
                    newSeeds.push(SeedGroup::new(seeds.lowerBound, m.upperBound).offset(m.offset));
                    seedgroups.push(SeedGroup::new(m.upperBound + 1, seeds.upperBound));
                    added = true;
                }
                break;
            } else if m.contains(seeds.upperBound) {
                newSeeds.push(SeedGroup::new(m.lowerBound, seeds.upperBound).offset(m.offset));
                seedgroups.push(SeedGroup::new(seeds.lowerBound, m.lowerBound - 1));
                added = true;
                break;
            }
        }
        if !added {
            newSeeds.push(seeds);
        }
    }
    return newSeeds;
}

pub fn main() {
    let start = time::Instant::now();
    let (mut seedgroups, mapsets) = parseInput(&fs::read_to_string("src/day5/Day 5 Input.txt").unwrap());
    for maps in mapsets {
        seedgroups = advanceSeeds(&maps, &mut seedgroups);
    }
    let lowest = seedgroups.iter().map(|s| s.lowerBound).min().unwrap();
    let timed = time::Instant::now() - start;
    println!("{lowest}");
    println!("Time taken: {timed:?}");
}