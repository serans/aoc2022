use std::cmp::Ordering;

#[derive(Debug)]
enum List {
    Number(u32),
    List(Vec<List>),
}

impl Clone for List {
    fn clone(&self) -> List {
        match &self {
            List::Number(x) => { List::Number(*x) }
            List::List(x) => {
                List::List(x.clone())
            }
        }
    }
}

struct Parser {
    text: String,
    i: usize,
}
use std::str;

impl Parser {
    fn parse(text: String) -> List {
        Parser {
            text,
            i:0
        }.parse_rec()
    }

    fn parse_number_from(&self, start: usize) -> u32 {
        str::from_utf8(&self.text.as_bytes()[start..self.i])
            .unwrap()
            .parse::<u32>()
            .unwrap()
    }

    fn parse_rec(&mut self) -> List {
        let mut curr_list: Vec<List> = Vec::new();
        let mut number_start: Option<usize> = None;
        while self.i < self.text.as_bytes().len() {
            match self.text.as_bytes()[self.i] as char {
                '[' => {
                    self.i += 1;
                    curr_list.push(self.parse_rec());
                }
                ']' => {
                    if let Some(start) = number_start {
                        curr_list.push(List::Number(self.parse_number_from(start)));
                    }
                    self.i += 1;
                    return List::List(curr_list);
                }
                ',' => {
                    if let Some(start) = number_start {
                        curr_list.push(List::Number(self.parse_number_from(start)));
                        number_start = None;
                    }
                    self.i += 1;
                }
                _ => {
                    if let None = number_start {
                        number_start = Some(self.i);
                    }
                    self.i += 1;
                }
            }
        }
        List::List(curr_list)
    }
}

fn cmp(left: &List, right: &List) -> Ordering {
    match left {
        List::Number(l) => {
            match right {
                List::Number(r) => {
                    l.cmp(r)
                }
                List::List(_) => {
                    cmp(&List::List(vec![left.clone()]), right)
                }
            }
        }
        List::List(l) => {
            match right {
                List::Number(_) => {
                    cmp(left, &List::List(vec![right.clone()]))
                }
                List::List(r) => {
                    for i in 0..l.len() {
                        if i >= r.len() {
                            return Ordering::Greater;
                        }
                        match cmp(&l[i], &r[i]) {
                            Ordering::Less => return Ordering::Less,
                            Ordering::Greater=> return Ordering::Greater,
                            _ => { /* continue */ }
                        }
                    }
                    return l.len().cmp(&r.len())
                }
            }
        }
    }
}

pub fn solve(lines: impl Iterator<Item = String>) {
    // parsing
    let lines = lines.collect::<Vec<String>>();

    let mut good_pair_sum = 0;
    let mut i = 0;

    let mut packets: Vec<List> = Vec::new();

    while i < lines.len() {
        let left = lines[i].clone();
        let right = lines[i + 1].clone();

        let packet_1 = Parser::parse(left.clone());
        let packet_2 = Parser::parse(right.clone());

        if cmp(&packet_1, &packet_2) == Ordering::Less {
            good_pair_sum += (i/3)+1;
        }
        i += 3;

        packets.push(packet_1);
        packets.push(packet_2);
    }
    println!("Solution 1: {}", good_pair_sum);
    
    // problem2, add separator packets
    let p1 = Parser::parse(String::from("[[2]]"));
    let p2 = Parser::parse(String::from("[[6]]"));

    packets.push(p1.clone());
    packets.push(p2.clone());
    packets.sort_by(|a,b| cmp(a,b));
    let pos1 = packets.iter().position(|r| cmp(&r,&p1) == Ordering::Equal).unwrap()+1;
    let pos2 = packets.iter().position(|r| cmp(&r,&p2) == Ordering::Equal).unwrap()+1;
    // 23862 too low
    // 19046 too low
    println!("Solution 2: {}", pos1 * pos2);
}
