use std::cmp::{max, min};

#[derive(Clone, Copy, Debug)]
struct PartNr {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub nr: i32,
}

fn adjacent_to_symbol(nr: &PartNr, schematic: &Vec<Vec<char>>) -> bool {
    let width = schematic[0].len() as i32;
    let height = schematic.len() as i32;

    let not_symbols = "0123456789.";

    for y in max(0, nr.y - 1)..min(height, nr.y + 2) {
        for x in max(0, nr.x - 1)..min(width, nr.x + nr.w + 1) {
            if !not_symbols.contains(schematic[y as usize][x as usize]) {
                return true;
            }
        }
    }

    false
}

fn find_all_numbers(schematic: &Vec<Vec<char>>) -> Vec<PartNr> {
    let mut numbers: Vec<PartNr> = vec![];
    let digits = "0123456789";

    let mut y = 0;
    for line in schematic {
        let mut x = 0;
        let mut nr = String::new();
        let mut new_nr = PartNr {
            x: 0,
            y,
            w: 0,
            nr: 0,
        };
        for c in line {
            if digits.contains(*c) {
                if nr.is_empty() {
                    new_nr.x = x;
                }

                nr.push(*c);
            } else {
                if !nr.is_empty() {
                    new_nr.w = x - new_nr.x;
                    new_nr.nr = nr.parse().unwrap();
                    if adjacent_to_symbol(&new_nr, schematic) {
                        numbers.push(new_nr);
                    }
                    nr.clear();
                }
            }
            x += 1;
        }
        if !nr.is_empty() {
            new_nr.w = x - new_nr.x;
            new_nr.nr = nr.parse().unwrap();
            if adjacent_to_symbol(&new_nr, schematic) {
                numbers.push(new_nr);
            }
        }

        y += 1;
    }

    numbers
}

pub fn part1(input: &str) -> i32 {
    let mut summed_part_nr = 0;
    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let numbers = find_all_numbers(&lines);

    for nr in numbers {
        if adjacent_to_symbol(&nr, &lines) {
            summed_part_nr += nr.nr;
        }
    }

    summed_part_nr
}

fn numbers_adjacent<'a>(pos_x: i32, pos_y: i32, schematic: &Vec<Vec<char>>, numbers: &'a Vec<PartNr>) -> Vec<&'a PartNr> {
    let mut adj = vec![];
    let width = schematic[0].len() as i32;
    let height = schematic.len() as i32;

    for nr in numbers {
        for y in max(0, nr.y - 1)..min(height, nr.y + 2) {
            for x in max(0, nr.x - 1)..min(width, nr.x + nr.w + 1) {
                if x == pos_x && y == pos_y {
                    adj.push(nr);
                }
            }
        }
    }

    adj
}

pub fn part2(input: &str) -> i32 {
    let mut summed_gear_ratios = 0;
    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut numbers = find_all_numbers(&lines);

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '*' {
                let adjacent_nrs = numbers_adjacent(x as i32, y as i32, &lines, &numbers);
                if adjacent_nrs.len() == 2 {
                    summed_gear_ratios += adjacent_nrs[0].nr * adjacent_nrs[1].nr;
                }
            }
        }
    }

    summed_gear_ratios
}

#[cfg(test)]
mod tests {
    use crate::day03::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "467..114..\n\
                    ...*......\n\
                    ..35..633.\n\
                    ......#...\n\
                    617*......\n\
                    .....+.58.\n\
                    ..592.....\n\
                    ......755.\n\
                    ...$.*....\n\
                    .664.598..";

        assert!(part1(input) == 4361);
    }

    #[test]
    fn test_part2() {
        let input = "467..114..\n\
                    ...*......\n\
                    ..35..633.\n\
                    ......#...\n\
                    617*......\n\
                    .....+.58.\n\
                    ..592.....\n\
                    ......755.\n\
                    ...$.*....\n\
                    .664.598..";

        assert!(part2(input) == 467835);
    }
}
