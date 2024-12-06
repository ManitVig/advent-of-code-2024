use std::{fs, iter::zip};

fn get_sorted_lists(input: std::str::Lines) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    
    input.for_each(|line| {
        let chars: Vec<i32> = line.split_whitespace().map(|entry| entry.parse::<i32>().expect("entry is supposed to be a number")).collect();
        left.push(chars[0]);
        right.push(chars[1]);
    });
    left.sort();
    right.sort();

    (left, right)
}

fn play(input: String) -> i32 {
    let lines = input.lines();
    let mut tot_distance = 0;

    let (left, right) = get_sorted_lists(lines);

    zip(left, right).for_each(|pair| {
        let distance = pair.0 - pair.1;
        tot_distance += distance.abs();
    });

    return tot_distance
} 

fn play2(input: String) -> i32 {
    let (left, right) = get_sorted_lists(input.lines());
    let mut tot_similarity = 0;


    left.iter().for_each(|l| {
        let mut occurr = 0;
        for r in right.iter() {
            if r > l {
                break;
            }
            if r == l {
                occurr += 1
            }
        }
        tot_similarity += l * occurr;
    });

    return tot_similarity
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input expected!");

    //dbg!(play(input));
    dbg!(play2(input));
}
