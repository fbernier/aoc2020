use std::collections::HashSet;

const INPUT: &str = include_str!("../input");

fn main() {
    let part1 = INPUT
        .split("\n\n")
        .map(|g| {
            g.split_whitespace()
                .flat_map(|p| p.chars())
                .collect::<HashSet<char>>()
                .iter()
                .count() as u32
        })
        .sum::<u32>();

    let part2 = INPUT
        .split("\n\n")
        .map(|g| {
                g.split_whitespace()
                .map(|line| {
                    println!("{:?}", line);
                    line.bytes().fold(0, |x, b| x | 1 << (b - b'a'))
                })
                .fold(!0u32, |acc, x| acc & x)
                .count_ones()
        })
        .sum::<u32>();

    println!("part1: {:?}, part2: {:?}", part1, part2);
}
