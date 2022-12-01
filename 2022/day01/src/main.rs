use std::fs;

fn read_calories(path: &str) -> Vec<i32> {
    let content = fs::read_to_string(path).unwrap();

    let elves = content.split("\n\n");

    let calories = elves.map(|elf_rows| {
        elf_rows
            .split("\n")
            .filter(|row| row.len() > 0)
            .map(|row| row.parse::<i32>().unwrap())
    });

    let mut total_calories: Vec<i32> = calories
        .map(|rows| rows.fold(0, |acc, x| acc + x))
        .collect();
    total_calories.sort();
    total_calories.reverse();

    return total_calories;
}

fn main() {
    let calories_per_elf = read_calories("./input.txt");

    println!("{:?}", &calories_per_elf);
    println!(
        "{:?}",
        &calories_per_elf[0] + &calories_per_elf[1] + &calories_per_elf[2]
    );
}
