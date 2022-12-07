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
            while parent_id.is_some() {
                filesystem[parent_id.unwrap()].dirsize += fsize;
                parent_id = filesystem[parent_id.unwrap()].parent;
            }
        }
    }
    filesystem
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let dirs = parse_directories(lines);

    let problem1_size = dirs.iter().fold(0, |accum, item| 
        if item.dirsize <= 100000 {
            accum + item.dirsize
        } else {
            accum
        }
    );
    println!("problem 1: {}", problem1_size);

    const TOTAL: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;
    let used = dirs[0].dirsize;
    let space_to_free = used - (TOTAL-REQUIRED_SPACE);

    let problem2_size = dirs.iter().fold(None, |accum, item| {
        match accum {
            None => {
                Some(item.dirsize)
            }
            Some(dirsize) => {
                if item.dirsize >= space_to_free && item.dirsize < dirsize {
                    Some(item.dirsize)
                } else {
                    Some(dirsize)
                }
            }
        }
    });

    println!("problem 2: {}", problem2_size.unwrap());
}
