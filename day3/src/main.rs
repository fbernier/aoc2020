use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_lines() -> Vec<Vec<u8>> {
    let file = File::open("input").unwrap();
    let lines: Vec<Vec<u8>> = BufReader::new(&file)
        .lines()
        .map(|s| s.unwrap().as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    lines
}

fn count_trees(grid: &Vec<Vec<u8>>, (h, v): &(usize, usize)) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut trees = 0;
    loop {
        x = x + h;
        y = y + v;
        let dx = x % grid[0].len();
        if grid[y][dx] == '#' as u8 {
            trees += 1;
        }

        if y == grid.len() - 1 {
            break;
        }
    }

    trees
}

fn part1(grid: &Vec<Vec<u8>>, tup: &(usize, usize)) -> usize {
    count_trees(grid, tup)
}

fn part2(grid: &Vec<Vec<u8>>, input: &Vec<(usize, usize)>) -> usize {
    input
        .iter()
        .fold(1, |acc, tup| acc * count_trees(&grid, tup))
}

fn main() {
    let grid = load_lines();

    let part2_input = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!(
        "part1: {:?}, part2: {:?}",
        part1(&grid, &(3, 1)),
        part2(&grid, &part2_input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let grid = load_lines();
        assert_eq!(part1(&grid, &(3, 1)), 181)
    }

    #[test]
    fn part2_test() {
        let grid = load_lines();
        let part2_input = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        assert_eq!(part2(&grid, &part2_input), 1260601650)
    }
}
