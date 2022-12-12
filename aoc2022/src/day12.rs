use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

type Map = Vec<Vec<u8>>;
type Position = (u32, u32);

struct Problem {
    start: Position,
    goal: Position,
    map: Map,
}

struct SearchNode {
    pos: Position,
    cost: u32,
    fscore: u32,
}
impl fmt::Debug for SearchNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x,y) = self.pos;
        write!(f, "({},{}, |{}|)",x, y, self.fscore)
    }
}

fn read_problem(lines: impl Iterator<Item = String>) -> Problem {
    let mut map:Map = Vec::new();
    let mut goal:Option<Position> = None;
    let mut start:Option<Position> = None;

    let mut y:u32 = 0;
    for row in lines {
        map.push(Vec::new());
        let mut x = 0;
        for cell in row.bytes() {
            let cell = match cell as char {
                'S' => { 
                    start = Some((y,x));
                    'a' 
                },
                'E' => { 
                    goal = Some((y,x));
                    'z' 
                },
                other => {
                    other
                }
            };
            map[y as usize].push(cell as u8);
            x+=1;
        }
        y+=1;
    }

    if let (Some(goal), Some(start)) = (goal, start) {
        Problem{start, goal, map}
    } else {
        panic!("Wrong problem input");
    }

}

fn manhattan(p1: Position, p2: Position) -> u32 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn displace(origin: &SearchNode, y_offset: i32, x_offset: i32, map: &Map , history: &HashMap<Position,u32>) -> Option<SearchNode> {
    // check map boundaries
    if y_offset < 0 && origin.pos.0 == 0 {
        return None
    }
    if x_offset < 0 && origin.pos.1 == 0 {
        return None
    }

    let height = map.len() as u32;
    let new_y = (origin.pos.0 as i32 + y_offset) as u32;
    if new_y >= height {
        return None
    }
    let width = map[0].len() as u32;
    let new_x = (origin.pos.1 as i32 + x_offset) as u32;
    if new_x >= width {
        return None
    }

    let elevation_origin = map[origin.pos.0 as usize][origin.pos.1 as usize];
    let elevation_dest = map[new_y as usize][new_x as usize];
    if elevation_dest > elevation_origin + 1 {
        return None
    }

    let new_position = (new_y, new_x);
    let new_cost = origin.cost + 1;
    if let Some(cost) = history.get(&new_position) {
        if *cost <= new_cost {
            return None
        }
    }
    Some(SearchNode{
        pos: new_position,
        cost: new_cost,
        fscore: new_cost + manhattan(origin.pos, new_position),
    })
}

fn a_star(problem: &Problem) -> Option<u32> {
    let init_node = SearchNode{
        pos:problem.start,
        cost:0,
        fscore:manhattan(problem.start, problem.goal),
    };

    let mut history:HashMap<Position, u32> = HashMap::new(); //from([(problem.start,0)]);
    let mut expanded:HashSet<(Position, u32)> = HashSet::new();

//    let mut frontier: Vec<SearchNode> = vec![init_node];
    let mut frontier: VecDeque<SearchNode> = VecDeque::new();
    frontier.push_back(init_node);

const LIMIT:u32 = 1000000;
let mut iter=0;
    while !frontier.is_empty() {
        iter+=1;
        if iter == LIMIT {
            panic!("no solution");
        }
        // expand nodes
//println!("{:?}", frontier);
        let next = frontier.pop_front().unwrap();
//println!("expabdubg {:?}\n", next);
        history.insert(next.pos, next.cost);
        // solution found
        if next.pos == problem.goal {
            println!("solution found after {} iterations", iter);
            return Some(next.cost)
        } else {
            // try moves
            if let Some(node) = displace(&next, 0,  1, &problem.map, &history) {
                //frontier.push(node);
                let pos = node.pos;
                let cost = node.cost;
                if ! expanded.contains(&(node.pos, node.cost)) {
                match frontier.binary_search_by(|probe| probe.fscore.cmp(&node.fscore)) {
                    Ok(pos) => { frontier.insert(pos, node)}
                    Err(pos) => { frontier.insert(pos, node)}
                }
                    expanded.insert((pos, cost));
                }
            }
            if let Some(node) = displace(&next, 0, -1, &problem.map, &history) {
                let pos = node.pos;
                let cost = node.cost;
                if ! expanded.contains(&(node.pos, node.cost)) {
                match frontier.binary_search_by(|probe| probe.fscore.cmp(&node.fscore)) {
                    Ok(pos) => { frontier.insert(pos, node)}
                    Err(pos) => { frontier.insert(pos, node)}
                }
                    expanded.insert((pos, cost));
                }
            }
            if let Some(node) = displace(&next, 1,  0, &problem.map, &history) {
                let pos = node.pos;
                let cost = node.cost;
                if ! expanded.contains(&(node.pos, node.cost)) {
                match frontier.binary_search_by(|probe| probe.fscore.cmp(&node.fscore)) {
                    Ok(pos) => { frontier.insert(pos, node)}
                    Err(pos) => { frontier.insert(pos, node)}
                }
                    expanded.insert((pos, cost));
                }
            }
            if let Some(node) = displace(&next,-1,  0, &problem.map, &history) {
                let pos = node.pos;
                let cost = node.cost;
                if ! expanded.contains(&(node.pos, node.cost)) {
                match frontier.binary_search_by(|probe| probe.fscore.cmp(&node.fscore)) {
                    Ok(pos) => { frontier.insert(pos, node)}
                    Err(pos) => { frontier.insert(pos, node)}
                }
                    expanded.insert((pos, cost));
                }
            }
        }
    }
    None
}

fn print_problem(p: &Problem, current_pos: &Position) {
    for (y,row) in p.map.iter().enumerate() {
        for (x,cell) in row.iter().enumerate() {
            let cell = *cell as char;
            if (y as u32,x as u32) == *current_pos {
                print!("\x1b[95m{}\x1b[0m",cell);
            } else if (y as u32,x as u32) == p.goal {
                print!("\x1b[91m{}\x1b[0m",cell);
            } else {
                print!("{}",cell);
            }
        }
        println!("");
    }
    println!("");
}

pub fn solve(lines: impl Iterator<Item = String>) {
    let problem = read_problem(lines);
    print_problem(&problem, &problem.start);
    let solution = a_star(&problem);
    println!("SOLUTION 1: {}", solution.unwrap());
}
