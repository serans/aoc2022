use regex::Regex;

type Point = (i32, i32);

fn dist(p0: Point, p1:Point) -> i32 {
    (p0.0-p1.0).abs() + (p0.1-p1.1).abs()
}

fn can_contain_beacon(p: Point, points: &Vec<(Point, Point)>) -> bool {
    for (sensor, beacon) in points.iter() {
        if dist(p, *beacon) == 0 {
            // there's already a beacon there
            return true;
        }
        if dist(p, *sensor) <= dist(*sensor, *beacon) {
            // there's a closer beacon
            return false;
        }
    }
    return true;
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {

    let mut points:Vec<(Point, Point)> = Vec::new();
    let mut max_x = 0;
    let mut min_x = 0;

    for row in lines {
        let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        let cap = re.captures(row.as_str()).unwrap();
        let x = cap[1].parse::<i32>().unwrap();
        let y = cap[2].parse::<i32>().unwrap();
        let bx = cap[3].parse::<i32>().unwrap();
        let by = cap[4].parse::<i32>().unwrap();

        min_x = min_x.min(x - dist((x,y),(bx,by)));
        max_x = max_x.max(x + dist((x,y),(bx,by)));

        points.push(((x,y), (bx, by)));
    }

    // search covered positions in line 10
    let mut i = 0;
    let y = 2000000;
    println!("--- FROM {min_x} -> {max_x}");
    for x in min_x..max_x {
        if !can_contain_beacon((x, y), &points) {
//            println!("{},{} -> no beacon",x,y);
            i+=1;
        }
    }
    println!("problem 1 {i}");
}
