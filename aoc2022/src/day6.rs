use std::collections::HashSet;

const CODE_LENGTH:usize = 14;

fn all_different(buff:&[char; CODE_LENGTH]) -> bool {
    let buff_set = HashSet::<char>::from_iter(buff.iter().cloned());
    buff_set.len() == CODE_LENGTH
}

#[allow(dead_code)]
pub fn solve(mut lines: impl Iterator<Item=String>) {
    let line = lines.next().unwrap();
    let mut buff: [char; CODE_LENGTH] = Default::default();
    for (i,c) in line.chars().enumerate() {
        let c = c;
        println!("{}:{}", i, c);
        buff[i % CODE_LENGTH] = c;
        if i > CODE_LENGTH && all_different(&buff) {
            println!("solution {}", i+1) ;
            break;
        }
    }
}
