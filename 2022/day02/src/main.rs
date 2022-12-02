use std::{
    fs::File,
    io::{self, BufRead},
};
// Rock -> Paper
// Paper -> Rock
// Scissor -> Scissor

#[derive(Debug, Clone, Copy)]
enum Shape {
    ROCK = 0,
    PAPER = 1,
    SCISSORS = 2,
}

fn char_to_shape(c: char) -> Shape {
    match c {
        'A' => Shape::ROCK,
        'B' => Shape::PAPER,
        'C' => Shape::SCISSORS,
        'X' => Shape::ROCK,
        'Y' => Shape::PAPER,
        'Z' => Shape::SCISSORS,
        _ => panic!("Unsupported strategy"),
    }
}

fn get_score(s1: &Shape, s2: &Shape) -> i32 {
    let mut score = *s1 as i32 + 1;

    let i1 = *s1 as i32;
    let i2 = *s2 as i32;

    if i1 == (i2 + 1) % 3 {
        // Win
        score += 6;
    } else if i1 == i2 {
        score += 3;
    }

    score
}

fn part_1() -> i32 {
    let file = File::open("./input.txt").unwrap();

    let mut score = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let me = char_to_shape(l.chars().nth(2).unwrap());
            let opponent = char_to_shape(l.chars().nth(0).unwrap());

            score += get_score(&me, &opponent);
        }
    }

    return score;
}

fn part_2() -> i32 {
    let file = File::open("./input.txt").unwrap();

    let mut score = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let opponent = char_to_shape(l.chars().nth(0).unwrap());
            let num_me = match l.chars().nth(2).unwrap() {
                'X' => (opponent as i32 - 1).rem_euclid(3),
                'Y' => opponent as i32,
                'Z' => (opponent as i32 + 1) % 3,
                _   => panic!("Invalid option"),
            };
            let me = match num_me {
                0 => Shape::ROCK,
                1 => Shape::PAPER,
                2 => Shape::SCISSORS,
                _ => panic!(""),
            };

            score += get_score(&me, &opponent);
        }
    }

    return score;
}

fn main() {
    println!("Part 1:{}", part_1());
    println!("Part 2:{}", part_2());
}
