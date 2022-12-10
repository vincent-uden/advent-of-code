use core::panic;
use std::{
    collections::VecDeque,
    fs,
    io::{self, BufRead},
};

#[derive(Clone, PartialEq, Debug)]
enum OpCode {
    NOOP,
    ADDX,
}

#[derive(Clone, Debug)]
struct Op {
    code: OpCode,
    val: Option<i32>,
}

fn part_1() -> i32 {
    let mut reg = 1;

    let file = fs::File::open("./input.txt").unwrap();

    let mut ops: Vec<Op> = vec![];
    let mut cycle = 0;
    let mut sum = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let items: Vec<&str> = l.split(" ").collect();

            let op = Op {
                code: match items[0] {
                    "noop" => OpCode::NOOP,
                    "addx" => OpCode::ADDX,
                    _ => panic!(),
                },
                val: if items.len() > 1 {
                    Some(items[1].parse::<i32>().unwrap())
                } else {
                    None
                },
            };

            //println!("{} {} {} {:?} ", cycle, reg, sum, op);

            if cycle >= 19 && (cycle - 19) % 40 == 0 {
                //println!("SUMMING {}", reg * ((cycle as i32) + 1));
                sum += reg * ((cycle as i32) + 1);
            }

            if op.code == OpCode::ADDX {
                cycle += 1;
                println!("{} {} {} {:?} ", cycle, reg, sum, op);
                if cycle >= 19 && (cycle - 19) % 40 == 0 {
                    //println!("SUMMING {}", reg * ((cycle as i32) + 1));
                    sum += reg * ((cycle as i32) + 1);
                }
                reg += op.val.unwrap();
            }

            cycle += 1;
        }
    }

    return sum;
}

fn part_2() {
    let mut reg = 1;

    let file = fs::File::open("./input.txt").unwrap();

    let mut ops: Vec<Op> = vec![];
    let mut cycle = 0;
    let mut sum = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            let items: Vec<&str> = l.split(" ").collect();

            let op = Op {
                code: match items[0] {
                    "noop" => OpCode::NOOP,
                    "addx" => OpCode::ADDX,
                    _ => panic!(),
                },
                val: if items.len() > 1 {
                    Some(items[1].parse::<i32>().unwrap())
                } else {
                    None
                },
            };

            //println!("{} {} {} {:?} ", cycle, reg, sum, op);

            if cycle >= 19 && (cycle - 19) % 40 == 0 {
                //println!("SUMMING {}", reg * ((cycle as i32) + 1));
                sum += reg * ((cycle as i32) + 1);
            }

            print!("{}", if (reg-1..reg+1).contains(&(cycle % 40)) {"#"} else {"."});
            if (cycle+1) % 40 == 0 {
                print!("\n");
            }

            if op.code == OpCode::ADDX {
                cycle += 1;
                //println!("{} {} {} {:?} ", cycle, reg, sum, op);
                if cycle >= 19 && (cycle - 19) % 40 == 0 {
                    //println!("SUMMING {}", reg * ((cycle as i32) + 1));
                    sum += reg * ((cycle as i32) + 1);
                }
            print!("{}", if (reg-1..reg+1).contains(&(cycle % 40)) {"#"} else {"."});
            if (cycle+1) % 40 == 0 {
                print!("\n");
            }
                reg += op.val.unwrap();
            }

            cycle += 1;

            // This doesnt really work, but the output is readable with some creative liberties
        }
    }
}

fn main() {
    //println!("{}", part_1());
    part_2();
}
