use itertools::{iterate, Itertools};
use std::{collections::HashMap, fs};

const SIZE: isize = 10;
const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), // up-left
    (-1, 0),  // up
    (-1, 1),  // up-right
    (0, -1),  // left
    (0, 1),   // right
    (1, -1),  // down-left
    (1, 0),   // down
    (1, 1),   // down-right
];

type Mapping = HashMap<(usize, usize), i32>;

fn inc_energy(map: &mut Mapping) -> i32 {
    let mut flashes = 0;
    let mut positions = map.clone().into_keys().collect_vec();
    while let Some(pos) = positions.pop() {
        let val = map.get_mut(&pos).unwrap();
        *val += 1;
        if *val == 10 {
            flashes += 1;
            positions.extend(get_neighbors(&pos));
        }
    }
    return flashes;
}

fn reset_flashed(map: &mut Mapping) {
    for val in map.values_mut() {
        if *val > 9 {
            *val = 0;
        }
    }
}

fn get_neighbors((y, x): &(usize, usize)) -> Vec<(usize, usize)> {
    return DIRECTIONS
        .iter()
        .map(|(dy, dx)| (*y as isize + *dy, *x as isize + *dx))
        .filter_map(|(y, x)| {
            if x < 0 || x >= SIZE {
                return None;
            }
            if y < 0 || y >= SIZE {
                return None;
            }
            return Some((y as usize, x as usize));
        })
        .collect_vec();
}

fn input_to_mapping(s: String) -> Mapping {
    let pos_vec = s
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
    let mut map: Mapping = HashMap::new();
    for (y, line) in pos_vec.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            map.insert((y, x), *c);
        }
    }
    return map;
}

fn step_board(map: &mut Mapping) -> i32 {
    let flashes = inc_energy(map);
    reset_flashed(map);
    flashes
}

fn part1(map: &mut Mapping, steps: usize) -> i32 {
    iterate(0, |&total| total + step_board(map))
        .take(steps + 1)
        .last()
        .unwrap()
}

fn part2(map: &mut Mapping) -> i32 {
    iterate(0, |_| step_board(map))
        .position(|flashes| flashes == 100)
        .unwrap() as i32
}

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let input_map = input_to_mapping(file_input);

    let part1 = part1(&mut input_map.clone(), 100);
    let part2 = part2(&mut input_map.clone());

    println!("Part 1 : {:?}", part1);
    println!("Part 2 : {:?}", part2);
}
