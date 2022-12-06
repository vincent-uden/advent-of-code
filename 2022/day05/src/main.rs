use std::fs;

fn part1() -> String {
    let text = fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let contents: Vec<&str> = text.split("\n\n").collect();

    let cargo_stack = contents[0].split("\n");
    let mut cargo: Vec<Vec<char>> = vec![];

    let mut j = 0;
    let mut k = 0;
    for line in cargo_stack {
        let chars = line.chars().collect::<Vec<char>>();
        for i in (1..line.len() - 1).step_by(4) {
            if j == 0 {
                cargo.push(vec![]);
            }
            if chars[i].is_alphabetic() {
                cargo[k].push(chars[i]);
            }
            k += 1;
        }
        j += 1;
        k = 0;
    }

    cargo = cargo
        .into_iter()
        .map(|x| x.into_iter().rev().collect())
        .collect();

    let mut instructions: Vec<&str> = contents[1]
        .split("\n")
        .map(|l| if l.len() > 0 { &l[5..] } else { "" })
        .collect();
    instructions.pop();

    let moves = instructions.into_iter().map(|l| {
        let parts: Vec<&str> = l.split(" ").collect();
        (
            str::parse::<i32>(parts[0]).unwrap(),
            str::parse::<usize>(parts[2]).unwrap(),
            str::parse::<usize>(parts[4]).unwrap(),
        )
    });

    for (amount, from, to) in moves {
        for _ in 0..amount {
            let popped = cargo[from - 1].pop().unwrap();
            cargo[to - 1].push(popped);
        }
    }

    return cargo
        .into_iter()
        .map(|x| x.last().unwrap().clone())
        .collect();
}

fn part2() -> String {
    let text = fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let contents: Vec<&str> = text.split("\n\n").collect();

    let cargo_stack = contents[0].split("\n");
    let mut cargo: Vec<Vec<char>> = vec![];

    let mut j = 0;
    let mut k = 0;
    for line in cargo_stack {
        let chars = line.chars().collect::<Vec<char>>();
        for i in (1..line.len() - 1).step_by(4) {
            if j == 0 {
                cargo.push(vec![]);
            }
            if chars[i].is_alphabetic() {
                cargo[k].push(chars[i]);
            }
            k += 1;
        }
        j += 1;
        k = 0;
    }

    cargo = cargo
        .into_iter()
        .map(|x| x.into_iter().rev().collect())
        .collect();

    let mut instructions: Vec<&str> = contents[1]
        .split("\n")
        .map(|l| if l.len() > 0 { &l[5..] } else { "" })
        .collect();
    instructions.pop();

    let moves = instructions.into_iter().map(|l| {
        let parts: Vec<&str> = l.split(" ").collect();
        (
            str::parse::<i32>(parts[0]).unwrap(),
            str::parse::<usize>(parts[2]).unwrap(),
            str::parse::<usize>(parts[4]).unwrap(),
        )
    });

    for (amount, from, to) in moves {
        let mut moving = vec![];
        for _ in 0..amount {
            let popped = cargo[from - 1].pop().unwrap();
            moving.push(popped);
        }

        for _ in 0..amount {
            let popped = moving.pop().unwrap();
            cargo[to - 1].push(popped);
        }
    }

    return cargo
        .into_iter()
        .map(|x| x.last().unwrap().clone())
        .collect();
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
