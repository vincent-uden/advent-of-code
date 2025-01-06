use std::collections::HashSet;

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    pub fn all_part_one() -> Vec<Op> {
        vec![Op::Add, Op::Mul]
    }

    pub fn all_part_two() -> Vec<Op> {
        vec![Op::Add, Op::Mul, Op::Concat]
    }
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    let mut out = vec![];

    for l in input.lines() {
        let parts: Vec<&str> = l.split(": ").collect();
        let operands: Vec<i64> = parts[1].split(" ").map(|x| x.parse().unwrap()).collect();
        out.push((parts[0].parse().unwrap(), operands));
    }

    out
}

fn calculate_eqn(operands: &[i64], operations: &[Op]) -> i64 {
    let mut out = operands[0];
    for i in 0..operations.len() {
        out = match operations[i] {
            Op::Add => out + operands[i + 1],
            Op::Mul => out * operands[i + 1],
            Op::Concat => {
                let mut x = out.to_string();
                let y = operands[i + 1].to_string();
                x.push_str(&y);
                x.parse().unwrap()
            }
        }
    }
    out
}

fn seq_from_int(operations: &[Op], i: usize, n: usize) -> Vec<Op> {
    let mut out = vec![];
    let mut i = i;
    while out.len() < n {
        out.push(operations[i % operations.len()]);
        i /= operations.len();
    }
    out
}

fn equation_possible_part_one((result, operands): &(i64, Vec<i64>)) -> bool {
    let operations = Op::all_part_one();
    let mut i = 0;
    while i < operations.len().pow((operands.len() - 1) as u32) {
        let ops: Vec<Op> = seq_from_int(&operations, i, operands.len() - 1);
        let calc_result = calculate_eqn(operands, &ops);
        if calc_result == *result {
            return true;
        }
        i += 1;
    }

    false
}

fn equation_possible_part_two((result, operands): &(i64, Vec<i64>)) -> bool {
    let operations = Op::all_part_two();
    let mut i = 0;
    while i < operations.len().pow((operands.len() - 1) as u32) {
        let ops: Vec<Op> = seq_from_int(&operations, i, operands.len() - 1);
        let calc_result = calculate_eqn(operands, &ops);
        if calc_result == *result {
            return true;
        }
        i += 1;
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let mut possible = vec![];
    for eqn in &equations {
        if equation_possible_part_one(eqn) {
            possible.push(eqn.0);
        }
    }
    let mut sum = 0;
    for x in possible {
        sum += x;
    }
    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let mut possible = vec![];
    for eqn in &equations {
        if equation_possible_part_two(eqn) {
            possible.push(eqn.0);
        }
    }
    let mut sum = 0;
    for x in possible {
        sum += x;
    }
    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
