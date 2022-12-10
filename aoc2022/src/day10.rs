enum SumPipeline {
    Inactive,
    Running{sum:i32, delay:u32},
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    // VM stuff
    let mut x_reg = 1;
    let mut adder = SumPipeline::Inactive;
    let mut eof = false;
    let mut cycle = 0;

    let mut instructions = lines.into_iter();
    let mut power = 0; // problem1 output
    const WIDTH: i32 = 40; // problem2 screen width

    while !eof {
        // execute
        if let SumPipeline::Running{sum, delay} = adder {
            adder = if delay == 0 {
                x_reg += sum;
                SumPipeline::Inactive
            } else {
                SumPipeline::Running{sum, delay: delay-1}
            }
        }

        // fetch
        if let SumPipeline::Inactive = adder {
            if let Some(operation) = instructions.next() {
                let mut op = operation.split(" ");
                match (op.next(), op.next()) {
                    (Some("noop"), None) => {},
                    (Some("addx"), Some(op1)) => {
                        adder = SumPipeline::Running{
                            sum: op1.parse::<i32>().unwrap(),
                            delay: 1
                        }
                    }
                    _ => {panic!("unknown command {}", operation)}
                }
            } else {
                eof = true;
            }
        }

        // problem1
        if [20, 60, 100, 140, 180, 220].contains(&(cycle+1)){
            power += x_reg*(cycle+1);
        }

        // problem2
        let x = cycle % WIDTH;
        let point = if [x_reg-1, x_reg, x_reg+1].contains(&x) { '#' } else { ' ' };
        print!("{point}");
        if x == WIDTH-1 {
            println!("")
        }

        cycle += 1;
    }
    println!("problem1 {}", power);
}
