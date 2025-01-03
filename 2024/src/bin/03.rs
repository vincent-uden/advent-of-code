advent_of_code::solution!(3);

#[derive(Debug, Clone, Copy)]
enum ParserState {
    ScanningOp,
    SkippingOp,
    ScanningArg { delim: char },
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul(i64, i64),
}

fn parse_one(input: &str) -> Vec<Op> {
    let mut out = vec![];
    let mut i = 0;
    let mut state = ParserState::ScanningOp;

    let mut arg1 = 0;

    while i < input.len() {
        match state {
            ParserState::ScanningOp => {
                if i + 4 > input.len() {
                    break;
                }
                if &input[i..(i + 4)] == "mul(" {
                    state = ParserState::ScanningArg { delim: ',' };
                    i += 4;
                } else {
                    i += 1;
                }
            }
            ParserState::ScanningArg { delim } => {
                let arg = input
                    .chars()
                    .skip(i)
                    .take_while(|c| *c != delim)
                    .collect::<String>();
                match arg.parse::<i64>() {
                    Ok(n) => {
                        if delim == ')' {
                            state = ParserState::ScanningOp;
                            out.push(Op::Mul(arg1, n));
                        } else {
                            state = ParserState::ScanningArg { delim: ')' };
                            arg1 = n;
                        }
                        // +1 for delimiter
                        i += arg.len() + 1;
                    }
                    Err(_) => {
                        state = ParserState::ScanningOp;
                        i += 1;
                    }
                }
            }
            _ => {}
        }
    }

    out
}

fn parse_two(input: &str) -> Vec<Op> {
    let mut out = vec![];
    let mut i = 0;
    let mut state = ParserState::ScanningOp;

    let mut arg1 = 0;

    while i < input.len() {
        match state {
            ParserState::ScanningOp => {
                if i + 4 < input.len() && &input[i..(i + 4)] == "mul(" {
                    state = ParserState::ScanningArg { delim: ',' };
                    i += 4;
                } else if i + 4 < input.len() && &input[i..(i + 4)] == "do()" {
                    state = ParserState::ScanningOp;
                    i += 4;
                } else if i + 7 < input.len() && &input[i..(i + 7)] == "don\'t()" {
                    state = ParserState::SkippingOp;
                    i += 7;
                } else {
                    i += 1;
                }
            }
            ParserState::ScanningArg { delim } => {
                let arg = input
                    .chars()
                    .skip(i)
                    .take_while(|c| *c != delim)
                    .collect::<String>();
                match arg.parse::<i64>() {
                    Ok(n) => {
                        // TODO: Additional checks. There are probably parseable integers which
                        // shouldnt be accepted
                        if delim == ')' {
                            state = ParserState::ScanningOp;
                            out.push(Op::Mul(arg1, n));
                        } else {
                            state = ParserState::ScanningArg { delim: ')' };
                            arg1 = n;
                        }
                        // +1 for delimiter
                        i += arg.len() + 1;
                    }
                    Err(_) => {
                        state = ParserState::ScanningOp;
                        i += 1;
                    }
                }
            }
            ParserState::SkippingOp => {
                if i + 4 < input.len() && &input[i..(i + 4)] == "do()" {
                    state = ParserState::ScanningOp;
                    i += 4;
                } else {
                    i += 1;
                }
            }
        }
    }

    out
}

pub fn part_one(input: &str) -> Option<u64> {
    let operations = parse_one(input);
    let result = operations.iter().fold(0, |acc, op| match op {
        Op::Mul(x, y) => acc + x * y,
    });

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let operations = parse_two(input);
    let result = operations.iter().fold(0, |acc, op| match op {
        Op::Mul(x, y) => acc + x * y,
    });

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }

    #[test]
    fn backwards_compat() {
        let result_1 = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        let result_2 = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));

        assert_eq!(result_1, result_2);
    }
}
