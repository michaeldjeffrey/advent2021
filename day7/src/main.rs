use itertools::Itertools;
use std::fs;

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let nums: Vec<i64> = file_input
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect_vec();

    let low = *nums.iter().min().unwrap();
    let high = *nums.iter().max().unwrap();

    let mut part1_fuel_usage = Vec::new();
    let mut part2_fuel_usage = Vec::new();

    for target in low..=high {
        let mut part1_fuel = 0;
        let mut part2_fuel = 0;
        for &pos in &nums {
            let change = abs(pos, target);
            part1_fuel += change;
            part2_fuel += binomial(change);
        }
        part1_fuel_usage.push(part1_fuel);
        part2_fuel_usage.push(part2_fuel);
    }
    let part1 = part1_fuel_usage.iter().min().unwrap();
    let part2 = part2_fuel_usage.iter().min().unwrap();

    println!("Part 1 : {:?}", part1);
    println!("Part 2 : {:?}", part2);
}

fn abs(a: i64, b: i64) -> i64 {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn binomial(n: i64) -> i64 {
    (n * (n + 1)) / 2
}
