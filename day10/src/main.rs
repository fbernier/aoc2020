const INPUT: &str = include_str!("../input");

fn main() {
    let mut adapters: Vec<u64> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
    let dev_jolt = *adapters.iter().max().unwrap() + 3;
    adapters.push(0);
    adapters.push(dev_jolt);
    adapters.sort_unstable();
    dbg!(&adapters.len());

    let mut diff: Vec<u64> = vec![0; 4];

    let mut prev = 0;
    for &a in &adapters {
        let d = a - prev;
        diff[d as usize] += 1;
        prev = a;
    }

    println!(
        "part1: {:?}",
        diff.iter().filter(|&d| *d != 0).product::<u64>()
    );

    let mut perms: Vec<u64> = vec![0; adapters.len() - 2];
    perms.insert(0, 1);

    for i in 0..adapters.len() - 1 {
        for j in 1..=3 {
            if i >= j && i < perms.len() && adapters[i] - adapters[i - j] <= 3 {
                perms[i] += perms[i - j];
            }
        }
    }

    println!("part 2: {:?}", perms.last().unwrap())
}
