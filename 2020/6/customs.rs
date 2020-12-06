use std::collections::{HashSet,HashMap};
use std::fs;

fn part1(groups: &Vec<&str>) -> usize {
    let mut sum = 0;

    for group in groups {
        let mut answers:HashSet<char> = HashSet::new();

        for c in group.chars() {
            if c != '\n' {
                answers.insert(c);
            }
        }

        sum += answers.len();
    }

    sum
}

fn part2(groups: &Vec<&str>) -> usize {
    let mut sum = 0;
    
    for group in groups {
        let mut answers:HashMap<char,i32> = HashMap::new();
        let rows: Vec<&str> = group.split("\n").collect();
        let mut size = rows.len();
        if rows.last().unwrap() == &"" {
            size -= 1;
        }

        for c in group.chars() {
            if c != '\n' {
                answers.entry(c).or_insert(0);
                answers.insert(c, answers.get(&c).unwrap() + 1);
                println!("{} {}", c, answers[&c]);
            }
        }

        for (_, value) in answers {
            if value == (size as i32) {
                sum += 1;
            }
        }

        println!("Sum {}, Size {}", sum, size);
        println!("{:?}", rows);
    }

    sum
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();

    let groups:Vec<&str> = content.split("\n\n").collect();

    let test = vec![ "abc\n", "a\nb\nc\n", "ab\nac\n", "a\na\na\na\n", "b\n" ];

    println!("{}", part1(&groups));
    println!("{}", part2(&groups));
    //println!("{}", part2(&test));
}
