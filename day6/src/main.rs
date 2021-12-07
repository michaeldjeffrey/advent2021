use itertools::Itertools;
use std::{collections::HashMap, fs};
type Cache = HashMap<(i64, i64), i64>;

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let nums: Vec<i64> = file_input
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect_vec();

    let mut cache: Cache = HashMap::new();

    let part1_dp = |&ttl| run(&mut cache, ttl, 80);
    let part1: i64 = nums.iter().map(part1_dp).sum();

    let part2_dp = |&ttl| run(&mut cache, ttl, 256);
    let part2: i64 = nums.iter().map(part2_dp).sum();

    // let over_days = |days| mk_dp(&mut cache, days);
    // let part1 = nums.iter().map(over_days(80)).sum();
    // let part2 = nums.iter().map(over_days(256)).sum();

    println!("Part 1 : {:?}", part1);
    println!("Part 2 : {:?}", part2);
}

fn run(cache: &mut Cache, ttl: i64, days: i64) -> i64 {
    if ttl >= days {
        return 1;
    }
    match cache.get(&(ttl, days)) {
        Some(&result) => result,
        None => {
            let left = days - ttl - 1;
            let result = run(cache, 6, left) + run(cache, 8, left);

            cache.insert((ttl, days), result);
            result
        }
    }
}
