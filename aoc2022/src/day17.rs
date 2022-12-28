const ROCK_HEIGHT:usize=4;
const NUM_ROCKS:usize=5;
const CHAMBER_WALLS:u16=0b100000001;
// The tall, vertical chamber is exactly seven units wide. 

type Rock = [u16; ROCK_HEIGHT];
const ROCKS: [Rock;NUM_ROCKS] = [
    //NOTE: rocks flipped upside down in this representation
//  0b_0000000 <--- vertical chamber (MSB not used)
    [0b1111,
     0b0000,
     0b0000,
     0b0000,]
    ,
    [0b0100,
     0b1110,
     0b0100,
     0b0000,]
    ,
    [0b1110,
     0b0010,
     0b0010,
     0b0000,]
    ,
    [0b1000,
     0b1000,
     0b1000,
     0b1000,]
    ,
    [0b1100,
     0b1100,
     0b0000,
     0b0000,]
];

fn print_chamber(chamber: &Vec<u16>) {
    for y in (0..chamber.len()).rev() {
        let row = chamber[y];
        // rock can be here
        for n in (0..9).rev() {
            if 1<<n & row != 0 { print!("#")}
            else { print!(".")}
        }
        println!();
    }
    println!();
}

fn print_scene(chamber: &Vec<u16>, rock: &Rock, rock_y:usize) {
    for y in (0..chamber.len()).rev() {
        print!("{y:5} ");
        let row = chamber[y];
        // rock can be here
        for n in (0..9).rev() {
            if rock_y+ROCK_HEIGHT > y && y >= rock_y  {
                if rock[y-rock_y] & 1<<n != 0 { print!("@")}
                else if 1<<n & row != 0 { print!("#")}
                else {print!(".")}
            } else {
                if 1<<n & row != 0 { print!("#")}
                else { print!(".")}
            }
        }
        println!();
    }
    println!();
}

enum Direction {
    Left,
    Right,
}
/*
fn shift(rock:Rock, rock_y:usize, chamber: &Vec<u16>,   direction:Direction) -> Rock {
    let mut shifted_rock = rock.clone();
    for n in 0..ROCK_HEIGHT {
        let chamber_row = chamber[rock_y+n];
        match direction {
            Direction::Right => {
                if rock[n] & 0b1 != 0 { return rock }
                shifted_rock[n] = shifted_rock[n] >> 1;
            }
            Direction::Left => {
                if rock[n] & 0b1000000 != 0 { return rock }
                shifted_rock[n] = shifted_rock[n] >> 1;
            }
        }
        if chamber_row & shifted_rock[n] !=0 { return rock }
    }
    shifted_rock
}
*/

fn collides(rock:&Rock, rock_y:usize, chamber: &Vec<u16>) -> bool {
    for n in 0..ROCK_HEIGHT {
        let chamber_row = chamber[rock_y+n];
        if chamber_row & rock[n] != 0 { return true }
    }
    false
}

fn height_of_top_rock(chamber: &Vec<u16>) -> usize {
    let mut top_rock = 0;
    for j in (0..chamber.len()).rev() {
        if chamber[j] != CHAMBER_WALLS { 
            top_rock = j;
            break;
        }
    }
    top_rock
}

#[allow(dead_code)]
pub fn solve(lines: impl Iterator<Item = String>) {
    let mut chamber:Vec<u16> = vec![0b111111111];

    let actions:Vec<u8> = lines.into_iter().next().unwrap().bytes().collect();

    for i in 0..4 {
        //let top_rock
        let top_rock = height_of_top_rock(&chamber);
        for _ in 0..((ROCK_HEIGHT + 4) - (chamber.len() - top_rock)) {
            chamber.push(CHAMBER_WALLS);
        }

        let action = actions[i % actions.len()];
        let mut rock = ROCKS[i%NUM_ROCKS];
        let mut rock_y = top_rock+4;
        print!("COLLIDES? {}", collides(&rock, rock_y, &chamber));

        print_scene(&chamber, &rock, rock_y);
        for n in 0..ROCK_HEIGHT {
            if rock[n] == 0 { break }
            chamber[rock_y+n] |= rock[n];
        }
    }
}
