use itertools::Itertools;
use std::{collections::HashMap, fs};
use text_io::scan;

type BigAMap = HashMap<(char, char), u128>;

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();

    let part1 = part1(file_input.clone(), 10);
    println!("Part 1 : {:?}", part1);

    let part2 = part2(file_input.clone(), 40);
    println!("Part 2 : {:?}", part2);
}

fn part2(file_input: String, steps: usize) -> u128 {
    let mut input = file_input.split("\n\n");
    let start = String::from(input.next().unwrap());
    let mut pairs: HashMap<(char, char), char> = HashMap::new();
    input
        .next()
        .unwrap()
        .lines()
        .map(parse_line)
        .for_each(|(key, val)| {
            pairs.insert(key, val);
            ()
        });

    let starter_map =
        start
            .chars()
            .collect_vec()
            .windows(2)
            .fold(HashMap::new(), |mut map, chars| match chars {
                &[left, right] => {
                    *map.entry((left, right)).or_insert(0) += 1;
                    return map;
                }
                _ => panic!("wat"),
            });

    let doer = itertools::iterate(starter_map, |map| {
        let mut new_map: BigAMap = HashMap::new();

        for (&(left, right), &val) in map.iter() {
            let &middle = pairs.get(&(left, right)).unwrap();
            *new_map.entry((left, middle)).or_insert(0) += val;
            *new_map.entry((middle, right)).or_insert(0) += val;
        }
        return new_map;
    });

    let chain = doer.clone().take(steps + 1).last().unwrap();

    let mut individuals = HashMap::new();
    for (&(left, _right), &val) in chain.iter() {
        *individuals.entry(left).or_insert(0) += val;
    }
    let last = start.chars().last().unwrap();
    *individuals.entry(last).or_insert(0) += 1;

    let polymer_counts = individuals.values();
    let part2 = polymer_counts.clone().max().unwrap() - polymer_counts.clone().min().unwrap();

    return part2;
}

fn part1(file_input: String, steps: usize) -> i32 {
    let mut input = file_input.split("\n\n");
    let start = String::from(input.next().unwrap());
    let mut pairs: HashMap<(char, char), char> = HashMap::new();
    input
        .next()
        .unwrap()
        .lines()
        .map(parse_line)
        .for_each(|(key, val)| {
            pairs.insert(key, val);
            ()
        });
    let doer = itertools::iterate(start, |s| {
        let mut new_s = s
            .chars()
            .collect_vec()
            .windows(2)
            .flat_map(|cs| match cs {
                [l, r] => {
                    let m = pairs.get(&(*l, *r)).unwrap();
                    [l, m]
                }
                _ => panic!("wat"),
            })
            .collect::<String>();
        new_s.push_str(String::from(s.chars().last().unwrap()).as_str());
        return new_s;
    });

    let chain = doer.clone().take(steps + 1).last().unwrap();
    let chain_chars = chain.chars();
    let counts = chain_chars.counts();
    let polymer_counts = counts.values();
    let part1 = polymer_counts.clone().max().unwrap() - polymer_counts.clone().min().unwrap();

    return part1 as i32;
}

fn parse_line(l: &str) -> ((char, char), char) {
    let left: String;
    let right: char;

    scan!(l.bytes() => "{} -> {}", left, right);
    let mut chars = left.chars();
    let ll = chars.next().unwrap();
    let lr = chars.next().unwrap();
    ((ll, lr), right)
}
