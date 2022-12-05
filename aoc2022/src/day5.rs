
use std::fs::File;
use std::io::{BufReader, Lines};


fn print_stacks(stacks: &[Vec<char>; NUM_STACKS]) {
    for (i,s) in stacks.iter().enumerate() {
        print!("{}",i+1);
        for c in s {
            print!(" [{}] ", c);
        }
        println!();
    }
}

fn stack_top(stacks: &[Vec<char>; NUM_STACKS]) {
    for s in stacks {
        print!("{}", s.last().unwrap());
    }
    println!();
}


const NUM_STACKS:usize = 9;
const STACK_WIDTH:usize = 4;

#[allow(dead_code)]
pub fn solve(lines: Lines<BufReader<File>>) {
    let mut stacks: [Vec<char>; NUM_STACKS] = Default::default();

    // PARSE INITIAL STATE
    let mut parsing_stack = true;

    let mut lines = lines.into_iter();
    for line in lines.by_ref() {

        let line = line.unwrap();
        if parsing_stack {
            if line.is_empty() {
                parsing_stack = false;
            }
            if ! line.starts_with("[") {
                continue;
            }
            for stack_number in 0..NUM_STACKS {
                let c = line.chars().nth(1 + (stack_number * STACK_WIDTH)).unwrap();
                if c != ' ' {
                    stacks[stack_number].insert(0,c);
                }
            }
        } else {
            // parsing move
            let line = line.split(" ").collect::<Vec<&str>>();
            if line.len() != 6 {
                panic!("wrong format");
            }
            let num_moves = line[1].parse::<usize>().unwrap();
            let origin = line[3].parse::<usize>().unwrap()-1;
            let dest = line[5].parse::<usize>().unwrap()-1;
            for _ in 0..num_moves {
                let c = stacks[origin].pop().unwrap();
                stacks[dest].push(c);
            }
        }
    }

    print_stacks(&stacks);
    stack_top(&stacks);
}
