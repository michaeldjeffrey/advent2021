use itertools::Itertools;
extern crate fstream;

type BitCriteria = fn(&String, usize, char) -> bool;
type Input<'a> = &'a Vec<String>;

fn main() {
    let input = read_file("./src/input.txt");

    let (gamma, epsilon) = part1(&input);
    let (oxygen, co2) = part2(&input);

    println!("part 1: {} * {} = {}", gamma, epsilon, gamma * epsilon);
    println!("part 2: {} * {} = {}", oxygen, co2, oxygen * co2);
}

fn part1(input: Input) -> (i32, i32) {
    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for idx in 0..input[0].len() {
        let most_common = most_common_in_column(idx, input);

        if most_common == 0 {
            gamma.push_str("0");
            epsilon.push_str("1")
        } else {
            gamma.push_str("1");
            epsilon.push_str("0")
        }
    }
    let g = binary_to_base10(gamma);
    let e = binary_to_base10(epsilon);
    return (g, e);
}

fn part2(input: Input) -> (i32, i32) {
    let len = input[0].len();

    let oxy_bit_criteria: BitCriteria = |line, idx, mc| starts_with_most_common(line, idx, mc);
    let co2_bit_criteria: BitCriteria = |line, idx, mc| !starts_with_most_common(line, idx, mc);

    let oxy = inner_part2(0, len, input, oxy_bit_criteria);
    let co2 = inner_part2(0, len, input, co2_bit_criteria);

    return (oxy, co2);
}

fn inner_part2(idx: usize, len: usize, input: Input, filter_func: BitCriteria) -> i32 {
    if idx > len {
        panic!("We reached the end")
    }

    if input.len() == 1 {
        return binary_to_base10(input[0].clone());
    }

    let most_common = most_common_in_column_as_char(idx, input);
    let next_input = prune_input(input, idx, most_common, filter_func);

    return inner_part2(idx + 1, len, &next_input, filter_func);
}

fn prune_input(
    input: Input,
    idx: usize,
    most_common: char,
    filter_func: BitCriteria,
) -> Vec<String> {
    return input
        .into_iter()
        .filter(|line| filter_func(line, idx, most_common))
        .cloned()
        .collect_vec();
}

fn starts_with_most_common(line: &String, idx: usize, most_common: char) -> bool {
    let c = line.chars().nth(idx).unwrap();
    let ans = c == most_common;
    return ans;
}

fn most_common_in_column_as_char(column: usize, input: Input) -> char {
    return most_common_in_column(column, input)
        .to_string()
        .chars()
        .nth(0)
        .unwrap();
}
fn most_common_in_column(column: usize, input: Input) -> i32 {
    let mut zeros = 0;
    let mut ones = 0;
    for line in input.clone() {
        match line.chars().nth(column).unwrap() {
            '0' => zeros += 1,
            '1' => ones += 1,
            _ => panic!("Dis ain't binary"),
        }
    }
    if zeros > ones {
        return 0;
    } else {
        return 1;
    }
}

fn binary_to_base10(numstr: String) -> i32 {
    return isize::from_str_radix(numstr.as_str(), 2).unwrap() as i32;
}

fn read_file(path: &str) -> Vec<String> {
    return fstream::read_lines(path).unwrap();
}

// fn parse_input(input: Input) -> Vec<Com> {
//     input.iter().map(parse_line).collect_vec()
// }
