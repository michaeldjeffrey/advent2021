use std::fs;

use itertools::Itertools;

const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0), // up
    (1, 0),  // down
    (0, -1), // left
    (0, 1),  // right
];

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let input: Vec<Vec<i32>> = file_input
        .lines()
        .map(|x| {
            x.chars()
                .map(|n| n.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec();

    let height = input.len() as i32;
    let width = input[0].len() as i32;
    let height2 = height * 5;
    let width2 = width * 5;

    let start = Pos(0, 0);
    let goal1 = Pos(width - 1, height - 1);
    let goal2 = Pos(width2 - 1, height2 - 1);

    let part1 = pathfinding::dijkstra(
        &start,
        |p| p.neighbors(&input, width, height),
        |p| *p == goal1,
    );
    let part2 = pathfinding::dijkstra(
        &start,
        |p| p.neighbors2(&input, width, height),
        |p| *p == goal2,
    );

    println!("Part 1 : {:#?}", part1.unwrap().1);
    println!("Part 2 : {:#?}", part2.unwrap().1);
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, PartialOrd, Ord)]
struct Pos(i32, i32);

impl Pos {
    fn neighbors(&self, map: &Vec<Vec<i32>>, width: i32, height: i32) -> Vec<(Pos, i32)> {
        let &Pos(x, y) = self;
        return DIRECTIONS
            .iter()
            .filter_map(move |(dy, dx)| {
                let ny = y + dy;
                let nx = x + dx;
                if nx < 0 || nx >= width {
                    return None;
                }
                if ny < 0 || ny >= height {
                    return None;
                }
                let pos = Pos(nx, ny);
                let val = map[ny as usize][nx as usize];
                return Some((pos, val));
            })
            .collect_vec();
    }
    fn neighbors2(&self, map: &Vec<Vec<i32>>, width: i32, height: i32) -> Vec<(Pos, i32)> {
        let &Pos(x, y) = self;
        return DIRECTIONS
            .iter()
            .filter_map(move |(dy, dx)| {
                let ny = y + dy;
                let nx = x + dx;
                if nx < 0 || nx >= (width * 5) {
                    return None;
                }
                if ny < 0 || ny >= (height * 5) {
                    return None;
                }
                let pos = Pos(nx, ny);
                let val_adder = (ny / height) + (nx / width);
                let val = map[(ny % height) as usize][(nx % width) as usize];

                let mut final_val = val + val_adder;
                while final_val > 9 {
                    final_val -= 9
                }
                return Some((pos, final_val));
            })
            .collect_vec();
    }
}
