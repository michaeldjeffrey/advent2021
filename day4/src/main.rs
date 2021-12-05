use itertools::Itertools;
use std::{fs, str::Split, vec};

type Row = Vec<Vec<i32>>;

#[derive(Debug)]
struct Board {
    done: bool,
    rows: Row,
    columns: Row,
    active: Vec<i32>,
}

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let mut lines = file_input.split("\n");

    let nums_to_draw: Vec<i32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect_vec();

    let mut boards = lines_to_vec_nums(lines)
        .chunks(5)
        .map(|bi| Board::from_input(bi.to_vec()))
        .collect_vec();

    for (idx, num) in nums_to_draw.iter().enumerate() {
        for board in boards.iter_mut() {
            board.play(*num);
            if board.is_winner() {
                // Part 1: First printout
                // Part 2: Last printout
                println!("We found a winner at board {} - {}", idx, board.answer());
            }
        }
    }
}

fn lines_to_vec_nums(input: Split<&str>) -> Row {
    return input.clone().filter_map(parse_non_empty_row).collect_vec();
}

fn parse_non_empty_row(row: &str) -> Option<Vec<i32>> {
    if row.is_empty() {
        None
    } else {
        Some(
            row.trim()
                .split(" ")
                .filter_map(parse_non_empty_num)
                .collect_vec(),
        )
    }
}

fn parse_non_empty_num(num: &str) -> Option<i32> {
    if num.is_empty() {
        None
    } else {
        Some(num.parse().unwrap())
    }
}

impl Board {
    fn from_input(bi: Row) -> Self {
        let mut cols = vec![];
        for idx in 0..5 {
            let mut col = vec![];
            for i in bi.clone() {
                col.push(i[idx]);
            }
            cols.push(col);
        }
        return Self {
            done: false,
            rows: bi,
            columns: cols,
            active: vec![],
        };
    }

    fn play(&mut self, num: i32) -> () {
        if self.done {
            return;
        };
        self.active.push(num);
    }

    fn is_winner(&self) -> bool {
        if self.done {
            return false;
        }
        for row in &self.rows {
            if self.is_complete(row) {
                return true;
            }
        }
        for col in &self.columns {
            if self.is_complete(col) {
                return true;
            }
        }
        return false;
    }

    fn is_complete(&self, row: &Vec<i32>) -> bool {
        return row.iter().all(|n| self.active.contains(n));
    }

    fn answer(&mut self) -> i32 {
        self.done = true;
        let all_nums = self.rows.clone().into_iter().flatten().collect_vec();
        let x = all_nums.iter().filter(|num| !self.active.contains(num));
        let sum: i32 = x.sum();
        return sum * self.active.last().unwrap();
    }
}
