use std::collections::HashMap;

pub fn part1(input: &str) {
    let mut sum: i32 = 0;
    for line in input.lines() {
        let mut number = String::new();
        for c in line.chars() {
            if "0123456789".contains(c) {
                number.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if "0123456789".contains(c) {
                number.push(c);
                break;
            }
        }
        sum += number.parse::<i32>().unwrap();
    }

    println!("{}", sum);
}

pub fn part2(input: &str) {
    let mut sum: i32 = 0;
    let mut digits: HashMap<&str, char> = HashMap::new();
    digits.insert("zero",  '0');
    digits.insert("one",   '1');
    digits.insert("two",   '2');
    digits.insert("three", '3');
    digits.insert("four",  '4');
    digits.insert("five",  '5');
    digits.insert("six",   '6');
    digits.insert("seven", '7');
    digits.insert("eight", '8');
    digits.insert("nine",  '9');
    digits.insert("0", '0');
    digits.insert("1", '1');
    digits.insert("2", '2');
    digits.insert("3", '3');
    digits.insert("4", '4');
    digits.insert("5", '5');
    digits.insert("6", '6');
    digits.insert("7", '7');
    digits.insert("8", '8');
    digits.insert("9", '9');

    for line in input.lines() {
        let mut forward = line.chars().collect::<String>();
        let mut reverse = line.chars().collect::<String>();
        let mut number = String::new();
        '_outer: while forward.len() > 0 {
            for key in digits.keys() {
                if forward.starts_with(key) {
                    number.push(digits[key]);
                    break '_outer;
                }
            }
            forward = forward[1..].to_string();
        }
        '_outer: while reverse.len() > 0 {
            for key in digits.keys() {
                if reverse.ends_with(key) {
                    number.push(digits[key]);
                    break '_outer;
                }
            }
            reverse.pop().unwrap();
        }
        println!("{} {}", line, number);
        sum += number.parse::<i32>().unwrap();
    }

    println!("{}", sum);
}
