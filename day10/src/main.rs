use std::{fs, str::Chars};

use itertools::Itertools;

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let lines = file_input.lines().map(|l| l.chars()).collect_vec();

    let part1 = part1(lines.clone());
    let part2 = part2(lines.clone());

    println!("Part 1 : {:?}", part1);
    println!("Part 2 : {:?}", part2);
}

fn part2(lines: Vec<Chars>) -> i64 {
    let mut scores = lines
        .into_iter()
        .filter_map(|line| {
            let mut stack = vec![];
            for c in line {
                match c {
                    '{' | '[' | '(' | '<' => stack.push(c),
                    '}' | ']' | ')' | '>' => {
                        let left = stack.pop().unwrap();
                        match (left, c) {
                            ('{', '}') => continue,
                            ('(', ')') => continue,
                            ('<', '>') => continue,
                            ('[', ']') => continue,
                            (expected, found) => {
                                let score = part1_score(c);
                                println!(
                                    "expected: {}, found: {}, score: {}",
                                    expected, found, score
                                );
                                return None;
                            }
                        }
                    }
                    _ => panic!("wat"),
                }
            }
            return Some(stack);
        })
        .map(|stack| {
            stack
                .clone()
                .iter()
                .rev()
                .map(into_pair)
                .map(part2_score)
                .fold(0, |acc, num| (5 * acc) + num)
        })
        .collect_vec();
    let len = scores.iter().len();
    scores.sort();
    let pos = (len - 1) / 2;
    println!("[{}] {:?}", pos, scores);

    scores[pos]
}

fn part1(lines: Vec<Chars>) -> i32 {
    let mut result = 0;
    for line in lines {
        let mut stack = vec![];
        for c in line {
            match c {
                '{' | '[' | '(' | '<' => stack.push(c),
                '}' | ']' | ')' | '>' => {
                    let left = stack.pop().unwrap();
                    match (left, c) {
                        ('{', '}') => continue,
                        ('(', ')') => continue,
                        ('<', '>') => continue,
                        ('[', ']') => continue,
                        (expected, found) => {
                            let score = part1_score(c);
                            println!("expected: {}, found: {}, score: {}", expected, found, score);
                            result += score;
                        }
                    }
                }
                _ => panic!("wat"),
            }
        }
    }
    return result;
}

fn into_pair(c: &char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("invalid input"),
    }
}

fn part2_score(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("invalid input"),
    }
}

fn part1_score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid input"),
    }
}
