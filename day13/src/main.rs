use itertools::Itertools;
use std::{collections::HashMap, fs};
use text_io::scan;

type Paper = HashMap<(i32, i32), i32>;

#[derive(Debug)]
enum Fold {
    Hotdog(i32),
    Hamburger(i32),
}

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let mut input = file_input.split("\n\n");
    let points = points_to_map(input.next().unwrap());
    let folds = folds_to_folds(input.next().unwrap());

    let part2 = folds.iter().fold(points, |paper, big_fold| match big_fold {
        &Fold::Hotdog(x_fold) => {
            let mut next_map: Paper = HashMap::new();
            for &(y, x) in paper.keys() {
                let new_x = if x > x_fold { x_fold - (x - x_fold) } else { x };
                next_map.insert((y, new_x), 1);
            }
            next_map
        }
        &Fold::Hamburger(y_fold) => {
            let mut next_map: Paper = HashMap::new();
            for &(y, x) in paper.keys() {
                let new_y = if y > y_fold { y_fold - (y - y_fold) } else { y };
                next_map.insert((new_y, x), 1);
            }
            next_map
        }
    });

    print_paper(&part2);
}

fn print_paper(paper: &Paper) -> () {
    let mut max_x = 0;
    let mut max_y = 0;
    for &(y, x) in paper.keys() {
        if x > max_x {
            max_x = x
        }
        if y > max_y {
            max_y = y
        }
    }
    for y in 0..=max_y {
        for x in 0..=max_x {
            match paper.get(&(y, x)) {
                Some(_) => print!("#"),
                None => print!("."),
            }
        }
        print!("\n");
    }
}

fn folds_to_folds(input: &str) -> Vec<Fold> {
    input.lines().map(|l| parse_fold(l)).collect_vec()
}

fn points_to_map(input: &str) -> Paper {
    let pairs = input.lines().map(|l| parse_pair(l)).collect_vec();

    let mut map: Paper = HashMap::new();
    for (x, y) in pairs {
        map.insert((y, x), 1);
    }

    return map;
}

fn parse_fold(l: &str) -> Fold {
    let dir: String;
    let line: i32;

    scan!(l.bytes() => "fold along {}={}", dir, line);
    match dir.as_str() {
        "x" => Fold::Hotdog(line),
        "y" => Fold::Hamburger(line),
        _ => panic!("unknown food"),
    }
}
fn parse_pair(l: &str) -> (i32, i32) {
    let left: i32;
    let right: i32;

    scan!(l.bytes() => "{},{}", left, right);
    (left, right)
}
