const NUM_STACKS: usize = 9;
const STACK_WIDTH: usize = 4;

type CratesLayout = [Vec<char>; NUM_STACKS];

fn parse_crates_layout(lines: impl Iterator<Item = String>) -> (CratesLayout, CratesLayout) {
    let mut stack1: CratesLayout = Default::default();
    let mut stack2: CratesLayout = Default::default();

    for line in lines {
        if line.is_empty() {
            break;
        }
        if !line.starts_with("[") {
            continue;
        }
        for stack_number in 0..NUM_STACKS {
            let c = line.chars().nth(1 + (stack_number * STACK_WIDTH)).unwrap();
            if c != ' ' {
                stack1[stack_number].insert(0, c);
                stack2[stack_number].insert(0, c);
            }
        }
    }
    (stack1, stack2)
}

fn stack_top(stacks: &CratesLayout) -> String {
    let mut output = String::new();
    for s in stacks {
        output.push(*s.last().unwrap());
    }
    output
}

#[allow(dead_code)]
pub fn solve(mut lines: impl Iterator<Item = String>) {
    let (mut layout_problem1, mut layout_problem2) = parse_crates_layout(lines.by_ref());

    for line in lines.by_ref() {
        let line = line.split(" ").collect::<Vec<&str>>();
        let num_moves = line[1].parse::<usize>().unwrap();
        let origin = line[3].parse::<usize>().unwrap() - 1;
        let dest = line[5].parse::<usize>().unwrap() - 1;
        let dest_size = layout_problem2[dest].len();
        for _ in 0..num_moves {
            let c = layout_problem1[origin].pop().unwrap();
            layout_problem1[dest].push(c);
            let c = layout_problem2[origin].pop().unwrap();
            layout_problem2[dest].insert(dest_size, c);
        }
    }

    println!(
        "Top of stack for 1st problem: {}",
        stack_top(&layout_problem1)
    );
    println!(
        "Top of stack for 2nd problem: {}",
        stack_top(&layout_problem2)
    );
}
