use core::fmt;
use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    index: i64,
    items: VecDeque<i64>,
    op1: String,
    op2: String,
    operator: String,
    test_mod: i64,
    test_true: i64,
    test_false: i64,
    inspected: i64,
}

impl Monkey {
    fn from_str(spec: String) -> Monkey {
        let lines: Vec<&str> = spec.split("\n").map(str::trim).collect();
        let index = lines[0]
            .split(" ")
            .last()
            .unwrap()
            .chars()
            .nth(0)
            .unwrap()
            .to_digit(10)
            .unwrap() as i64;

        let items: VecDeque<i64> = lines[1]
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let expr: Vec<String> = lines[2]
            .split("= ")
            .last()
            .unwrap()
            .split(" ")
            .map(|x| String::from(x))
            .collect();

        let test_mod = lines[3].split(" ").last().unwrap().parse::<i64>().unwrap();

        let test_true = lines[4].split(" ").last().unwrap().parse::<i64>().unwrap();
        let test_false = lines[5].split(" ").last().unwrap().parse::<i64>().unwrap();

        Monkey {
            index,
            items,
            op1: expr[0].clone(),
            op2: expr[2].clone(),
            operator: expr[1].clone(),
            test_mod,
            test_true,
            test_false,
            inspected: 0,
        }
    }

    fn operate(&self, x: i64) -> i64 {
        println!("  Monkey inspects an item with a worry level of {}", x);
        let operand1 = match self.op1.as_str() {
            "old" => x,
            _ => self.op1.parse::<i64>().unwrap(),
        };

        let operand2 = match self.op2.as_str() {
            "old" => x,
            _ => self.op2.parse::<i64>().unwrap(),
        };

        return match self.operator.as_str() {
            "+" => {
                println!(
                    "    Worry level is increased by {} to {}.",
                    operand2,
                    operand1 + operand2
                );
                operand1 + operand2
            }
            "-" => {
                println!(
                    "    Worry level is subtracted by {} to {}.",
                    operand2,
                    operand1 - operand2
                );
                operand1 - operand2
            }
            "*" => {
                println!(
                    "    Worry level is multiplied by {} to {}.",
                    operand2,
                    operand1 * operand2
                );
                operand1 * operand2
            }
            "/" => {
                println!(
                    "    Worry level divided multiplied by {} to {}.",
                    operand2,
                    operand1 / operand2
                );
                operand1 / operand2
            }
            _ => panic!(),
        };
    }

    fn operate2(&self, x: i64) -> i64 {
        let operand1 = match self.op1.as_str() {
            "old" => x,
            _ => self.op1.parse::<i64>().unwrap(),
        };

        let operand2 = match self.op2.as_str() {
            "old" => x,
            _ => self.op2.parse::<i64>().unwrap(),
        };

        return match self.operator.as_str() {
            "+" => operand1 + operand2,
            "-" => operand1 - operand2,
            "*" => operand1 * operand2,
            "/" => operand1 / operand2,
            _ => panic!(),
        };
    }

    fn test(&self, x: i64) -> i64 {
        return if x % self.test_mod == 0 {
            self.test_true
        } else {
            self.test_false
        };
    }
}

fn part_1(input: &str) {
    let monkey_spcs = input.split("\r\n\r\n").map(String::from);
    let mut monkeys: Vec<Monkey> = monkey_spcs.map(Monkey::from_str).collect();
    println!("{:?}", monkeys);

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            println!("Monkey {}:", monkeys[i].index);
            while let Some(item) = monkeys[i].items.pop_front() {
                let new_worry = monkeys[i].operate(item) / 3;
                println!(
                    "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                    new_worry
                );
                let next_monkey = monkeys[i].test(new_worry) as usize;
                println!(
                    "    Item with worry level {} is thrown to monkey {}",
                    new_worry, next_monkey
                );
                monkeys[i].inspected += 1;

                monkeys[next_monkey].items.push_back(new_worry);
            }
        }
    }

    for m in &monkeys {
        println!("{:?}", m.inspected);
    }
}

fn part_2(input: &str) {
    let monkey_spcs = input.split("\r\n\r\n").map(String::from);
    let mut monkeys: Vec<Monkey> = monkey_spcs.map(Monkey::from_str).collect();
    println!("{:?}", monkeys);

    for round in 0..10000 {
        for i in 0..monkeys.len() {
            //println!("Monkey {}:", monkeys[i].index);
            while let Some(item) = monkeys[i].items.pop_front() {
                let new_worry = monkeys[i].operate2(item) % (2*3*5*7*11*13*17*19);
                //println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", new_worry);
                let next_monkey = monkeys[i].test(new_worry) as usize;
                //println!("    Item with worry level {} is thrown to monkey {}", new_worry, next_monkey);
                monkeys[i].inspected += 1;

                monkeys[next_monkey].items.push_back(new_worry);
            }
        }
    }

    for m in &monkeys {
        println!("{:?}", m.inspected);
    }
}

fn main() {
    let inp = include_str!("../input.txt");
    part_1(inp);
    part_2(inp);
}
