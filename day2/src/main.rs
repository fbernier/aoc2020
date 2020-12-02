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

    fn is_valid_p1(&self) -> bool {
        let count = self.text.matches(self.chr).count();

        if count >= self.lower && count <= self.higher {
            return true;
        }
        return false;
    }

    fn is_valid_p2(&self) -> bool {
        let pos1 = self.text.chars().nth(self.lower - 1).unwrap();
        let pos2 = self.text.chars().nth(self.higher - 1).unwrap();

        if (pos1 == self.chr || pos2 == self.chr) && pos1 != pos2 {
            return true;
        }
        return false;
    }
}

fn day2() -> (u32, u32) {
    let regex = Regex::new(r#"(\d+)-(\d+) (.): (.*)$"#).unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    let file = File::open("input").unwrap();

    for str_line in BufReader::new(&file).lines() {
        let str_line = str_line.unwrap();

        let captures = regex
            .captures(&str_line)
            .map(|captures| {
                captures
                    .iter()
                    .skip(1) // Skipping the complete match
                    .map(|c| c.unwrap().as_str()) // Grab the original strings
                    .collect::<Vec<_>>()
            })
            .unwrap();

        let line = Line::new(&captures);

        if line.is_valid_p1() {
            part1 += 1
        };
        if line.is_valid_p2() {
            part2 += 1
        };
    }

    (part1, part2)
}

fn main() {
    let (part1, part2) = day2();
    println!("part1: {:?}, part2: {:?}", part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_test() {
        assert_eq!(day2(), (536, 558));
    }
}
