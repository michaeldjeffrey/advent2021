use itertools::Itertools;
use std::{collections::HashMap, fs, str::FromStr};

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
    let mut uniq_patterns0 = vec![
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
    ];
    uniq_patterns0 = uniq_patterns0
        .iter_mut()
        .map(|s| s.chars().sorted().collect::<String>())
        .collect_vec();

    let mut uniq_patterns = uniq_patterns0.clone();
    uniq_patterns.sort();

    let mut all_ans = vec![];

    let mut seg_nums = input.lines().map(SegNum::from_str).collect_vec();
    for t in seg_nums.iter_mut() {
        if let Ok(tester) = t {
            // println!("{:?}", tester);

            let plain = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].iter().collect_vec();

            for perm in ['a', 'b', 'c', 'd', 'e', 'f', 'g'].iter().permutations(7) {
                let mut map = HashMap::new();
                for (&&key, &val) in plain.iter().zip(perm) {
                    map.insert(val, key);
                }

                let mut new_clues = vec![];
                for s in &tester.left {
                    let mut x = vec![];
                    for c in s.chars() {
                        x.push(*map.get(&c).unwrap());
                    }
                    x.sort();
                    let a = x.iter().collect::<String>();
                    new_clues.push(a);
                }

                let good = &uniq_patterns;

                new_clues.sort();
                let is_good = good.iter().zip(new_clues.iter()).all(|(l, r)| {
                    let eq = l == r;
                    // if eq {
                    // println!("{:?} , {:?} == {:?}", l, r, eq);
                    // }
                    eq
                });
                if is_good {
                    let mut rightside = vec![];
                    for d in &tester.right {
                        let mut pre_cleaned = vec![];
                        for c in d.chars() {
                            pre_cleaned.push(*map.get(&c).unwrap());
                        }
                        pre_cleaned.sort();
                        let cleaned = pre_cleaned.iter().collect::<String>();

                        let num = uniq_patterns0
                            .iter()
                            .position(|r| {
                                let eq = r == &cleaned;
                                // println!("{:?} == {:?}, {:?}", eq, r, &cleaned);
                                eq
                            })
                            .unwrap();
                        // println!("{:?}", num);
                        rightside.push(num.to_string());
                    }
                    let ans = rightside.join("").parse::<i32>().unwrap();
                    all_ans.push(ans);
                    println!("decoded: {:?}", ans);
                }
            }
        }
    }

    all_ans.iter().sum()
}

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let part1 = part1(file_input.clone());
    let part2 = part2(file_input.clone());

    println!("Part 1 : {}", part1);
    println!("Part 2 : {}", part2);
}
impl SegNum {}
