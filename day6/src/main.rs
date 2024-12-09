use std::{fs, usize};

#[derive(Debug, PartialEq, Clone)]
struct Pos {
    x: usize,
    y: usize
}

struct Map {
    obstacles: Vec<Pos>,
    player_pos: Pos,
    ymax: usize,
    xmax: usize
}

fn parse_map(input: String) -> Map {
    let map_raw: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let ymax = map_raw.len() - 1;
    let xmax = map_raw[0].len() - 1;

    let mut obstacles: Vec<Pos> = vec![];
    let mut player_pos: Pos = Pos {x: 0, y: 0};

    for y in 0..=ymax {
        for x in 0..=xmax {
            let curr = map_raw[y][x];
            if curr == '#' {
                obstacles.push(Pos {x, y});
            } else if curr == '^' {
                player_pos = Pos{x, y};
            }
        }
    }

    Map {
        obstacles,
        player_pos,
        ymax,
        xmax
    }
}

fn turn_right(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        (0, 1) => (-1, 0),
        (1, 0) => (0, 1),
        (0, -1) => (1, 0),
        (-1, 0) => (0, -1),
        _ => panic!("invalid direction")
    }
}

fn move_pos(pos: &Pos, direction: &(i32, i32)) -> Pos {
    let mut new_x = pos.x;
    let mut new_y = pos.y;
    if direction.0 >= 0 {
        new_x += direction.0 as usize;
    } else {
        new_x -= direction.0.abs() as usize;
    }

    if direction.1 >= 0 {
        new_y += direction.1 as usize;
    } else {
        new_y -= direction.1.abs() as usize;
    }

    Pos{x: new_x, y: new_y}
} 

fn play_part1(input: String) -> usize {
    let mut map: Map = parse_map(input);
    let mut direction: (i32, i32) = (0, -1);
    let mut steps: Vec<Pos> = vec![map.player_pos.clone(),];
    while map.player_pos.x < map.xmax && map.player_pos.y < map.ymax && map.player_pos.x >= 0 && map.player_pos.y >= 0{
        let next_pos = move_pos(&map.player_pos, &direction);
        if map.obstacles.iter().find(|p| *p == &next_pos).is_some() {
            direction = turn_right(direction);
            continue
        } else {
            map.player_pos = next_pos;
            if steps.iter().find(|pos| *pos == &map.player_pos).is_none(){
                steps.push(map.player_pos.clone());
            }
        }
    } 
    return steps.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input expected!");
    assert_eq!(play_part1(input), 41)
}
