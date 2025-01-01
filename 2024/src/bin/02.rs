advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.split(" ").map(|x| x.parse().unwrap()).collect())
        .collect()
}

enum Gradient {
    Increasing,
    Decreasing,
}

fn is_transition_safe(d: i64, gradient: &Gradient) -> bool {
    (match gradient {
        Gradient::Increasing => d > 0,
        Gradient::Decreasing => d < 0,
    }) && d.abs() >= 1
        && d.abs() <= 3
}

fn is_safe(report: &[i64]) -> bool {
    let gradient = if report[1] > report[0] {
        Gradient::Increasing
    } else if report[1] < report[0] {
        Gradient::Decreasing
    } else {
        return false;
    };

    report
        .windows(2)
        .map(|w| w[1] - w[0])
        .fold(true, |acc, d| acc && is_transition_safe(d, &gradient))
}

pub fn part_one(input: &str) -> Option<u64> {
    let reports = parse_input(input);
    let mut safe = 0;
    for report in &reports {
        if is_safe(report) {
            safe += 1;
        }
    }

    Some(safe)
}

pub fn part_two(input: &str) -> Option<u64> {
    let reports = parse_input(input);
    let mut safe = 0;
    for report in &reports {
        if is_safe(report) {
            safe += 1;
        } else {
            for i in 0..report.len() {
                let mut r = report.clone();
                r.remove(i);
                if is_safe(&r) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    Some(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
