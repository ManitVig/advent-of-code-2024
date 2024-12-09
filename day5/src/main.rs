use std::{fs, thread::current};

#[derive(Debug, PartialEq)]
struct Rule {
    first: u32,
    second: u32
}

fn parse_rules(rules: &str) -> Vec<Rule> {
    rules.lines().map(|line| {
        let (first, second) = line.split_once("|").unwrap();
        Rule {
            first: first.parse().unwrap(),
            second: second.parse().unwrap()
        }
    }).collect()
}

fn is_safe_update(rules: &Vec<Rule>, update: &Vec<u32>) -> bool {
    for i in 0..update.len()-1 {
        let curr = update[i];
        for j in update[i..].iter() {
            if rules.iter().find(|rule| **rule == Rule {first: *j, second: curr}).is_some() {
                return false
            }
        }
    }
    return true
}

fn get_middle_sum(updates: &Vec<Vec<u32>>) -> u32 {
    updates.iter().map(|update| update[((update.len() + 1)/2) - 1]).sum()
}

fn play_part1(input: String) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules);
    let updates: Vec<Vec<u32>> = updates.lines().map(|line| {
        let split = line.split(",");
        split.map(|num| num.parse().unwrap()).collect()
    }).collect();
    let valid_updates: Vec<Vec<u32>> = updates.iter().filter(|update| is_safe_update(&rules, update)).map(|update| update.to_vec()).collect();
    return get_middle_sum(&valid_updates)
}

fn fix_update<'a>(mut update: Vec<u32>, rules: &Vec<Rule>) -> Vec<u32> {
    let mut i = 0;
    while i < update.len()-1 {
        let mut j = i+1;
        let curr = update[i];
        let mut updated = false;
        while j < update.len() {
            let cursor = update[j];
            if rules.iter().find(|rule| **rule == Rule {first: cursor, second: curr}).is_some() {
                update.swap(i, j);
                updated = true;
                break;
            } else {
                j += 1;
            }
        }
        if updated {
            i = 0;
        } else {
            i += 1;
        }
    }

    return update
}

fn play_part2(input: String) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules);
    let updates: Vec<Vec<u32>> = updates.lines().map(|line| {
        let split = line.split(",");
        split.map(|num| num.parse().unwrap()).collect()
    }).collect();
    let invalid_updates: Vec<Vec<u32>> = updates.iter().filter(|update| !(is_safe_update(&rules, *update))).map(|update| update.to_vec()).collect();
    let corrected_updates: Vec<Vec<u32>> = invalid_updates.iter().map(|update| fix_update(update.to_vec(), &rules)).collect();
    return get_middle_sum(&corrected_updates)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input expected!");
    dbg!(play_part2(input));
}
