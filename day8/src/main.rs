use itertools::Itertools;
use std::{collections::HashMap, fs, str::FromStr};

type PermMap = HashMap<char, char>;

fn part1(input: String) -> i32 {
    input
        .lines()
        .map(|l| l.split(" | ").nth(1).unwrap().split(" "))
        .flatten()
        .filter(|l| match l.len() {
            2 => true,
            3 => true,
            4 => true,
            7 => true,
            _ => false,
        })
        .collect_vec()
        .len() as i32
}

#[derive(Debug)]
struct SegNum {
    left: Vec<String>,
    right: Vec<String>,
    mapping: HashMap<u8, u8>,
}

impl FromStr for SegNum {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, " | ");
        Ok(SegNum {
            left: parts
                .next()
                .unwrap()
                .split(" ")
                .map(String::from)
                .collect_vec(),
            right: parts
                .next()
                .unwrap()
                .split(" ")
                .map(|s| String::from(s).chars().sorted().collect::<String>())
                .collect_vec(),
            mapping: HashMap::new(),
        })
    }
}

fn part2(input: String) -> i32 {
    let patterns_in_order = vec![
        "cagedb".to_string(),  //: 0
        "ab".to_string(),      //: 1
        "gcdfa".to_string(),   //: 2
        "fbcad".to_string(),   //: 3
        "eafb".to_string(),    //: 4
        "cdfbe".to_string(),   //: 5
        "cdfgeb".to_string(),  //: 6
        "dab".to_string(),     //: 7
        "acedgfb".to_string(), //: 8
        "cefabd".to_string(),  //: 9
    ]
    .iter_mut()
    .map(sort_string)
    .collect_vec();

    let sorted_patterns = sorted(&patterns_in_order);
    let plain = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].iter().collect_vec();

    return input
        .lines()
        .map(|line| {
            let tester = SegNum::from_str(line).unwrap();
            return ['a', 'b', 'c', 'd', 'e', 'f', 'g']
                .iter()
                .permutations(7)
                .find_map(|perm| {
                    // Find the permutation where the sorted results equal the sorted known patterns
                    let perm_map = make_map_with_perm(&plain, &perm);
                    let new_left = apply_perm_map(&perm_map, &tester.left);
                    if all_elements_equal(&sorted_patterns, &new_left) {
                        Some(perm_map)
                    } else {
                        None
                    }
                })
                .and_then(|perm_map| {
                    // Take the good perm_map and apply it to the answer
                    let ans = tester
                        .right
                        .iter()
                        .map(|s| {
                            let cleaned = clue_from_perm_map(&perm_map, s);
                            let num = patterns_in_order
                                .iter()
                                .position(|r| r == &cleaned)
                                .unwrap();
                            num
                        })
                        .join("")
                        .parse::<i32>()
                        .unwrap();

                    return Some(ans);
                })
                .unwrap();
        })
        .sum();
}

fn sorted(l: &Vec<String>) -> Vec<String> {
    let mut l2 = l.clone();
    l2.sort();
    return l2;
}

fn sort_string(s: &mut String) -> String {
    s.chars().sorted().collect()
}

fn apply_perm_map(perm_map: &HashMap<char, char>, left: &Vec<String>) -> Vec<String> {
    return left
        .iter()
        .map(|s| clue_from_perm_map(&perm_map, s))
        .sorted()
        .collect_vec();
}

fn all_elements_equal(left: &Vec<String>, right: &Vec<String>) -> bool {
    left.iter().zip(right).all(|(l, r)| l == r)
}

fn clue_from_perm_map(perm_map: &PermMap, string: &String) -> String {
    let mut x = vec![];
    for c in string.chars() {
        x.push(*perm_map.get(&c).unwrap());
    }
    x.sort();
    let a = x.iter().collect::<String>();
    return a;
}

fn make_map_with_perm(base: &Vec<&char>, perm: &Vec<&char>) -> PermMap {
    let mut map = HashMap::new();
    for (&&key, &&val) in base.iter().zip(perm) {
        map.insert(val, key);
    }
    map
}

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let part1 = part1(file_input.clone());
    let part2 = part2(file_input.clone());

    println!("Part 1 : {}", part1);
    println!("Part 2 : {}", part2);
}
impl SegNum {}
