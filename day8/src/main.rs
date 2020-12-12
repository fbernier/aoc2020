const INPUT: &str = include_str!("../input");

#[derive(Debug)]
struct VM<'a> {
    acc: isize,
    pc: isize,
    prevs: Vec<isize>,
    prog: &'a Vec<Instruction<'a>>,
}

#[derive(Debug)]
struct Instruction<'a> {
    op: &'a str,
    arg: isize,
}

static mut a: u32 = 0;

impl<'a> VM<'a> {
    fn step(&mut self) {
        match self.prog[self.pc as usize].op {
            "nop" => (),
            "acc" => self.acc += self.prog[self.pc as usize].arg,
            "jmp" => {
                self.pc += self.prog[self.pc as usize].arg;
                return
            }
            _ => panic!("derp"),
        }

        self.pc += 1;
    }

    fn build_and_run_till(prog: &'a Vec<Instruction<'a>>) -> VM<'a> {
        let mut vm = VM {
            acc: 0,
            pc: 0,
            prevs: vec![0; prog.len()],
            prog,
        };

        loop {
            if (vm.pc as usize) >= vm.prog.len() || vm.prevs[vm.pc as usize] != 0 {
                return vm
            }
            vm.prevs[vm.pc as usize] += 1;
            vm.step();
        }
    }
}

fn main() {
    let mut lines: Vec<Instruction> = INPUT
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| {
            let v: Vec<&str> = l.split(" ").collect();
            Instruction {
                op: v[0],
                arg: v[1].parse().unwrap(),
            }
        })
        .collect();
    let len = lines.len();

    let vm = VM::build_and_run_till(&lines);
    println!("part1: {:?}", vm.acc);

    let mut part2 = 0;
    for i in 0..lines.len() {
        match lines[i].op {
            "nop" => {
                lines[i].op = "jmp";
                let vm = VM::build_and_run_till(&lines);
                if len == vm.pc as usize { part2 = vm.acc; break }
                lines[i].op = "nop";
            },
            "jmp" => {
                lines[i].op = "nop";
                let vm = VM::build_and_run_till(&lines);
                if len == vm.pc as usize { part2 = vm.acc; break }
                lines[i].op = "jmp";
            },
            _ => ()
        }
    }
    println!("part1: {:?}", part2);
}
