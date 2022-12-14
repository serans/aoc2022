use std::cmp::Ordering;

struct Map {
    x_offset: usize,
    width: usize,
    height: usize,
    cells: Vec<Vec<char>>,
}

impl Map {
    fn new(lines: &Vec<String>) -> Map {
        let mut max_x = 0;
        let mut min_x = 1000;
        let mut max_y = 0;
        for row in lines.iter().clone() {
            for point in row.split("->") {
                let point:Vec<&str> = point.split(",").collect();
                let x = point[0].trim().parse::<usize>().unwrap();
                let y = point[1].trim().parse::<usize>().unwrap();
                max_x = max_x.max(x);
                min_x = min_x.min(x);
                max_y = max_y.max(y);
            }
        }
        let width = max_x-min_x+1;
        let height = max_y+1;
        let mut map = Map {
            x_offset: min_x,
            width, height,
            cells: Vec::new(),
        };
        for y in 0..height as usize {
            map.cells.push( Vec::new());
            for _ in 0..width as usize {
                map.cells[y].push('.');
            }
        }
        map
    }

    fn read_walls(&mut self, lines: &Vec<String>) {
        let mut last_point:Option<(usize, usize)>;
        for row in lines.iter().clone() {
            last_point = None;
            for point in row.split("->") {
                let point:Vec<&str> = point.split(",").collect();
                let x = point[0].trim().parse::<usize>().unwrap(); // - X_OFFSET as i32;
                let y = point[1].trim().parse::<usize>().unwrap();
                let point = (x,y);
                while let Some((px, py)) = last_point {
                    self.cells[py][px-self.x_offset] = '#';
                    match px.cmp(&x) {
                        Ordering::Less => { last_point = Some((px+1, py)) }
                        Ordering::Greater => { last_point = Some((px-1, py)) }
                        Ordering::Equal => { /* skip */ }
                    }
                    match py.cmp(&y) {
                        Ordering::Less => { last_point = Some((px, py+1)) }
                        Ordering::Greater => { last_point = Some((px, py-1)) }
                        Ordering::Equal => { /* skip */ }
                    }
                    if (px, py) == (x,y) {break} // putting this in the while condition is not
                }
                last_point = Some(point);
            }
        }
    }

    fn print(&self) {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                print!("{}", self.cells[y][x]);
            }
            println!("");
        }
        println!("");
    }

}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let lines = lines.collect::<Vec<String>>();

    let mut map = Map::new(&lines);
    map.read_walls(&lines);
    map.print();
}
