use std::fs::File;
use std::io::{BufRead, BufReader};

const ADDED_RESULT: u32 = 2020;

fn day1(lines: Vec<u32>) -> (u32, u32) {
    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    for x in 0..lines.len() {
        for y in 0..lines.len() {
            if lines[x] + lines[y] == ADDED_RESULT {
                part1 = lines[x] * lines[y];
            }
            for z in 0..lines.len() {
                if lines[x] == lines[y] || lines[y] == lines[z] || lines[x] == lines[z] {
                    continue;
                };
                if lines[x] + lines[y] + lines[z] == ADDED_RESULT {
                    part2 = lines[x] * lines[y] * lines[z];
                    break;
                }
            }
        }
    }

    (part1, part2)
}

fn load_lines() -> Vec<u32> {
    let file = File::open("input").unwrap();
    let lines: Vec<u32> = BufReader::new(&file)
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect::<Vec<u32>>();

    lines
}

fn main() {
    let lines = load_lines();
    let (part1, part2) = day1(lines);
    println!("part1: {:?}, part2: {:?}", part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_test() {
        assert_eq!(day1(load_lines()), (1010884, 253928438));
    }
}
