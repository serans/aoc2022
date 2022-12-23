/* new solution:
* - Each valve represented by a number 0 to 56
* - Each one can be a bit in a i64
* - Therefore connectivity graph is just a number
* - Also, list of open valves is just a number
*/

use std::collections::HashSet;
use regex::Regex;

#[derive(Eq, Hash, PartialEq)]
struct State {
    time_left: u8,
    position: u8,
    last_position: u8,
    open_valves: u64,
    psi: u32,
}

fn calculate_psi(open_valves:u64, psi:&Vec<u8>) -> u32 {
    let mut output:u32 = 0;
    let mut open_valves = open_valves;
    let mut i=0;
    while open_valves > 0 {
        if open_valves & 0x1 == 0x1 {
            output += psi[i] as u32;
        }
        open_valves = open_valves >> 1;
        i+=1;
    }
    output
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let mut valves:Vec<String> = Vec::new();
    let mut psi:Vec<u8> = Vec::new();
    let mut connections:Vec<u64> = Vec::new();

    // read input
    let re = Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    for line in lines {
        let cap = re.captures(line.as_str()).unwrap();

        let name = String::from(&cap[1]);
        if !valves.contains(&name) {
            valves.push(name.clone());
            connections.push(0);
            psi.push(0);
        }
        let valve_index = valves.iter().position(|x| x==&name).unwrap();
        psi[valve_index] = cap[2].parse::<u8>().unwrap();

        for c in cap[3].split(",") {
            let other = String::from(c.trim());
            if !valves.contains(&other) {
                valves.push(other.clone());
                connections.push(0);
                psi.push(0);
            }
            let position = valves.iter().position(|x| x==&other).unwrap();
            connections[valve_index] |= 1 << position;
        }
    }

    let init_state = State {
        time_left: 30,
        position: valves.iter().position(|x| x=="AA").unwrap() as u8,
        last_position: valves.iter().position(|x| x=="AA").unwrap() as u8,
        open_valves: 0x0,
        psi: 0,
    };

    let mut frontier:Vec<State> = Vec::new();
    let mut expanded:HashSet<State> = HashSet::new();
    frontier.push(init_state);
   
    let mut max_psi:Option<u32> = None;
    // explore moves
    while !frontier.is_empty() {
        let next = frontier.pop().unwrap();
        if expanded.contains(&next) {
            continue;
        }

        // time is out, check out max
        if next.time_left == 0 {
            max_psi = Some(max_psi.unwrap_or(next.psi).max(next.psi));
            continue;
        }

        let psi_released = next.psi + calculate_psi(next.open_valves, &psi);

        // since moving is free, we don't consider the case of just sitting
        let mut moves = connections[next.position as usize];
        let mut i:u8 = 0;
        while moves != 0 {
            if moves & 0x1 == 1 && next.last_position != i {
                frontier.push(State{
                    time_left: next.time_left -1,
                    position: i,
                    last_position: next.position,
                    open_valves: next.open_valves,
                    psi: psi_released,
                });
            } 
            moves = moves >> 1;
            i += 1;
        }
        // 2) open valve if it has pressure and it wasn't open already
        if psi[next.position as usize]>0 && next.open_valves & 1<<next.position == 0 {
            frontier.push(State{
                time_left: next.time_left -1,
                position: next.position,
                last_position: next.position,
                open_valves: next.open_valves | 1<<next.position,
                psi: psi_released,
            });
        }
        expanded.insert(next);
    }

    println!("max psi: {}", max_psi.unwrap());
}
