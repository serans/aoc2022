enum Pipeline {
    None,
    Sum{sum:i32, del:u32},
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    const WIDTH: i32 = 40;
    const HEIGHT: i32 = 6;

    let mut regx = 1;
    let mut power = 0; // problem1 output
    let mut instructions = lines.into_iter();

    let mut pipeline = Pipeline::None;
    for cycle in 0..(WIDTH * HEIGHT) {
        // process pipeline
        if let Pipeline::Sum{sum, del} = pipeline {
            if del == 0 {
                regx += sum;
                pipeline = Pipeline::None;
            } else {
                pipeline = Pipeline::Sum{sum, del: del-1};
            }
        }

        // fetch
        if let Pipeline::None = pipeline {
            let new_op = instructions.next().unwrap();
            if new_op != "noop" {
                pipeline = Pipeline::Sum{
                    sum: new_op.split(" ").nth(1).unwrap().parse::<i32>().unwrap(),
                    del:1
                };
            }
        }

        // problem1
        if [20, 60, 100, 140, 180, 220].contains(&(cycle+1)){
            power += regx*(cycle+1);
        }

        // problem2
        let x = cycle % WIDTH;
        let point = if [regx-1, regx, regx+1].contains(&x) { '#' } else { ' ' };
        print!("{point}");
        if x == WIDTH-1 {
            println!("")
        }
    }
    println!("problem1 {}", power);
}
