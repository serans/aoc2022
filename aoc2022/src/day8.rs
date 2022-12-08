struct Tree {
    vis_up: bool,
    vis_down: bool,
    vis_left: bool,
    vis_right: bool,
    height: i32,
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    // parse forest
    let mut forest: Vec<Vec<Tree>> = Vec::new();
    for l in lines {
        let mut forest_line: Vec<Tree> = Vec::new();
        for t in l.chars() {
            forest_line.push(Tree{
                height: t as i32 - 48, // ascii to int
                vis_up: false,
                vis_down: false,
                vis_right: false,
                vis_left: false,
            });
        }
        forest.push(forest_line);
    }

    let forest_width = forest[0].len();
    let forest_height = forest.len();

    // left
    for y in 0..forest_height {
        let mut tallest_tree = -1;
        for x in 0..forest_width {
            if forest[y][x].height > tallest_tree {
                forest[y][x].vis_left = true;
                tallest_tree = forest[y][x].height;
            }
        }
    }

    // right
    for y in 0..forest_height {
        let mut tallest_tree = -1;
        for x in (0..forest_width).rev() {
            if forest[y][x].height > tallest_tree {
                forest[y][x].vis_left = true;
                tallest_tree = forest[y][x].height;
            }
        }
    }

    // top
    for x in 0..forest_width {
        let mut tallest_tree = -1;
        for y in 0..forest_height {
            if forest[y][x].height > tallest_tree {
                forest[y][x].vis_left = true;
                tallest_tree = forest[y][x].height;
            }
        }
    }

    // down
    for x in 0..forest_width {
        let mut tallest_tree = -1;
        for y in (0..forest_height).rev() {
            if forest[y][x].height > tallest_tree {
                forest[y][x].vis_left = true;
                tallest_tree = forest[y][x].height;
            }
        }
    }

    let mut num_visible = 0;
    for row in &forest {
        for tree in row {
            if  tree.vis_up || tree.vis_right || tree.vis_down || tree.vis_left {
                num_visible += 1;
            }
        }
    }
    println!("Number of visible trees: {}", num_visible);

    let mut max_score = 0;
    for y in 0..forest_height {
        for x in 0..forest_width {
            let height = forest[y][x].height;
            // left
            let mut score_left = 0;
            while x - score_left >0 {
                score_left +=1;
                if forest[y][x - score_left].height >= height {
                    break;
                }
            }
            // left
            let mut score_right = 0;
            while x + score_right < forest_width-1 {
                score_right +=1;
                if forest[y][x + score_right].height >= height {
                    break;
                }
            }
            // left
            let mut score_up = 0;
            while y - score_up >0 {
                score_up +=1;
                if forest[y - score_up][x].height >= height {
                    break;
                }
            }
            // left
            let mut score_down = 0;
            while y + score_down < forest_height-1 {
                score_down +=1;
                if forest[y + score_down][x].height >= height {
                    break;
                }
            }
            let score = score_up * score_down * score_right * score_left;
            if score > max_score { max_score = score }
        }
    }
    println!("Max scenic score: {}", max_score);
}
