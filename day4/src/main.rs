extern crate nom;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use nom::{
    branch::alt,
    bytes::complete::{take, take_till},
    character::complete::{char, line_ending, newline, space0},
    combinator::eof,
    multi::many_till,
    sequence::tuple,
    IResult,
};

const INPUT: &str = include_str!("../input");

#[derive(Debug, PartialEq)]
pub struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

#[derive(Debug, PartialEq)]
pub struct Field<'a> {
    name: &'a str,
    value: &'a str,
}

impl<'a> Passport<'a> {
    pub fn validate(&self) -> bool {
        let derp: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .collect();
        derp.is_subset(
            &self
                .fields
                .iter()
                .map(|(k, v)| *k)
                .collect::<HashSet<&str>>(),
        )
    }

    pub fn validate2(&self) -> bool {
        if !self.validate() { return false }

        let byr: u32 = self.fields.get("byr").unwrap().parse().unwrap();
        let iyr: u32 = self.fields.get("iyr").unwrap().parse().unwrap();
        let eyr: u32 = self.fields.get("eyr").unwrap().parse().unwrap();
        let hgt: &str = self.fields.get("hgt").unwrap();
        let hcl: &str = self.fields.get("hcl").unwrap();
        let ecl: &str = self.fields.get("ecl").unwrap();
        let pid: &str = self.fields.get("pid").unwrap();
        if byr < 1920 || byr > 2002 { return false };
        if iyr < 2010 || iyr > 2020 { return false };
        if eyr < 2020 || eyr > 2030 { return false };
        let num: u32 = hgt[..hgt.len()-2].parse().unwrap();
        if hgt.ends_with("cm") {
            if num < 150 || num > 193  { return false }
        } else if hgt.ends_with("in") {
            if num < 59 || num > 76  { return false }
        } else { return false }

        if !hcl.starts_with("#") || !hcl[1..].chars().all(char::is_alphanumeric) || hcl.len() != 7 {
            return false;
        }

        if !&["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl) {
            return false
        }

        if pid.len() != 9 || !pid.chars().all(char::is_numeric) {
            return false
        }

        true
    }
}

fn field(input: &str) -> IResult<&str, Field> {
    let field_name = take(3usize);
    let colon = char(':');
    let value = take_till(|ch: char| ch == '\n' || ch == ' ');
    let (input, (field_name, _, value)) = tuple((field_name, colon, value))(input)?;
    let (input, _) = alt((char('\n'), char(' ')))(input)?;
    Ok((
        input,
        Field {
            name: field_name,
            value,
        },
    ))
}

fn passport(input: &str) -> IResult<&str, Passport> {
    let passport_end = alt(((line_ending), eof));
    let (input, (fields, _)) = many_till(field, passport_end)(input)?;
    Ok((
        input,
        Passport {
            fields: fields
                .iter()
                .map(|f| (f.name, f.value))
                .collect::<HashMap<&str, &str>>(),
        },
    ))
}

fn passports(input: &str) -> IResult<&str, Vec<Passport>> {
    let (input, (vec, _)) = many_till(passport, eof)(input)?;
    Ok((input, vec))
}

fn main() {
    let (_, passports) = passports(INPUT).unwrap();
    let part1 = passports.iter().filter(|p| p.validate()).count();
    let part2 = passports.iter().filter(|p| p.validate2()).count();
    println!("part1: {:?}, part2: {:?}", part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        //let grid = load_lines();
        //
        //assert_eq!(part1(&grid, &(3, 1)), 181)
    }

    #[test]
    fn part2_test() {
        //let grid = load_lines();
        //let part2_input = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        //assert_eq!(part2(&grid, &part2_input), 1260601650)
    }
}
