use std::collections::HashMap;

pub fn part1(input: &str) -> i32 {
    let mut amounts: HashMap<&str, i32> = HashMap::new();
    amounts.insert("red", 12);
    amounts.insert("green", 13);
    amounts.insert("blue", 14);

    let mut buckets = amounts.clone();

    let mut summed_ids = 0;
    let games = input.trim().split('\n');

    for g in games {
        let parts: Vec<&str> = g[5..].split(':').collect();
        let id = parts[0];
        let showings = parts[1].split(';');

        let mut possible = true;
        for s in showings {
            for k in amounts.keys() {
                buckets.insert(k, 0);
            }
            let colors = s.split(',').map(|x| x.trim());
            for c in colors {
                let c_parts: Vec<&str> = c.split(' ').collect();
                let prev_amount = buckets.get(c_parts[1]).unwrap();
                buckets.insert(c_parts[1], prev_amount + c_parts[0].parse::<i32>().unwrap());
            }

            for k in amounts.keys() {
                if amounts[k] < buckets[k] {
                    possible = false;
                }
            }
        }
        if possible {
            summed_ids += id.parse::<i32>().unwrap();
        }
    }

    summed_ids
}

pub fn part2(input: &str) -> i32 {
    let mut amounts: HashMap<&str, i32> = HashMap::new();
    amounts.insert("red", 12);
    amounts.insert("green", 13);
    amounts.insert("blue", 14);

    let mut buckets = amounts.clone();

    let mut summed_powers = 0;
    let games = input.trim().split('\n');

    for g in games {
        let showings = g.split(':').collect::<Vec<&str>>()[1].split(';');

        for k in amounts.keys() {
            buckets.insert(k, 0);
        }

        for s in showings {
            let colors = s.split(',').map(|x| x.trim());
            for c in colors {
                let c_parts: Vec<&str> = c.split(' ').collect();
                let prev_amount = buckets.get(c_parts[1]).unwrap();
                let new_amount: i32 = c_parts[0].parse().unwrap();
                if new_amount > *prev_amount {
                    buckets.insert(c_parts[1], new_amount);
                }
            }
        }

        summed_powers += buckets.values().fold(1, |x, acc| acc * x);
    }

    summed_powers
}

#[cfg(test)]
mod tests {
    use crate::day02::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert!(part1(input) == 8);
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert!(part2(input) == 2286);
    }
}
