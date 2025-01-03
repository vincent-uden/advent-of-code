use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn build_rule_set(input: &str) -> HashMap<i64, Vec<i64>> {
    let mut out: HashMap<i64, Vec<i64>> = HashMap::new();

    let rules = input.split("\n\n").take(1).collect::<Vec<&str>>()[0].lines();
    for l in rules {
        let parts: Vec<_> = l.split("|").collect();
        let before = parts[0].parse().unwrap();
        let after: i64 = parts[1].parse().unwrap();

        if let Some(x) = out.get_mut(&after) {
            x.push(before);
        } else {
            out.insert(after, vec![before]);
        }
    }

    out
}

pub fn parse_sequences(input: &str) -> Vec<Vec<i64>> {
    let mut out = vec![];
    let sequences = input.split("\n\n").skip(1).take(1).collect::<Vec<&str>>()[0].lines();
    for seq in sequences {
        let numbers = seq.split(",");
        let mut xs = vec![];
        for n in numbers {
            xs.push(n.parse().unwrap());
        }
        out.push(xs);
    }
    out
}

pub fn validate_sequence(sequence: &[i64], rules: &HashMap<i64, Vec<i64>>) -> bool {
    let mut cache = HashSet::new();
    for n in sequence {
        if let Some(before) = rules.get(n) {
            for b in before {
                if !cache.contains(b) && sequence.contains(b) {
                    return false;
                }
            }
        }
        cache.insert(n);
    }
    return true;
}

pub fn middle(sequence: &[i64]) -> i64 {
    sequence[sequence.len() / 2]
}

fn insertable(sequence: &[i64], rules: &HashMap<i64, Vec<i64>>, n: i64) -> bool {
    for (after, before) in rules.iter() {
        if before.contains(&n) && sequence.contains(after) {
            return false;
        }
    }
    return true;
}

/// Insertion sort with a rules-based comparison operator
pub fn re_order(sequence: &[i64], rules: &HashMap<i64, Vec<i64>>) -> Vec<i64> {
    let mut out: Vec<i64> = vec![];

    for i in 0..sequence.len() {
        for j in 0..(out.len() + 1) {
            let slice = &out[0..(out.len() - j)];
            if insertable(slice, rules, sequence[i]) {
                out.insert(out.len() - j, sequence[i]);
                break;
            }
        }
    }

    out
}

pub fn part_one(input: &str) -> Option<u64> {
    let rules = build_rule_set(input);
    let sequences = parse_sequences(input);
    let valid_sequences: Vec<_> = sequences
        .iter()
        .filter(|s| validate_sequence(s, &rules))
        .collect();

    Some(
        valid_sequences
            .iter()
            .fold(0_i64, |acc, seq| acc + middle(seq)) as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let rules = build_rule_set(input);
    let sequences = parse_sequences(input);
    let invalid_sequences: Vec<_> = sequences
        .iter()
        .filter(|s| !validate_sequence(s, &rules))
        .collect();
    let re_ordered: Vec<Vec<i64>> = invalid_sequences
        .iter()
        .map(|seq| re_order(seq, &rules))
        .collect();

    Some(re_ordered.iter().fold(0_i64, |acc, seq| acc + middle(&seq)) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
