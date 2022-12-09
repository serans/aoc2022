use std::collections::HashSet;

fn touches(p1: &(i32, i32), p2:&(i32, i32)) -> bool {
    (p1.0-p2.0).abs()<=1 && (p1.1-p2.1).abs() <=1
}

fn move_towards(origin: &(i32, i32), dest: &(i32, i32)) -> (i32, i32) {
    let mut origin = origin.clone();
    if origin.0 > dest.0 {origin = (origin.0-1, origin.1)}
    if origin.0 < dest.0 {origin = (origin.0+1, origin.1)}
    if origin.1 > dest.1 {origin = (origin.0,   origin.1-1)}
    if origin.1 < dest.1 {origin = (origin.0,   origin.1+1)}
    origin
}

pub fn solve_for_n(n: usize, lines: Vec<String>) -> usize {
    let mut rope:Vec<(i32, i32)> = vec![(0,0); n];
    let mut tail_positions:HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert(*rope.last().unwrap());

    for command in lines {
        let command = command.split(" ").collect::<Vec<&str>>();
        let distance = command[1].parse::<i32>().unwrap();

        let mut head = *rope.first().unwrap();
        let head_dest = match command[0] {
            "L" => ( head.0-distance, head.1 ),
            "R" => ( head.0+distance, head.1 ),
            "U" => ( head.0         , head.1+distance),
            "D" => ( head.0         , head.1-distance),
            _ => panic!("unknown move {}", command[0])
        };

        while head != head_dest {
            head = move_towards(&head, &head_dest);
            rope[0] = head;

            for i in 0..rope.len()-1 {
                let front = rope[i];
                let back = rope[i+1];
                if !touches(&front, &back) {
                    rope[i+1] = move_towards(&back, &front);
                }
            }
            tail_positions.insert(*rope.last().unwrap());
        }
    }
    tail_positions.len()
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let lines: Vec<String> = lines.collect();
    println!("Solution for rope of length 2: {}", solve_for_n(2, lines.clone()));
    println!("Solution for rope of length 10: {}", solve_for_n(10, lines.clone()));
}

