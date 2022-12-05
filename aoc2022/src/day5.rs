fn stack_top(stacks: &[Vec<char>; NUM_STACKS]) -> String {
    let mut output = String::new();
    for s in stacks {
        output.push(*s.last().unwrap());
    }
    output
}


const NUM_STACKS:usize = 9;
const STACK_WIDTH:usize = 4;

#[allow(dead_code)]
pub fn solve(mut lines: impl Iterator<Item=String>) {
    let mut stacks_silver: [Vec<char>; NUM_STACKS] = Default::default();
    let mut stacks_gold: [Vec<char>; NUM_STACKS] = Default::default();

    // PARSE INITIAL STATE
    for line in lines.by_ref() {

        if line.is_empty() {
            break;
        }
        if ! line.starts_with("[") {
            continue;
        }
        for stack_number in 0..NUM_STACKS {
            let c = line.chars().nth(1 + (stack_number * STACK_WIDTH)).unwrap();
            if c != ' ' {
                stacks_silver[stack_number].insert(0,c);
                stacks_gold[stack_number].insert(0,c);
            }
        }
    }

    // PARSE MOVES
    for line in lines.by_ref() {
        let line = line.split(" ").collect::<Vec<&str>>();
        if line.len() != 6 {
            panic!("wrong format");
        }
        let num_moves = line[1].parse::<usize>().unwrap();
        let origin = line[3].parse::<usize>().unwrap()-1;
        let dest = line[5].parse::<usize>().unwrap()-1;
        let dest_size = stacks_gold[dest].len();
        for _ in 0..num_moves {
            let c = stacks_silver[origin].pop().unwrap();
            stacks_silver[dest].push(c);
            let c = stacks_gold[origin].pop().unwrap();
            stacks_gold[dest].insert(dest_size, c);
        }
    }

    println!("Top of stack for 1st problem: {}",stack_top(&stacks_silver));
    println!("Top of stack for 2nd problem: {}",stack_top(&stacks_gold));
}
