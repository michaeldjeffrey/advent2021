extern crate fstream;

fn main() {
    let lines = fstream::read_lines("./src/input.txt").unwrap();
    let numbers = lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut count1 = 0;
    for (pos, curr) in numbers.iter().enumerate() {
        // skip the 0th pos
        if pos == 0 {
            continue;
        }
        let prev = numbers[pos - 1];
        if curr > &prev {
            count1 += 1
        }
    }

    let mut count2 = 0;
    for (pos, el) in numbers.iter().enumerate() {
        // skip first 2, start sliding window to end
        if pos < 3 {
            continue;
        }
        let prev1 = numbers[pos - 1];
        let prev2 = numbers[pos - 2];
        let prev3 = numbers[pos - 3];
        let prev_sum = prev3 + prev2 + prev1;
        let curr_sum = prev2 + prev1 + el;

        if curr_sum > prev_sum {
            count2 += 1
        }
    }

    println!("part 1: {:?}", count1);
    println!("part 2: {:?}", count2);
    println!("part 2 (wrong, too low): {:?}", 1610);
}
