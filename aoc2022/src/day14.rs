use std::cmp::Ordering;

struct Map {
    width: usize,
    height: usize,
    cells: Vec<Vec<char>>,
}

impl Map {
    fn new(lines: &Vec<String>) -> Map {
        let mut max_x = 0;
        let mut max_y = 0;
        for row in lines.iter().clone() {
            for point in row.split("->") {
                let point:Vec<&str> = point.split(",").collect();
                max_x = max_x.max(point[0].trim().parse::<usize>().unwrap());
                max_y = max_y.max(point[1].trim().parse::<usize>().unwrap());
            }
        }
        let mut map = Map {
            width: max_x + 2,
            height:max_y + 2,
            cells: Vec::new(),
        };
        map.cells = vec![vec!['.'; map.width]; map.height];
        map.read_walls(lines);
        map
    }

    fn read_walls(&mut self, lines: &Vec<String>) {
        let mut last_point:Option<(usize, usize)>;
        for row in lines.iter().clone() {
            last_point = None;
            for point in row.split("->") {
                let point:Vec<&str> = point.split(",").collect();
                let x = point[0].trim().parse::<usize>().unwrap();
                let y = point[1].trim().parse::<usize>().unwrap();
                let point = (x,y);
                while let Some((px, py)) = last_point {
                    self.cells[py][px] = '#';
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

    fn sand_step(&self, x:usize, y:usize) -> Option<(usize, usize)> {
        // sand fell of the bottom
        if y == self.height-1 { return None }

        // sand continues straight down
        if self.cells[y+1][x] == '.' { return Some((x, y+1)) }

        // sand fell of the left edge
        if x == 0 { return None }

        // sand continues diagonal left
        if self.cells[y+1][x-1] == '.' { return Some((x-1, y+1))}

        // sand fell of the right edge
        if x == self.width-1 { return None }

        // sand continues diagonal right
        if self.cells[y+1][x+1] == '.' { return Some((x+1, y+1))}

        // sand stands where it is
        return Some((x,y))
    }

    fn drop_sand(&mut self) -> bool {
        // sand starts at 500,0
        let mut sand_x:usize = 500;
        let mut sand_y:usize = 0;
        
        loop {
            match self.sand_step(sand_x, sand_y) {
                Some((new_x, new_y)) => { 
                    if new_x == sand_x && new_y == sand_y {
                        // sand settled
                        self.cells[sand_y][sand_x] = 'o';
                        return true;
                    } else {
                        // sand continues to fall
                        sand_x = new_x;
                        sand_y = new_y;
                    }
                }
                None => {
                    // sand fell off the edge
                    return false
                }
            }
        }
    }

}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let mut lines = lines.collect::<Vec<String>>();

    // problem 1
    let mut map = Map::new(&lines);
    let mut n=0;
    while map.drop_sand() { n+=1 }
    println!("Problem 1: Dropped {n} grains of sand");

    // problem 2
    let floor_height = map.height;
    let bottom_line = format!("1,{floor_height} -> 1000,{floor_height}");
    // add floor below the map
    lines.push(bottom_line);
    let mut map2 = Map::new(&lines);

    let mut n=0;
    while map2.drop_sand() {
        n+=1;
        if map2.cells[0][500] == 'o' {
            break;
        }
    }
    println!("Problem 2: Sand reaches top after {n} grains have fallen");
}
