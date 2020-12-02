extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Line<'a> {
    lower: usize,
    higher: usize,
    chr: char,
    text: &'a str,
}

impl<'a> Line<'a> {
    pub fn new(vec_line: &'a Vec<&str>) -> Line<'a> {
        Line {
            lower: vec_line[0].parse().unwrap(),
            higher: vec_line[1].parse().unwrap(),
            chr: vec_line[2].chars().nth(0).unwrap(),
            text: vec_line[3],
        }
    }
}

fn is_valid_p1(line: &Line) -> bool {
    let count = line.text.matches(line.chr).count();

    if count >= line.lower && count <= line.higher {
        return true;
    }
    return false;
}

fn is_valid_p2(line: &Line) -> bool {
    let pos1 = line.text.chars().nth(line.lower - 1).unwrap();
    let pos2 = line.text.chars().nth(line.higher - 1).unwrap();

    if (pos1 == line.chr || pos2 == line.chr) && pos1 != pos2 {
        return true;
    }
    return false;
}

fn day2() -> (u32, u32) {
    let regex = Regex::new(r#"(\d+)-(\d+) (.): (.*)$"#).unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    let file = File::open("input").unwrap();

    for line in BufReader::new(&file).lines() {
        let l = line.unwrap();

        let captures = regex
            .captures(&l)
            .map(|captures| {
                captures
                    .iter()
                    .skip(1) // Skipping the complete match
                    .flat_map(|c| c) // Ignoring all empty optional matches
                    .map(|c| c.as_str()) // Grab the original strings
                    .collect::<Vec<_>>()
            })
            .unwrap();

        let l = Line::new(&captures);

        if is_valid_p1(&l) {
            part1 += 1
        };
        if is_valid_p2(&l) {
            part2 += 1
        };
    }

    (part1, part2)
}

fn main() {
    let (part1, part2) = day2();

    println!("{:?}", part1);
    println!("{:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_test() {
        assert_eq!(day2(), (536, 558));
    }
}
