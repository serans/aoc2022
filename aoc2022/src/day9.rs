use std::collections::HashSet;

type Point = (i32, i32);
/*
fn len(p: &Point) -> f64 {
    f64::from(p.0.abs().pow(2) + p.1.abs().pow(2)).sqrt()
}
fn distance(p1: &Point, p2:&Point) -> f64 {
    len(&diff(p1,p2))
}
fn diff(p1: &Point, p2: &Point) -> Point {
    (p1.0 - p2.0, p1.1 - p2.1)
}
fn add(p1: &Point, p2:&Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}*/
fn touch(p1: &Point, p2:&Point) -> bool {
    (p1.0-p2.0).abs()<=1 && (p1.1-p2.1).abs() <=1
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let mut head = (0,0);
    let mut tail = (0,0);
    let mut tail_positions:HashSet<Point> = HashSet::new();
    tail_positions.insert(tail);

    for command in lines {
        let command = command.split(" ").collect::<Vec<&str>>();
        let distance = command[1].parse::<i32>().unwrap();
        let head_dest = match command[0] {
            "L" => ( head.0-distance, head.1 ),
            "R" => ( head.0+distance, head.1 ),
            "U" => ( head.0         , head.1+distance),
            "D" => ( head.0         , head.1-distance),
            _ => panic!("unknown move {}", command[0])
        };

        while head != head_dest {
            //head moves are always in one dimension only
            if head.0 > head_dest.0 {
                head = (head.0-1, head.1)
            }
            if head.0 < head_dest.0 {
                head = (head.0+1, head.1)
            }
            if head.1 > head_dest.1 {
                head = (head.0, head.1-1)
            }
            if head.1 < head_dest.1 {
                head = (head.0, head.1+1)
            }
//            println!("{:?} {:?}", head, head_dest);
            if !touch(&head, &tail) {
                // move tail towards head
                if head.0 > tail.0 { tail = (tail.0+1, tail.1);}
                if head.0 < tail.0 { tail = (tail.0-1, tail.1);}
                if head.1 > tail.1 { tail = (tail.0,   tail.1+1);}
                if head.1 < tail.1 { tail = (tail.0,   tail.1-1);}
            }
//            println!("\t{:?}", tail);
            tail_positions.insert(tail);

        }

    }

    println!("The tail visited {} positions", tail_positions.len());
}
