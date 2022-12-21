/*
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II

    II ------ <JJ:21>
     |
    AA ------ <DD:20> - <EE:3> - FF - GG - <HH:22> 
     |           |
    <BB:13> - <CC:2>
*/

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Valve {
    name: String,
    psi: u32,
    distance: HashMap<String, u32>,
}

impl Valve {
    fn from(text: &String) -> Valve {
        let re = Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
        let cap = re.captures(text.as_str()).unwrap();
        let mut valve = Valve{
            name: String::from(&cap[1]),
            psi: cap[2].parse::<u32>().unwrap(),
            distance: HashMap::new(),
        };
        for c in cap[3].split(",") {
            valve.distance.insert(String::from(c.trim()), 1);
        }
        valve
    }
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let mut valves:HashMap<String, Valve> = HashMap::new();
    for row in lines {
        let v = Valve::from(&row);
        valves.insert(v.name.clone(), v);
    }

    let mut valve_names:Vec<String> = valves.keys().cloned().collect();
    valve_names.sort();

    // update distances
    for name in valve_names.clone() {
        // frontier: list of nodes to be explored
        let mut frontier: Vec<(String, u32)> = valves[&name].distance.clone().into_iter().collect();
        // visited: list of nodes already visited (to avoid cycles)
        let mut visited: HashSet<String> = HashSet::from([name.clone()]);
        while !frontier.is_empty() {
            // get the first node in the list (at the beginning, one of the immediate neighbors)
            let (neighbor, dist) = frontier.pop().unwrap();
            if visited.contains(&neighbor) { continue }
            visited.insert(neighbor.clone());
            valves.get_mut(&name).unwrap().distance.insert(neighbor.clone(), dist);
            for (n,d) in valves[&neighbor].distance.clone() {
                frontier.insert(0, (n,dist+d)); // d is always 1, but whatever
            }
        }
    }

    // now we have a list of nodes, each with distances to one another
    // let's try a greedy algorithm
    let mut node = String::from("AA");
    let mut open:HashSet<String> = HashSet::new();
    let mut pressure = 0;
    let mut valves_psi = 0;
    let mut t:i64 = 0;
    while t<30 {
        println!("Node: {node} Time: {t} // released {pressure} | {valves_psi}");
        let x = valves.get(&node).unwrap().clone().distance.iter().clone()
            // ignore valves that are already open, or that have no pressure
            .filter(|(v,_)| !open.contains(*v) && valves.get(*v).unwrap().psi >0 )
            .reduce(
                // find the top value node
                |(v1, d1), (v2, d2)| {
                    let dist1:i64 = *d1 as i64;
                    let dist2:i64 = *d2 as i64;
                    let value1:i64 = (31-t-dist1)*valves[v1].psi as i64;
                    let value2:i64 = (31-t-dist2)*valves[v2].psi as i64;
                    if value1 > value2 { (v1, d1) } else { (v2, d2) }
                }
            );
        
        match x {
            None => {
                println!(" - No new valve to open");
                pressure += valves_psi;
                t+=1;
            }
            Some(y) => {
                node = y.0.clone();
                let distance = y.1;
                let new_pressure = valves[&node].psi;
                println!(" - opening node: {} (+{} psi)", node, new_pressure);
                t+=*distance as i64 + 1; // +1 minute to open the valve
                pressure += valves_psi * (distance +1); // open valves release pressure
                valves_psi += valves[&node].psi; // we add the new valve
                open.insert(node.clone()); // we mark as open
                println!("      {} minute(s) to reach the valve", distance);
                println!("       1 minute to open the valve");
                println!("-----------");
                println!("TIME: {t}")
                // 1596 is too low
            }
        }
    }
    println!("Final pressure {pressure}");

}
