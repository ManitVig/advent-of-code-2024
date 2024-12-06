use std::{fs, iter::zip};

fn play(input: String) -> i32 {
    let lines = input.lines();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    let mut tot_distance = 0;

    lines.for_each(|line| {
        let chars: Vec<i32> = line.split_whitespace().map(|entry| entry.parse::<i32>().expect("entry is supposed to be a number")).collect();
        left.push(chars[0]);
        right.push(chars[1]);
    });
    left.sort();
    right.sort();

    zip(left, right).for_each(|pair| {
        let distance = pair.0 - pair.1;
        tot_distance += distance.abs();
    });

    return tot_distance
} 

fn main() {
    let input = fs::read_to_string("input.txt").expect("input expected!");

    dbg!(play(input));
}
