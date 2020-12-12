use itertools::Itertools;
const INPUT: &str = include_str!("../input");

fn main() {

    let mut lines = INPUT.lines().map(|l| l.parse().unwrap()).collect::<Vec<usize>>();

    let mut part1 = 0;

    for n in 25..lines.len() {
        let possibilities: Vec<usize> = lines[n-25..n].iter().permutations(2).map(|v| v[0] + v[1]).collect();

        if !possibilities.contains(&lines[n]) {
            part1 = lines[n];
            break;
        }
    }
    println!("part1: {:?}", part1);

    'outer: for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let s: usize = (&lines[i..j]).iter().sum();
            if s == part1 {
                let a = (&lines[i..j]).iter().min().unwrap();
                let b = (&lines[i..j]).iter().max().unwrap();
                println!("part2: {:?}", a + b);
                break 'outer;
            } else if s > part1 {
                break;
            }
        }
    }
}
