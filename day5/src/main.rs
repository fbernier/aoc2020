const INPUT: &str = include_str!("../input");

fn main() {
    let mut part1 = 0;
    let mut ids: Vec<bool> = vec![false; 1024];

    for l in INPUT.lines() {
        let bin: String = l
            .chars()
            .map(|c| match c {
                'F' | 'L' => '0',
                'B' | 'R' => '1',
                _ => panic!("derp"),
            })
            .collect();

        let row = usize::from_str_radix(&bin[..7], 2).unwrap();
        let col = usize::from_str_radix(&bin[7..], 2).unwrap();

        ids[(row * 8) + col] = true;
        let seat_id = (row * 8) + col;

        if seat_id > part1 {
            part1 = seat_id;
        }
    }

    let mut part2 = 0;
    for i in 1..ids.len() {
        if !ids[i] && ids[i - 1] {
            part2 = i;
            break;
        }
    }

    println!("part1: {:?}, part2: {:?}", part1, part2);
}
