pub struct Dir {
    parent: Option<usize>,
    dirsize: usize,
}

pub fn parse_directories(lines: impl Iterator<Item = String>) -> Vec<Dir> {
    let mut filesystem: Vec<Dir> = Vec::new();

    filesystem.push(Dir {
        parent: None,
        dirsize: 0,
    });

    let mut curr_idx = 0;

    for l in lines {
        if l == "$ cd /" {
            curr_idx = 0;
        } else if l == "$ cd .." {
            curr_idx = filesystem[curr_idx].parent.unwrap();
        } else if l.starts_with("$ cd") {
            // note: this works because no dir is listed twice
            filesystem.push(Dir {
                parent: Some(curr_idx),
                dirsize: 0,
            });
            curr_idx = filesystem.len() - 1;
        } else if !l.starts_with("$ ls") && !l.starts_with("dir") {
            let fsize = l.split(" ").next().unwrap().parse::<usize>().unwrap();
            filesystem[curr_idx].dirsize += fsize;
            // update parent dirs
            let mut parent_id = filesystem[curr_idx].parent;
            loop {
                match parent_id {
                    Some(id) => {
                        parent_id = filesystem[id].parent;
                        filesystem[id].dirsize += fsize;
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }
    filesystem
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let dirs = parse_directories(lines);

    let mut problem1_size = 0;
    for dir in &dirs {
        if dir.dirsize <= 100000 {
            problem1_size += dir.dirsize;
        }
    }
    println!("problem 1: {}", problem1_size);

    const TOTAL: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;
    const USABLE: usize = TOTAL - REQUIRED_SPACE;
    let used = dirs[0].dirsize;

    let space_to_free = used - USABLE;

    let mut problem2_size: Option<usize> = None;
    for dir in &dirs {
        if dir.dirsize >= space_to_free {
            match problem2_size {
                None => {
                    problem2_size = Some(dir.dirsize);
                }
                Some(x) => {
                    if x > dir.dirsize {
                        problem2_size = Some(dir.dirsize)
                    }
                }
            }
        }
    }
    println!("problem 2: {}", problem2_size.unwrap());
}
