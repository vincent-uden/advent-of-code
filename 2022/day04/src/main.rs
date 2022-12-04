use std::{
    cmp::{max, min},
    fs::{self, File},
    io::{self, BufRead},
};

fn part_1() -> i32 {
    let file = File::open("./input.txt").unwrap();

    let mut contained = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let elves: Vec<&str> = l.split(",").collect();
            let e1_range: Vec<i32> = elves[0]
                .split("-")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let e2_range: Vec<i32> = elves[1]
                .split("-")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            if e1_range[0] <= e2_range[0] && e1_range[1] >= e2_range[1] {
                contained += 1;
            } else if e2_range[0] <= e1_range[0] && e2_range[1] >= e1_range[1] {
                contained += 1;
            }
        }
    }

    return contained;
}

fn part_2() -> i32 {
    let file = File::open("./input.txt").unwrap();

    let mut overlapping = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let elves: Vec<&str> = l.split(",").collect();
            let e1_range: Vec<i32> = elves[0]
                .split("-")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let e2_range: Vec<i32> = elves[1]
                .split("-")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            if e1_range[1] - e1_range[0] + e2_range[1] - e2_range[0] + 2
                > max(e1_range[1], e2_range[1]) - min(e1_range[0], e2_range[0]) + 1
            {
                overlapping += 1;
            }
        }
    }

    return overlapping;
}

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}
