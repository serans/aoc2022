use std::collections::HashSet;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {x:i32, y:i32}

impl Point {
    fn touches(&self, other:&Point) -> bool {
        (self.x-other.x).abs()<=1 && (self.y-other.y).abs() <=1
    }

    fn move_towards(&mut self, dest: &Point) {
        if self.x > dest.x { self.x-=1; }
        if self.x < dest.x { self.x+=1; }
        if self.y > dest.y { self.y-=1; }
        if self.y < dest.y { self.y+=1; }
    }
}

pub fn solve_for_n(n: usize, lines: Vec<String>) -> usize {
    let mut rope:Vec<Point> = vec![Point{x:0, y:0}; n];
    let mut tail_positions:HashSet<Point> = HashSet::new();
    tail_positions.insert(*rope.last().unwrap());

    for command in lines {
        let command = command.split(" ").collect::<Vec<&str>>();
        let distance = command[1].parse::<i32>().unwrap();

        let mut head = *rope.first().unwrap();
        let head_dest = match command[0] {
            "L" => Point{ x:head.x-distance, y:head.y         },
            "R" => Point{ x:head.x+distance, y:head.y         },
            "U" => Point{ x:head.x         , y:head.y+distance},
            "D" => Point{ x:head.x         , y:head.y-distance},
            _ => panic!("unknown move {}", command[0])
        };

        while head != head_dest {
            head.move_towards(&head_dest);
            rope[0] = head;

            for i in 0..rope.len()-1 {
                let front = rope[i];
                let mut back = rope[i+1];
                if !back.touches(&front) {
                    back.move_towards(&front);
                    rope[i+1] = back;
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

