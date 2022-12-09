use std::collections::HashSet;
use std::io::BufRead;
use std::iter::zip;
use std::{fs, io};

use itertools::izip;

fn part_1() -> usize {
    // (row, col)
    let mut last_head: (i32, i32) = (0, 0);
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    let file = fs::File::open("./input.txt").unwrap();

    let mut visited: HashSet<(i32,i32)> = HashSet::new();

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let inst: Vec<&str> = l.split(" ").collect();
            let step_size = inst[1].parse::<i32>().unwrap();


            for _ in 0..step_size {
                last_head = head.clone();
                match inst[0] {
                    "R" => head.1 += 1,
                    "L" => head.1 -= 1,
                    "U" => head.0 -= 1,
                    "D" => head.0 += 1,
                    _ => panic!(),
                }

                if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                    tail = last_head.clone();
                }

                visited.insert(tail);
            }
        }
    }

    return visited.len();
}

fn part_2() -> usize {
    // (row, col)
    let mut knots: Vec<(i32,i32)> = vec![];

    let file = fs::File::open("./input.txt").unwrap();

    let mut visited: HashSet<(i32,i32)> = HashSet::new();

    for i in 0..10 {
        knots.push((0,0));
    }

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let inst: Vec<&str> = l.split(" ").collect();
            let step_size = inst[1].parse::<i32>().unwrap();

            for _ in 0..step_size {
                match inst[0] {
                    "R" => knots[0].1 += 1,
                    "L" => knots[0].1 -= 1,
                    "U" => knots[0].0 -= 1,
                    "D" => knots[0].0 += 1,
                    _ => panic!(),
                }

                for i in 1..knots.len() {
                    let knot1 = knots[i-1];
                    let knot2 = knots[i];
                    let mut step = (0, 0);
                    if (knot1.0 - knot2.0) >= 1 {
                        step.0 = 1;
                    } else if (knot1.0 - knot2.0) <= -1 {
                        step.0 = -1;
                    }
                    if (knot1.1 - knot2.1) >= 1 {
                        step.1 = 1;
                    } else if (knot1.1 - knot2.1) <= -1 {
                        step.1 = -1;
                    }

                    if (step.0 != 0) ^ (step.1 != 0) {
                        if (knot1.0 - knot2.0).abs() == 1 {
                            step.0 = 0;
                        }
                        if (knot1.1 - knot2.1).abs() == 1 {
                            step.1 = 0;
                        }
                    }

                    if (knot1.0 - knot2.0).abs() == 1 && (knot1.1 - knot2.1).abs() == 1 {
                        step.0 = 0;
                        step.1 = 0;
                    }

                    knots[i].0 += step.0;
                    knots[i].1 += step.1;
                }

                //println!("{}\n{:?}", l, knots);
                visited.insert(knots[knots.len()-1]);
            }
        }
    }

    return visited.len();
}

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}
