use std::fs;

mod tests;

fn unique_chars(signal: &str) -> bool {
    let mut un: Vec<char> = signal.chars().collect();
    un.sort();
    un.dedup();

    return un.len() == signal.len();
}

fn part_1(signal: &str) -> i32 {
    for i in 0..signal.len() - 4 {
        if unique_chars(&signal[i..i+4]) {
            return (i + 4) as i32;
        }
    }

    return -1;
}

fn part_2(signal: &str) -> i32 {
    for i in 0..signal.len() - 14 {
        if unique_chars(&signal[i..i+14]) {
            return (i + 14) as i32;
        }
    }

    return -1;
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    println!("{}", part_1(&content));
    println!("{}", part_2(&content));
}
