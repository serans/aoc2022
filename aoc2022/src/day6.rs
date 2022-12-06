use std::collections::HashSet;

fn all_elements_unique(text: &Vec<char>) -> bool {
    let letter_set = HashSet::<char>::from_iter(text.iter().cloned());
    text.len() == letter_set.len()
}

fn find_unique_sequence(len: usize, text: &String) -> Option<usize> {
    let mut buff = std::vec::from_elem(' ', len);
    for (i,c) in text.chars().enumerate() {
        buff[i % len] = c;
        if i > len && all_elements_unique(&buff) {
            return Some(i+1)
        }
    }
    None
}

#[allow(dead_code)]
pub fn solve(mut lines: impl Iterator<Item=String>) {
    let text = lines.next().unwrap();
    println!("Problem 1: {}", find_unique_sequence( 4, &text).unwrap());
    println!("Problem 2: {}", find_unique_sequence(14, &text).unwrap());
}
