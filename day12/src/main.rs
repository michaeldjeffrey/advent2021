use itertools::Itertools;
use std::{collections::HashMap, fs};
use text_io::scan;

type Visited<'a> = Vec<&'a String>;
type CaveSystem = HashMap<String, Vec<String>>;
type Cave = String;

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let system = input_to_cave_system(file_input);

    let start = &String::from("start");

    let mut part_1 = 0;
    let mut part_2 = 0;
    let empty = &Vec::new();
    for cave in system.get(start).unwrap().clone() {
        let visited = new_visited_with(&empty, &start);
        explore(
            cave.clone(),
            visited.clone(),
            &system,
            part_1_can_continue,
            &mut part_1,
        );
        explore(
            cave.clone(),
            visited.clone(),
            &system,
            part_2_can_continue,
            &mut part_2,
        );
    }

    println!("Part 1 : {:?}", part_1);
    println!("Part 2 : {:?}", part_2);
}

fn explore(
    cave: Cave,
    visited: Visited,
    system: &CaveSystem,
    can_continue_fn: fn(cave: &String, visited: &Vec<&String>) -> bool,
    ends: &mut i32,
) -> () {
    if cave == "end" {
        *ends += 1;
        return;
    }

    if can_continue_fn(&cave, &visited) {
        return;
    }

    for neighbor in system.get(&cave).unwrap().iter() {
        explore(
            neighbor.clone(),
            new_visited_with(&visited, &cave),
            system,
            can_continue_fn,
            ends,
        );
    }
}

fn new_visited_with<'a>(visited: &'a Vec<&String>, cave: &'a String) -> Vec<&'a String> {
    let mut n = visited.clone();
    n.push(cave);
    return n;
}

fn part_1_can_continue(cave: &String, visited: &Vec<&String>) -> bool {
    is_lowercase(cave.as_str()) && visited.contains(&cave)
}

fn part_2_can_continue(cave: &String, visited: &Vec<&String>) -> bool {
    part_1_can_continue(cave, visited)
        && visited
            .clone()
            .iter()
            .filter(|&&x| is_lowercase(x))
            .counts()
            .values()
            .any(|x| *x == 2)
}

fn is_lowercase(cave: &str) -> bool {
    let a = cave.chars().nth(0).unwrap() as char;
    let b = cave.chars().nth(0).unwrap() as char;
    let b2 = b.to_lowercase().to_string().chars().nth(0).unwrap() as char;

    b2 == a
}

fn input_to_cave_system(input: String) -> CaveSystem {
    let pairs = input.lines().map(|l| parse_line(l)).collect_vec();

    let mut map: CaveSystem = HashMap::new();
    for (start, end) in pairs {
        match (start.as_str(), end.as_str()) {
            // Don't go back to start
            ("start", _) => map.entry(start.clone()).or_insert(vec![]).push(end.clone()),
            // Don't come back from end
            (_, "end") => map.entry(start.clone()).or_insert(vec![]).push(end.clone()),
            _ => {
                map.entry(start.clone()).or_insert(vec![]).push(end.clone());
                map.entry(end.clone()).or_insert(vec![]).push(start.clone())
            }
        }
    }

    return map;
}

fn parse_line(l: &str) -> (String, String) {
    let left: String;
    let right: String;

    scan!(l.bytes() => "{}-{}", left, right);
    (left, right)
}
