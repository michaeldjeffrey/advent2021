use std::fs;

use itertools::Itertools;

type Input = Vec<Vec<i32>>;

const DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), // up
    (1, 0),  // down
    (0, -1), // left
    (0, 1),  // right
];

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let input = file_input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|n| {
                    return n.to_digit(10).unwrap() as i32;
                })
                .collect_vec()
        })
        .collect_vec();

    let get_neighbs = |y, x| get_neighbs(&input.clone(), y, x);
    let mut result = 0;

    for (y, nums) in input.iter().enumerate() {
        for (x, val) in nums.iter().enumerate() {
            if get_neighbs(y as isize, x as isize).iter().all(|n| n > val) {
                result += val + 1;
            }
        }
    }

    println!("Part 1 : {}", result);
}

fn get_neighbs(input: &Input, y: isize, x: isize) -> Vec<i32> {
    let height = input.len() as isize;
    let width = input[0].len() as isize;
    return DIRECTIONS
        .iter()
        .filter_map(move |(dy, dx)| {
            let ny = y + dy;
            let nx = x + dx;
            if nx < 0 || nx >= width {
                return None;
            }
            if ny < 0 || ny >= height {
                return None;
            }
            return Some(input[ny as usize][nx as usize]);
        })
        .collect_vec();
}
