pub struct Dir {
    parent: Option<usize>,
    size: usize,
}

pub fn parse_directories(lines: impl Iterator<Item = String>) -> Vec<Dir> {
    let mut filesystem: Vec<Dir> = Vec::new();

    filesystem.push(Dir {
        parent: None,
        size: 0,
    });

    let mut dir_index = 0;

    for l in lines {
        if l == "$ cd /" {
            dir_index = 0;
        } else if l == "$ cd .." {
            dir_index = filesystem[dir_index].parent.unwrap();
        } else if l.starts_with("$ cd") {
            // note: this only works because no dir is listed twice
            filesystem.push(Dir {
                parent: Some(dir_index),
                size: 0,
            });
            dir_index = filesystem.len() - 1;
        } else if !l.starts_with("$ ls") && !l.starts_with("dir") {
            let fsize = l.split(" ").next().unwrap().parse::<usize>().unwrap();
            filesystem[dir_index].size += fsize;
            // update parent dirs
            let mut parent_id = filesystem[dir_index].parent;
            while parent_id.is_some() {
                filesystem[parent_id.unwrap()].size += fsize;
                parent_id = filesystem[parent_id.unwrap()].parent;
            }
        }
    }
    filesystem
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let dirs = parse_directories(lines);

    //let problem1_size = dirs.iter().filter(|d| d.size <= 100000).fold(0, |acc, item| {acc+item.size});
    let problem1_size = dirs.iter().filter(|d| d.size <= 100000).sum();
    println!("problem 1: {}", problem1_size);

    let used = dirs[0].size;
    let space_to_free = used - (70000000 - 30000000);
    let problem2_size = dirs.iter().filter(|d| d.size>= space_to_free).reduce(|acc, item| {
        if acc.size < item.size {
            acc
        } else {
            item
        }
    }).unwrap().size;
    println!("problem 2: {}", problem2_size);
}
