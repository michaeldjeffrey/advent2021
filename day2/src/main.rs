use itertools::Itertools;
extern crate fstream;

fn main() {
    let input = read_file("./src/input.txt");
    let commands = parse_input(input);

    let mut part1_sub = Submarine1::new();
    let mut part2_sub = Submarine2::new();

    part1_sub.drive(commands.clone());
    part2_sub.drive(commands.clone());

    println!("part 1: {}", part1_sub.answer());
    println!("part 2: {}", part2_sub.answer());
}

struct Submarine1 {
    depth: i32,
    hoz: i32,
}

struct Submarine2 {
    depth: i32,
    hoz: i32,
    aim: i32,
}

fn read_file(path: &str) -> Vec<String> {
    return fstream::read_lines(path).unwrap();
}

fn parse_input(input: Vec<String>) -> Vec<Com> {
    input.iter().map(parse_line).collect_vec()
}

#[derive(Clone)]
enum Com {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse_line(line: &String) -> Com {
    let mut split = line.split(" ");
    let com = split.next().unwrap();
    let num: i32 = split.next().unwrap().parse().unwrap();
    return match com {
        "forward" => Com::Forward(num),
        "up" => Com::Up(num),
        "down" => Com::Down(num),
        _ => panic!("We don't know what to do with this input"),
    };
}

impl Submarine1 {
    fn answer(&self) -> i32 {
        return self.depth * self.hoz;
    }

    fn new() -> Self {
        Self { depth: 0, hoz: 0 }
    }

    fn handle(&mut self, command: &Com) {
        match command {
            &Com::Forward(x) => self.forward(x),
            &Com::Up(x) => self.up(x),
            &Com::Down(x) => self.down(x),
        }
    }

    fn forward(&mut self, x: i32) {
        self.hoz += x;
    }

    fn up(&mut self, x: i32) {
        self.depth -= x;
    }

    fn down(&mut self, x: i32) {
        self.depth += x;
    }

    fn drive(&mut self, commands: Vec<Com>) {
        for command in commands {
            self.handle(&command)
        }
    }
}

impl Submarine2 {
    fn answer(&self) -> i32 {
        return self.depth * self.hoz;
    }

    fn new() -> Self {
        Self {
            depth: 0,
            hoz: 0,
            aim: 0,
        }
    }

    fn handle(&mut self, command: &Com) {
        match command {
            &Com::Forward(x) => self.forward(x),
            &Com::Up(x) => self.up(x),
            &Com::Down(x) => self.down(x),
        }
    }

    fn forward(&mut self, x: i32) {
        self.depth += self.aim * x;
        self.hoz += x;
    }

    fn up(&mut self, x: i32) {
        self.aim -= x
    }

    fn down(&mut self, x: i32) {
        self.aim += x;
    }

    fn drive(&mut self, commands: Vec<Com>) {
        for command in commands {
            self.handle(&command)
        }
    }
}
