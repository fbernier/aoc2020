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

fn load_lines() -> Vec<String> {
    let file = File::open("input").unwrap();
    let lines: Vec<_> = BufReader::new(&file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    lines
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

fn main() {
    let regex = Regex::new(r#"(\d+)-(\d+) (.): (.*)$"#).unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in load_lines() {
        let captures = regex
            .captures(&line)
            .map(|captures| {
                captures
                    .iter() // All the captured groups
                    .skip(1) // Skipping the complete match
                    .flat_map(|c| c) // Ignoring all empty optional matches
                    .map(|c| c.as_str()) // Grab the original strings
                    .collect::<Vec<_>>() // Create a vector
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

    println!("{:?}", part1);
    println!("{:?}", part2);
}
