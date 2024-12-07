use std::fs;

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
    dbg!(play_part1(input));
}
