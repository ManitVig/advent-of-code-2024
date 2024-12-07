use std::fs;

#[derive(Debug)]
enum Instructions {
    Mul(i32, i32),
    Do,
    Dont
}

fn build_modifier_stack(stack: &mut Vec<Instructions>, mul_token_idx: usize, do_token: Option<usize>, dont_token: Option<usize>) {
    match (do_token, dont_token) {
        (Some(do_idx), Some(dont_idx)) => {
            if do_idx < mul_token_idx && dont_idx < mul_token_idx {
                if do_idx > dont_idx {
                    if do_idx < mul_token_idx {
                        stack.push(Instructions::Do);
                    }
                } else {
                    if dont_idx < mul_token_idx {
                        stack.push(Instructions::Dont);
                    }
                }
            } else {
                if do_idx < mul_token_idx {
                    stack.push(Instructions::Do);
                }
                if dont_idx < mul_token_idx {
                    stack.push(Instructions::Dont);
                }
            }
        },
        (Some(do_idx), None) => {
            if do_idx < mul_token_idx {
                stack.push(Instructions::Do);
            }
        },
        (None, Some(dont_idx)) => {
            if dont_idx < mul_token_idx {
                stack.push(Instructions::Dont);
            }
        },
        (None, None) => {return}
    }
}

fn play_part2(input: String) -> i32 {
    let mut remaining_input = input.as_str();
    let mut instruction_queue: Vec<Instructions> = vec![];
    let mut modifier_stack: Vec<Instructions> = vec![];
    while let Some(mul_token) = remaining_input.find("mul(") {
        let do_idx = remaining_input.find("do()");
        let dont_idx = remaining_input.find("don't()");
        build_modifier_stack(&mut modifier_stack, mul_token, do_idx, dont_idx);
        let mut arg_idx = mul_token + 4;
        remaining_input = &remaining_input[arg_idx..];
        let mut num1: String = String::new();
        let mut chars = remaining_input.chars();
        let mut char = chars.next();
        while let Some(c) = char {
            if c.is_digit(10) {
                num1 += c.to_string().as_str();
                char = chars.next();
            } else {
                break
            }
        }
        if num1.len() == 0 {continue}
        arg_idx = num1.len();
        remaining_input = &remaining_input[arg_idx..];
        if !(remaining_input.chars().nth(0).unwrap() == ',') {
            continue
        }
        remaining_input = &remaining_input[1..];
        let mut num2: String = String::new();
        let mut chars = remaining_input.chars();
        let mut char = chars.next();
        while let Some(c) = char {
            if c.is_digit(10) {
                num2 += c.to_string().as_str();
                char = chars.next();
            } else {
                break
            }
        }
        if num1.len() == 0 {continue}
        arg_idx = num2.len();
        remaining_input = &remaining_input[arg_idx..];
        if !(remaining_input.chars().nth(0).unwrap() == ')') {
            continue
        }
        if let Some(modifier) = modifier_stack.pop() {
            instruction_queue.push(modifier);
            modifier_stack.clear();
        }
        instruction_queue.push(Instructions::Mul(num1.parse().unwrap(), num2.parse().unwrap()));
        remaining_input = &remaining_input[1..];
    }
    let mut total = 0;
    let mut is_active = true;
    for instruction in instruction_queue.iter() {
        match instruction {
            Instructions::Dont => {
                is_active = false;
            },
            Instructions::Do => {
                is_active = true;
            },
            Instructions::Mul(n1, n2) => {
                if is_active {
                    total += n1 * n2;
                }
            }
        }
    }
    return total
}

fn play_part1(input: String) -> i32 {
    let mut remaining_input = input.as_str();
    let mut safe_muls: Vec<(i32, i32)> = vec![];
    while let Some(mul_token) = remaining_input.find("mul(") {
        let mut arg_idx = mul_token + 4;
        remaining_input = &remaining_input[arg_idx..];
        let mut num1: String = String::new();
        let mut chars = remaining_input.chars();
        let mut char = chars.next();
        while let Some(c) = char {
            if c.is_digit(10) {
                num1 += c.to_string().as_str();
                char = chars.next();
            } else {
                break
            }
        }
        if num1.len() == 0 {continue}
        arg_idx = num1.len();
        remaining_input = &remaining_input[arg_idx..];
        if !(remaining_input.chars().nth(0).unwrap() == ',') {
            continue
        }
        remaining_input = &remaining_input[1..];
        let mut num2: String = String::new();
        let mut chars = remaining_input.chars();
        let mut char = chars.next();
        while let Some(c) = char {
            if c.is_digit(10) {
                num2 += c.to_string().as_str();
                char = chars.next();
            } else {
                break
            }
        }
        if num1.len() == 0 {continue}
        arg_idx = num2.len();
        remaining_input = &remaining_input[arg_idx..];
        if !(remaining_input.chars().nth(0).unwrap() == ')') {
            continue
        }
        safe_muls.push((num1.parse().unwrap(), num2.parse().unwrap()));
        remaining_input = &remaining_input[1..];
    }
    return safe_muls.iter().map(|mul| mul.0 * mul.1).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input expected!");
    //dbg!(play_part1(input));
    dbg!(play_part2(input));
}
