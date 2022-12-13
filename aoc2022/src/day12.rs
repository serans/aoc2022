use std::collections::{HashMap, HashSet, VecDeque};

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

struct Search {
    problem: Problem,
    history: HashMap<Position, u32>,
    expanded: HashSet<(Position, u32)>,
    frontier: VecDeque<SearchNode>,
}

impl Search {
    fn solve(problem: Problem) -> Option<u32> {
        Search {
            problem,
            history: HashMap::new(),
            expanded: HashSet::new(),
            frontier: VecDeque::new(),
        }
        .a_star_search()
    }

    fn a_star_search(&mut self) -> Option<u32> {
        self.frontier.push_back(SearchNode {
            pos: self.problem.start,
            cost: 0,
            fscore: manhattan_distance(self.problem.start, self.problem.goal),
        });

        const LIMIT: u32 = 1000000;
        let mut iter = 0;
        while !self.frontier.is_empty() {
            iter += 1;
            if iter == LIMIT {
                panic!("no solution found in {} iterations", iter);
            }
            let next = self.frontier.pop_front().unwrap();
            self.history.insert(next.pos, next.cost);

            if next.pos == self.problem.goal {
                return Some(next.cost);
            } else {
                // try moves
                self.try_move(&next, 0, 1);
                self.try_move(&next, 0, -1);
                self.try_move(&next, 1, 0);
                self.try_move(&next, -1, 0);
            }
        }
        None
    }

    fn try_move(&mut self, origin: &SearchNode, y_offset: i32, x_offset: i32) {
        if let Some(node) = self.check_move(origin, y_offset, x_offset) {
            let pos = node.pos;
            let cost = node.cost;
            if !self.expanded.contains(&(node.pos, node.cost)) {
                match self
                    .frontier
                    .binary_search_by(|probe| probe.fscore.cmp(&node.fscore))
                {
                    // @TODO: no easier way to insert keeping vector sorted?
                    Ok(pos) => self.frontier.insert(pos, node),
                    Err(pos) => self.frontier.insert(pos, node),
                }
                self.expanded.insert((pos, cost));
            }
        }
    }

    fn check_move(&self, origin: &SearchNode, y_offset: i32, x_offset: i32) -> Option<SearchNode> {
        // check map boundaries
        if y_offset < 0 && origin.pos.0 == 0 {
            return None;
        }
        if x_offset < 0 && origin.pos.1 == 0 {
            return None;
        }

        let height = self.problem.map.len() as u32;
        let new_y = (origin.pos.0 as i32 + y_offset) as u32;
        if new_y >= height {
            return None;
        }
        let width = self.problem.map[0].len() as u32;
        let new_x = (origin.pos.1 as i32 + x_offset) as u32;
        if new_x >= width {
            return None;
        }

        // check elevation
        let elevation_origin = self.problem.map[origin.pos.0 as usize][origin.pos.1 as usize];
        let elevation_dest = self.problem.map[new_y as usize][new_x as usize];
        if elevation_dest > elevation_origin + 1 {
            return None;
        }

        // check that position is not already explored
        let new_position = (new_y, new_x);
        let new_cost = origin.cost + 1;
        if let Some(cost) = self.history.get(&new_position) {
            if *cost <= new_cost {
                return None;
            }
        }

        Some(SearchNode {
            pos: new_position,
            cost: new_cost,
            fscore: new_cost + manhattan_distance(origin.pos, new_position),
        })
    }
}
fn manhattan_distance(p1: Position, p2: Position) -> u32 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn read_problem(lines: impl Iterator<Item = String>) -> Problem {
    let mut map: Map = Vec::new();
    let mut goal: Option<Position> = None;
    let mut start: Option<Position> = None;

    let mut y: u32 = 0;
    for row in lines {
        map.push(Vec::new());
        let mut x = 0;
        for cell in row.bytes() {
            let cell = match cell as char {
                'S' => {
                    start = Some((y, x));
                    'a'
                }
                'E' => {
                    goal = Some((y, x));
                    'z'
                }
                other => other,
            };
            map[y as usize].push(cell as u8);
            x += 1;
        }
        y += 1;
    }

    if let (Some(goal), Some(start)) = (goal, start) {
        Problem { start, goal, map }
    } else {
        panic!("Wrong problem input");
    }
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let problem = read_problem(lines);

    let problem1 = Problem {
        map: problem.map.clone(),
        goal: problem.goal,
        start: problem.start,
    };
    println!("Problem 1: {}", Search::solve(problem1).unwrap());

    let mut solution2: Option<u32> = None;
    for (y, row) in problem.map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == &('a' as u8) {
                let problem2 = Problem {
                    map: problem.map.clone(),
                    goal: problem.goal,
                    start: (y as u32, x as u32),
                };
                if let Some(partial_solution) = Search::solve(problem2) {
                    solution2 = match solution2 {
                        None => Some(partial_solution),
                        Some(x) => Some(x.min(partial_solution)),
                    }
                }
            }
        }
    }
    println!("SOLUTION 2: {}", solution2.unwrap());
}
