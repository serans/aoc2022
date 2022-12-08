/*
* Visibility l, r, t, d
* Height
*/

struct Tree {
    vis_up: bool,
    vis_down: bool,
    vis_left: bool,
    vis_right: bool,
    height: u32,
}

impl Tree {
    fn new(height: u32) -> Tree {
        Tree {
            height,
            vis_up: false,
            vis_down: false,
            vis_left: false,
            vis_right: false,
        }
    }

    fn visible(&self) -> bool {
        self.vis_up || self.vis_right || self.vis_down || self.vis_left
    }
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    // parse forest
    let mut forest: Vec<Vec<Tree>> = Vec::new();
    for l in lines {
        let mut forest_line: Vec<Tree> = Vec::new();
        for t in l.chars() {
            forest_line.push(Tree::new(t as u32 - 48));
        }
        forest.push(forest_line);
    }

    let forest_width = forest[0].len();
    let forest_height = forest.len();

    // left
    for y in 0..forest_height {
        for x in 0..forest_width {
            if x == 0 { 
                forest[y][x].vis_left = true
            } else {
                forest[y][x].vis_left = forest[y][x-1].vis_left && forest[y][x-1].height < forest[y][x].height;
            }
        }
    }

    // right
    for y in 0..forest_height {
        for x in (0..forest_width).rev() {
            if x == forest_width - 1 { 
                forest[y][x].vis_right = true
            } else {
                forest[y][x].vis_right = forest[y][x+1].vis_right && forest[y][x+1].height < forest[y][x].height;
            }
        }
    }

    // top
    for x in 0..forest_width {
        for y in 0..forest_height {
            if y == 0 { 
                forest[y][x].vis_up = true
            } else {
                forest[y][x].vis_up = forest[y-1][x].vis_up && forest[y-1][x].height < forest[y][x].height;
            }
        }
    }

    // down
    for x in 0..forest_width {
        for y in (0..forest_height).rev() {
            if y == forest_height-1 { 
                forest[y][x].vis_down = true
            } else {
                forest[y][x].vis_down = forest[y+1][x].vis_down && forest[y+1][x].height < forest[y][x].height;
            }
        }
    }


    let mut num_visible = 0;
    for l_x in forest {
        for t in l_x {
            if t.visible() {
                num_visible += 1;
                print!("\x1b[31m{}\x1b[0m", t.height)
            } else {
                print!("{}", t.height);
            }
        }
        println!("");
    }
    println!("number of visible trees {}", num_visible);

}
