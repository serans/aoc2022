struct Pipeline {
    sum: i32,
    delay: u32,
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    const WIDTH: i32 = 40;
    const HEIGHT: i32 = 6;

    let mut regx = 1;
    let mut power = 0; // problem1 output
    let mut instructions = lines.into_iter();

    let mut pipeline: Option<Pipeline> = None;
    for cycle in 0..(WIDTH * HEIGHT) {
        if let Some(mut op) = pipeline {
            if op.delay == 0 {
                regx += op.sum;
                pipeline = None;
            } else {
                op.delay -=1;
                pipeline = Some(op);
            }
        }

        if let None = pipeline {
            let new_op = instructions.next().unwrap();
            if new_op != "noop" {
                pipeline = Some(Pipeline{
                    sum: new_op.split(" ").nth(1).unwrap().parse::<i32>().unwrap(),
                    delay:1
                });
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
