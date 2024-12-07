use std::fs;
mod part1;
mod part2;

use part1::play_part1;
use part2::play_part2;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input expected!");
    
    let reports: Vec<Vec<i32>> = input.trim().lines().map(|line| {
        line.split_whitespace().map(|inp| inp.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    }).collect();

   // dbg!(play_part1(reports));
   dbg!(play_part2(reports));
}
