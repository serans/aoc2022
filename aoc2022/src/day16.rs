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
/*
fn update_distances(valve_name: &String, valves: &mut HashMap<String, Valve>) {
    let mut expanded:HashSet<String> = HashSet::new();
    let mut frontier:Vec<(String, u32)> = vec![(valve_name.clone(),0)];

    while !frontier.is_empty() {
        let (node_name, distance) = frontier.pop().unwrap();
        if expanded.contains(&node_name) {
            // node already visited, skip
            continue
        }

        // mark as visited
        expanded.insert(node_name.clone());

        // get the valve corresponding to the node_name
        let valve = valves.get(&node_name).unwrap();
        let valve = valve.distance.iter().clone();

        // for each "children", update the distance
        for (child, distance_to_child) in valve {
            // get child node
            let child = valves.get_mut(child).unwrap();
        }

    }
}
*/

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let mut valves:HashMap<String, Valve> = HashMap::new();
    for row in lines {
        println!("{row}");
        let v = Valve::from(&row);
        valves.insert(v.name.clone(), v);
    }

    /*
    let mut valve_names:Vec<String> = Vec::new();
    for (k,_) in valves.iter().clone() {
        valve_names.push(k.clone());
    }
    for valve_name in valve_names {
        update_distances(&valve_name, &mut valves);
    }*/

    println!("{:#?}", valves);
}
