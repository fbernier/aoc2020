const INPUT: &str = include_str!("../input");

#[derive(Debug)]
struct VM {
    accumulator: isize,
    current_pos: isize,
    previous: Vec<isize>,
}

fn main() {
    let lines: Vec<(&str, isize)> = INPUT
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| {
            let v: Vec<&str> = l.split(" ").collect();
            (v[0], v[1].parse().unwrap())
        })
        .collect();

    let mut vm = VM {
        accumulator: 0,
        current_pos: 0,
        previous: vec![],
    };

    loop {
        let instruction = lines[vm.current_pos as usize];
        if vm.previous.contains(&vm.current_pos) {
            println!("part1: {:?}", vm.accumulator);
            break;
        }
        vm.previous.push(vm.current_pos);
        vm.current_pos += 1;

        match instruction.0 {
            "nop" => continue,
            "acc" => vm.accumulator += instruction.1,
            "jmp" => vm.current_pos += instruction.1 - 1,
            _ => panic!("derp")
        }
    }
}
