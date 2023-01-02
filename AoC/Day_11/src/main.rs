
use std::cell::RefCell;

#[derive(Debug)]
struct Monkey {
    items: RefCell<Vec<i64>>,
    operation: String,
    test: i64,
    targets: [usize; 2],
}


fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for monkey_def in input.split("\n\n") {
        let mut lines = monkey_def.lines();
        lines.next();
        let items_str = lines.next().unwrap().split(": ").nth(1).unwrap();
        let items: Vec<i64> = items_str.split(", ").filter_map(|s| s.parse::<i64>().ok()).collect();
        let operation = &lines.next().unwrap()[23..];
        let test = lines.next().unwrap().split("by ").nth(1).unwrap().parse().unwrap();
        let target_1 = lines.next().unwrap().split("monkey ").nth(1).unwrap().parse().unwrap();
        let target_2 = lines.next().unwrap().split("monkey ").nth(1).unwrap().parse().unwrap();
        let monkey = Monkey {items: RefCell::new(items), operation: operation.to_string(), test: test, targets: [target_1, target_2]};
        monkeys.push(monkey);
    }
    monkeys
}


fn main() {
    let input = include_str!("../input.txt");
    let monkeys = parse_input(input);
    println!("{:?}", monkeys);
    let mut count = vec![0; monkeys.len()];
    let m: i64 = monkeys.iter().map(|m| m.test).product();
    for _ in 0..10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in monkey.items.borrow().iter() {
                count[i] += 1;
                let mut new_item = *item;
                if monkey.operation.chars().nth(0) == Some('+') {
                    new_item = item + monkey.operation.split("+ ").nth(1).unwrap().parse::<i64>().unwrap(); 
                }
                if monkey.operation.chars().nth(0) == Some('*') {
                    let val = monkey.operation.split("* ").nth(1).unwrap();
                    if let Ok(v) = val.parse::<i64>(){
                        new_item = item * v; 
                    }
                    else {
                        new_item = item * item;
                    }
                }
                new_item %= m;
                monkeys[monkey.targets[(new_item % monkey.test != 0) as usize]].items.borrow_mut().push(new_item);
            }
            monkey.items.borrow_mut().clear();   
        }
    }
    count.sort_unstable_by(|a, b| b.cmp(a));
    println!("{:?}", count);
}

