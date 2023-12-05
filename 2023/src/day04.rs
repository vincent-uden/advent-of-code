use std::collections::VecDeque;

pub fn part1(input: &str) -> i32 {
    let mut points = 0;

    for line in input.lines() {
        let mut parts = line[8..].split(" | ");
        let winning_nrs: Vec<&str> = parts.next().unwrap().split(" ").collect();
        let card_nrs = parts.next().unwrap().split(" ");

        let matches: Vec<&str> = card_nrs
            .filter(|x| winning_nrs.contains(x) && !x.is_empty())
            .collect();
        if matches.len() > 0 {
            points += 2_i32.pow(matches.len() as u32 - 1);
        }
    }

    points
}

#[derive(Clone, Debug)]
struct Card {
    pub id: i32,
    pub winning_nrs: Vec<i32>,
    pub card_nrs: Vec<i32>,
}
pub fn part2(input: &str) -> i32 {
    let mut cards_obtained: Vec<i32> = vec![];
    let mut cards: Vec<Card> = vec![];
    let mut queue: VecDeque<Card> = VecDeque::new();

    for line in input.lines() {
        let mut header_nrs = line[5..].split(":");
        let id = header_nrs.next().unwrap().trim().parse().unwrap();
        let mut parts = header_nrs.next().unwrap().split(" | ");
        let card = Card {
            id,
            winning_nrs: parts
                .next()
                .unwrap()
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim().parse().unwrap())
                .collect(),
            card_nrs: parts
                .next()
                .unwrap()
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim().parse().unwrap())
                .collect(),
        };
        cards.push(card.clone());
        queue.push_back(card);
        cards_obtained.push(1);
    }

    while !queue.is_empty() {
        let card = queue.pop_front().unwrap();
        let matches = card.card_nrs
            .iter()
            .filter(|x| card.winning_nrs.contains(x));

        for (i, _) in matches.enumerate() {
            queue.push_back(cards[card.id as usize + i].clone());
            cards_obtained[card.id as usize + i] += 1;
        }
    }

    let mut sum = 0;
    for c in cards_obtained {
        sum += c;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::day04::{part1, part2};

    #[test]
    fn test_part1() {
        let input = " \
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert!(part1(input) == 13);
    }

    #[test]
    fn test_part2() {
        let input = " \
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert!(part2(input) == 30);
    }
}
