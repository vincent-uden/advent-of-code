use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

// Two compartments
// Every item must go into one of the two compartments
// The elf that failed, missed this rule for exactly one item
//
// Every item is a letter, a != A
//
// The first half of each line is compartent 1, the second half is compartment 2
//
// Lowercase has prio 1 -> 26
// Uppercase has prio 27 -> 52
//
// For each rucksack, find the item that appears in both compartments. Sum
// the priorities of all duplicate items over all rucksacks.

fn get_prio(c: &char) -> i32 {
    let i = *c as i32;

    if i >= 97 && i <= 122 {
        return i - 96;
    }

    if i >= 65 && i <= 90 {
        return i - 65 + 27;
    } else {
        panic!("Invalid char");
    }
}

fn part_1() -> i32 {
    let file = File::open("./input.txt").unwrap();

    let mut prio = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let rucksack: Vec<char> = l.chars().collect();

            let mut c1: HashMap<char, i32> = HashMap::new();
            let mut c2: HashMap<char, i32> = HashMap::new();

            for i in 0..rucksack.len() / 2 {
                *c1.entry(rucksack[i]).or_insert(0) += 1;
                *c2.entry(rucksack[i + rucksack.len() / 2]).or_insert(0) += 1;
            }

            for (k, _) in &c1 {
                if c2.contains_key(k) {
                    prio += get_prio(k);
                }
            }
        }
    }

    return prio;
}

fn part_2() -> i32 {
    let file = File::open("./input.txt").unwrap();

    let mut prio = 0;
    let mut i = 0;

    let mut group_mask: HashSet<char> = HashSet::new();
    let mut tmp_mask: HashSet<char> = HashSet::new();

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            if i % 3 == 0 {
                group_mask.clear();

                for c in l.chars() {
                    group_mask.insert(c.clone());
                }
            } else {
                tmp_mask.clear();

                for c in l.chars() {
                    if group_mask.contains(&c) {
                        tmp_mask.insert(c.clone());
                    }
                }

                group_mask.clear();
                for c in &tmp_mask {
                    group_mask.insert(c.clone());
                }
            }

            if i % 3 == 2 {
                // There should only be one left here
                assert!(group_mask.len() == 1);
                for c in &group_mask {
                    prio += get_prio(c);
                }
            }
        }

        i += 1;
    }

    return prio;
}

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}
