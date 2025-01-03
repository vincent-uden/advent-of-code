advent_of_code::solution!(4);

pub fn scan(field: &Vec<Vec<char>>, kernel: &Vec<Vec<char>>) -> i64 {
    let mut n_matches = 0;
    for i in 0..(field.len() - kernel.len() + 1) {
        for j in 0..(field[0].len() - kernel[0].len() + 1) {
            let mut matching = true;
            for y in 0..kernel.len() {
                for x in 0..kernel[0].len() {
                    if field[i + y][j + x] != kernel[y][x] && kernel[y][x] != '.' {
                        matching = false;
                    }
                }
            }
            if matching {
                n_matches += 1;
            }
        }
    }

    n_matches
}

pub fn part_one(input: &str) -> Option<u64> {
    let kernels = vec![
        vec!["XMAS"],
        vec!["SAMX"],
        vec!["X", "M", "A", "S"],
        vec!["S", "A", "M", "X"],
        vec!["X...", ".M..", "..A.", "...S"],
        vec!["...S", "..A.", ".M..", "X..."],
        vec!["S...", ".A..", "..M.", "...X"],
        vec!["...X", "..M.", ".A..", "S..."],
    ];
    let kernels: Vec<Vec<Vec<char>>> = kernels
        .iter()
        .map(|s| s.iter().map(|s| s.chars().collect::<Vec<char>>()).collect())
        .collect();
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut total_matches = 0;
    for k in &kernels {
        let m = scan(&input, k);
        total_matches += m;
    }

    Some(total_matches as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let kernels = vec![
        vec!["M.S", ".A.", "M.S"],
        vec!["M.M", ".A.", "S.S"],
        vec!["S.M", ".A.", "S.M"],
        vec!["S.S", ".A.", "M.M"],
    ];
    let kernels: Vec<Vec<Vec<char>>> = kernels
        .iter()
        .map(|s| s.iter().map(|s| s.chars().collect::<Vec<char>>()).collect())
        .collect();
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut total_matches = 0;
    for k in &kernels {
        let m = scan(&input, k);
        total_matches += m;
    }

    Some(total_matches as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
