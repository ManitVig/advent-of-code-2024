use std::{fs, usize};

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize
}

fn search_horizontal(matrix: &Vec<Vec<String>>, x: usize, y: usize) -> (String, Vec<Point>) {
    let mut out = String::new();
    let mut points: Vec<Point> = vec![];
    for i in x..x+4 {
        out += matrix[y][i].as_str();
        points.push(Point {
                x: i,
                y,
            });
    }
    return (out, points)
}

fn search_horizontal_neg(matrix: &Vec<Vec<String>>, x: usize, y: usize) -> (String, Vec<Point>) {
    let mut out = String::new();
    let mut points: Vec<Point> = vec![];
    for i in (x-3..=x).rev() {
        out += matrix[y][i].as_str();
        points.push(Point {
                x: i,
                y,
            });
    }
    return (out, points)
}

fn search_vertical(matrix: &Vec<Vec<String>>, x: usize, y: usize) -> (String, Vec<Point>) {
    let mut out = String::new();
    let mut points: Vec<Point> = vec![];
    for i in y..y+4 {
        out += matrix[i][x].as_str();
        points.push(Point {
                x,
                y: i,
            });
    }
    return (out, points)
}

fn search_vertical_neg(matrix: &Vec<Vec<String>>, x: usize, y: usize) -> (String, Vec<Point>) {
    let mut out = String::new();
    let mut points: Vec<Point> = vec![];
    for i in (y-3..=y).rev() {
        out += matrix[i][x].as_str();
        points.push(Point {
                x,
                y: i,
            });
    }
    return (out, points)
}

fn search_diag1(matrix: &Vec<Vec<String>>, x: usize, y: usize) -> (String, Vec<Point>) {
    let mut out = String::new();
    let mut points: Vec<Point> = vec![];
    let mut i = x;
    let mut j = y;
    while i < x+4 && j < y+4 {
        out += matrix[j][i].as_str();
        points.push(Point {
                x: i,
                y: j,
            });
        i += 1;
        j += 1;
    }
    return (out, points)
}

fn search_diag2(matrix: &Vec<Vec<String>>, x: usize, y: usize) -> (String, Vec<Point>) {
    let mut out = String::new();
    let mut points: Vec<Point> = vec![];
    let mut i = x;
    let mut j = y;
    while i < x+4 && j >= y-3 {
        out += matrix[j][i].as_str();
        points.push(Point {
                x: i,
                y: j,
            });
        if j == 0 {break};
        i += 1;
        j -= 1;
    }
    return (out, points)
}

fn search_diag1_neg(matrix: &Vec<Vec<String>>, x: usize, y: usize) -> (String, Vec<Point>) {
    let mut out = String::new();
    let mut points: Vec<Point> = vec![];
    let mut i = x;
    let mut j = y;
    while i >= x-3 && j >= y-3 {
        out += matrix[j][i].as_str();
        points.push(Point {
                x: i,
                y: j,
            });
        if i == 0 || j == 0 {break};
        i -= 1;
        j -= 1;
    }
    return (out, points)
}

fn search_diag2_neg(matrix: &Vec<Vec<String>>, x: usize, y: usize) -> (String, Vec<Point>) {
    let mut out = String::new();
    let mut points: Vec<Point> = vec![];
    let mut i = x;
    let mut j = y;
    while i >= x-3 && j < y+4 {
        out += matrix[j][i].as_str();
        points.push(Point {
                x: i,
                y: j,
            });
        if i == 0 {break};
        i -= 1;
        j += 1;
    }
    return (out, points)
}

fn search_x(matrix: &Vec<Vec<String>>, x:usize, y: usize) -> (String, String) {
    let mut diag1 = String::new();
    let mut diag2 = String::new();

    diag1 += (matrix[y-1][x-1].clone() + &matrix[y][x] + &matrix[y+1][x+1]).as_str();
    diag2 += (matrix[y-1][x+1].clone() + &matrix[y][x] + &matrix[y+1][x-1]).as_str();

    return (diag1, diag2)
}

fn count_xmas(xmas_list: &mut Vec<Vec<Point>>, search: String, mut points: Vec<Point>) {
    if search == "XMAS".to_string() {
        if !(xmas_list.contains(&points)) {
            xmas_list.push(points);
        }
    }else if search == "SAMX".to_string() {
        points.reverse();
        if !(xmas_list.contains(&points)) {
            xmas_list.push(points);
        }
    }
}

fn play_part1(input: String) -> usize {
    let matrix: Vec<Vec<String>> = input.lines().map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<_>>()).collect();
    let mut xmas: Vec<Vec<Point>> = vec![];
    for y in 0..matrix.len() {
        let row = &matrix[y];
        for x in 0..row.len() {
            if x+4 < row.len() {
                let (search, points) = search_horizontal(&matrix, x, y);
                count_xmas(&mut xmas, search, points);
            }
            if x >= 3 {
                let (search, points) = search_horizontal_neg(&matrix, x, y);
                count_xmas(&mut xmas, search, points);
            }
            if y+4 < matrix.len() {
                let (search, points) = search_vertical(&matrix, x, y);
                count_xmas(&mut xmas, search, points);
            }
            if y >= 3 {
                let (search, points) = search_vertical_neg(&matrix, x, y);
                count_xmas(&mut xmas, search, points);
            }
            if x+4 < row.len() && y+4 < matrix.len() {
                let (search, points) = search_diag1(&matrix, x, y);
                count_xmas(&mut xmas, search, points);
            }
            if x >= 3 && y >= 3 {
                let (search, points) = search_diag1_neg(&matrix, x, y);
                count_xmas(&mut xmas, search, points);
            }
            if x+4 < row.len() && y >= 3{
                let (search, points) = search_diag2(&matrix, x, y);
                count_xmas(&mut xmas, search, points);
            }
            if x >= 3 && y+4 < matrix.len() {
                let (search, points) = search_diag2_neg(&matrix, x, y);
                count_xmas(&mut xmas, search, points);
            }

        }
    }
    return xmas.len()
}

fn play_part2(input: String) -> u32 {
    let matrix: Vec<Vec<String>> = input.lines().map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<_>>()).collect();
    let mut xmas_count = 0;
    for y in 0..matrix.len() {
        let row = &matrix[y];
        for x in 0..row.len() {
            let curr = &matrix[y][x];
            if curr == "A" {
                if y >= 1 && x >= 1 && x+1 < row.len() && y+1 < matrix.len() {
                    let (s1, s2) = search_x(&matrix, x, y);
                    if (s1 == "MAS" || s1 == "SAM") && (s2 == "MAS" || s2 == "SAM") {
                        xmas_count += 1
                    }
                } 
            }
        }
    }
    return xmas_count
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input expected!");
    //dbg!(play_part1(input));
    dbg!(play_part2(input));
}
