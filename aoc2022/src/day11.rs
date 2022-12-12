enum Operator {
    Old,
    Scalar(u64),
}

enum Operation {
    Addition,
    Multiplication,
}

struct Monkey {
    items: Vec<u64>,
    test: u64,
    if_true: usize,
    if_false: usize,
    operation: Operation,
    operator: Operator,
    inspected_items : u64,
}

struct Throw {
    monkey_index: usize,
    item: u64,
}

impl Monkey {
    fn turn(&mut self, first_round: bool, lcm: u64) -> Vec<Throw> {
        let mut throws: Vec<Throw> = Vec::new();
        for item in self.items.iter() {
            self.inspected_items+=1;
            let mut item = *item;
            let op2 = match self.operator {
                Operator::Old => { item },
                Operator::Scalar(x) => { x },
            };

            item = match self.operation {
                Operation::Addition => { item + op2 }
                Operation::Multiplication => { item * op2 }
            };
            if first_round { item = item / 3 }
            item = item % lcm;

            if item % self.test == 0 {
                throws.push(Throw{monkey_index:self.if_true, item})
            } else {
                throws.push(Throw{monkey_index:self.if_false, item})
            }
        }
        self.items = Vec::new();
        throws
    }
}

#[allow(dead_code)]
pub fn solve() {
    let mut monkeys: Vec<Monkey> = vec![
        // faster than parsing...
        Monkey{
          items:vec![ 93, 98],
          operator: Operator::Scalar(17),
          operation: Operation::Multiplication,
          test:19,
          if_true :  5,
          if_false :  3,
          inspected_items:0,
        },
        Monkey{
          items:vec![ 95, 72, 98, 82, 86],
          operator: Operator::Scalar(5),
          operation: Operation::Addition,
          test:13,
          if_true :  7,
          if_false :  6,
          inspected_items:0,
        },
        Monkey{
          items:vec![ 85, 62, 82, 86, 70, 65, 83, 76],
          operator: Operator::Scalar(8),
          operation: Operation::Addition,
          test:5,
          if_true :  3,
          if_false :  0,
          inspected_items:0,
        },
        Monkey{
          items:vec![ 86, 70, 71, 56],
          operator: Operator::Scalar(1),
          operation: Operation::Addition,
          test:7,
          if_true :  4,
          if_false :  5,
          inspected_items:0,
        },
        Monkey{
          items:vec![ 77, 71, 86, 52, 81, 67],
          operator: Operator::Scalar(4),
          operation: Operation::Addition,
          test:17,
          if_true :  1,
          if_false :  6,
          inspected_items:0,
        },
        Monkey{
          items:vec![ 89, 87, 60, 78, 54, 77, 98],
          operator: Operator::Scalar(7),
          operation: Operation::Multiplication,
          test:2,
          if_true :  1,
          if_false :  4,
          inspected_items:0,
        },
        Monkey{
          items:vec![ 69, 65, 63],
          operator: Operator::Scalar(6),
          operation: Operation::Addition,
          test:3,
          if_true :  7,
          if_false :  2,
          inspected_items:0,
        },
        Monkey{
          items: vec![ 89],
          operator: Operator::Old,
          operation: Operation::Multiplication,
          test:11,
          if_true :  0,
          if_false :  2,
          inspected_items:0,
        }
    ];


    let lcm = monkeys.iter().fold(1, |acc,monke| acc*monke.test);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for throw in monkeys[i].turn(false, lcm) {
                monkeys[throw.monkey_index].items.push(throw.item);
            }
        }
    }
    
    let mut inspected_items = monkeys.iter().map(|m| {m.inspected_items}).collect::<Vec<u64>>();
    inspected_items.sort_by(|a,b| b.cmp(a));
    let monkey_business:u64 = inspected_items[0..2].iter().product();
    println!("monkey business {}", monkey_business);
}
