use std::{collections::HashMap, fs};

use itertools::{izip, Itertools};
use text_io::scan;

#[derive(Debug, PartialEq)]
enum Orientation {
    Vertical,
    Horizontal,
    Diagonal,
}

#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    orientation: Orientation,
}

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let input_lines = file_input.split("\n");

    let lines = input_lines.map(|l| Line::from_str(l)).collect_vec();

    let part1_answer = part1(&lines);
    let part2_answer = part2(&lines);

    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}

fn part2(lines: &Vec<Line>) -> i32 {
    let mut map = HashMap::new();
    for line in lines {
        let points = line.all_points();
        for point in points {
            *map.entry(point).or_insert(0) += 1;
        }
    }
    let mut count = 0;
    for val in map.values() {
        if *val > 1 {
            count += 1
        }
    }

    return count;
}

fn part1(lines: &Vec<Line>) -> i32 {
    let mut map = HashMap::new();
    for line in lines {
        if line.is_diagonal() {
            continue;
        }
        let points = line.all_points();
        for point in points {
            *map.entry(point).or_insert(0) += 1;
        }
    }
    let mut count = 0;
    for val in map.values() {
        if *val > 1 {
            count += 1
        }
    }
    // assert!(count != 3829, "too low");
    // assert!(count != 3845, "too low, added inclusive range");
    // assert!(count == 5443, "correct, fixed range order");
    return count;
}
impl Line {
    fn from_str(line: &str) -> Self {
        let x1: i32;
        let x2: i32;
        let y1: i32;
        let y2: i32;
        scan!(line.bytes() => "{},{} -> {},{}", x1, y1, x2, y2);
        let orientation = if x1 == x2 {
            Orientation::Horizontal
        } else if y1 == y2 {
            Orientation::Vertical
        } else {
            Orientation::Diagonal
        };
        Self {
            x1,
            y1,
            x2,
            y2,
            orientation,
        }
    }

    fn is_diagonal(&self) -> bool {
        self.orientation == Orientation::Diagonal
    }

    fn all_points(&self) -> Vec<(i32, i32)> {
        match self.orientation {
            Orientation::Vertical => {
                let range = create_range(self.x1, self.x2);
                let mapped = range.map(|x| (x, self.y1));
                mapped.collect_vec()
            }
            Orientation::Horizontal => {
                let range = create_range(self.y1, self.y2);
                let mapped = range.map(|y| (self.x1, y));
                mapped.collect_vec()
            }
            Orientation::Diagonal => {
                let x_range = create_range(self.x1, self.x2);
                let y_range = create_range(self.y1, self.y2);
                izip!(x_range, y_range).collect_vec()
            }
        }
    }
}

fn create_range(
    b: i32,
    e: i32,
) -> itertools::Either<impl Iterator<Item = i32>, impl Iterator<Item = i32>> {
    if b < e {
        itertools::Either::Left(b..=e)
    } else {
        itertools::Either::Right((e..=b).rev())
    }
}
