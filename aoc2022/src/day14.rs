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

    #[allow(unused)]
    fn print(&self) {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                print!("{}", self.cells[y][x]);
            }
            println!("");
        }
        println!("");
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
        let mut sand_x:usize = 500-self.x_offset;
        let mut sand_y:usize = 0;
        
        loop {
            match self.sand_step(sand_x, sand_y) {
                Some((new_x, new_y)) => { 
                    if new_x == sand_x && new_y == sand_y {
                        self.cells[sand_y][sand_x] = 'o';
                        return true;
                    } else {
                        sand_x = new_x;
                        sand_y = new_y;
                    }
                }
                None => {return false}
            }
        }
    }

}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let lines = lines.collect::<Vec<String>>();

    let mut map = Map::new(&lines);
    map.read_walls(&lines);
  
    let mut n=0;
    /*
    for _ in 0..27 {
        println!("{n} ---------------------");
        let x = map.drop_sand();
        if x {println!("continue")} else {println!("stop")}
        map.print();
        n+=1;
    }
        */
    while map.drop_sand() { n+=1 }
    println!("Problem 1: Dropped {n} grains of sand");
}
