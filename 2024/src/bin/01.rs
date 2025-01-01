use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(|x| {
            let numbers: Vec<i64> = x.split("   ").map(|n| n.parse().unwrap()).collect();
            (numbers[0], numbers[1])
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();
    let x: i64 = left.iter().zip(&right).map(|(l, r)| (l - r).abs()).sum();
    Some(x as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, mut right) = parse_input(input);
    right.sort();

    let mut occurences = HashMap::<i64, usize>::new();
    let mut start_i = 0;
    for (i, n) in right.iter().enumerate() {
        if *n != right[start_i] {
            occurences.insert(right[i - 1], i - start_i);
            start_i = i;
        }
    }
    occurences.insert(*right.last().unwrap(), right.len() - start_i);

    let mut similarity = 0;
    for x in &left {
        match occurences.get(x) {
            Some(n) => similarity += x * (*n as i64),
            None => {}
        }
    }

    Some(similarity as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
