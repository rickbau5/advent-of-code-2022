use std::collections::HashMap;

use crate::input;

pub fn run() -> (String, String) {
    let inp = input::load_input(11);
    // let inp = input::load_test();
    return run_both(inp);
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Mult(u64),
    Add(u64),
    Sq,
}

impl Operation {
    fn apply(&self, value: u64) -> u64 {
        match self {
            Operation::Mult(by) => value * by,
            Operation::Add(by) => value + by,
            Operation::Sq => value * value,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn inspect(&self, item: u64) -> u64 {
        self.operation.apply(item)
    }

    fn test(&self, item: u64) -> usize {
        if item % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

fn parse(inp: String) -> Vec<Monkey> {
    let mut lines = inp.lines();
    let mut monkeys = vec![];

    loop {
        let monkey_line = lines.next();
        if monkey_line.is_none() {
            break;
        }

        let items = lines
            .next()
            .expect("should find starting items")
            .split(": ")
            .last()
            .expect("should find item list")
            .split(" ")
            .filter_map(|p| {
                if p.contains(",") {
                    p.replace(",", "").parse::<u64>().ok()
                } else {
                    p.parse::<u64>().ok()
                }
            })
            .collect::<Vec<u64>>();

        let operation = lines
            .next()
            .expect("should find start of operation")
            .split(": ")
            .last()
            .map(|s| {
                let mut parts = s.split(" ").collect::<Vec<&str>>();
                let by = parts.pop();
                let op = parts.pop();
                match by {
                    Some("old") => Operation::Sq,
                    Some(s) => match op {
                        Some("+") => Operation::Add(s.parse::<u64>().expect("should parse")),
                        Some("*") => Operation::Mult(s.parse::<u64>().expect("should parse")),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            })
            .expect("should get an operation");

        let test = lines
            .next()
            .expect("should find test")
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .expect("should get divisible by num");

        let if_true = lines
            .next()
            .expect("should find test true")
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("should get if true target");

        let if_false = lines
            .next()
            .expect("should find test false")
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("should get if false target");

        monkeys.push(Monkey {
            items: items.clone(),
            operation,
            test,
            if_true,
            if_false,
        });

        // consume whitespace
        lines.next();
    }

    monkeys
}

fn run_both(inp: String) -> (String, String) {
    let mut monkeys = parse(inp);
    let mut inspections: HashMap<usize, u32> = HashMap::new();

    let mut part_1 = "".to_string();

    for i in 1..21 {
        let mut throw_to: HashMap<usize, Vec<u64>> = HashMap::new();

        for m in 0..monkeys.len() {
            let monkey = monkeys.get_mut(m).unwrap();

            throw_to.entry(m).and_modify(|f| {
                monkey.items.append(f);
            });

            for j in 0..monkey.items.len() {
                let item = monkey.items[j];
                let mut worry = monkey.inspect(item);
                worry /= 3;
                let target = monkey.test(worry);
                throw_to
                    .entry(target)
                    .and_modify(|f| f.push(worry))
                    .or_insert(vec![worry]);
            }

            inspections
                .entry(m)
                .and_modify(|f| *f += monkey.items.len() as u32)
                .or_insert(monkey.items.len() as u32);

            monkey.items.clear();
        }

        for (to, items) in throw_to.iter() {
            for ele in items {
                monkeys[*to].items.push(*ele);
            }
        }
        throw_to.clear();

        if i == 20 {
            let mut top = inspections.clone().into_values().collect::<Vec<u32>>();
            top.sort();
            top.reverse();
            part_1 = (top[0] * top[1]).to_string()
        }
    }

    for monkey in 0..monkeys.len() {
        let num = inspections[&monkey];
        println!("Monkey {monkey} inspected items {num} times.");
    }

    let mut top = inspections.into_values().collect::<Vec<u32>>();
    top.sort();
    top.reverse();

    (part_1, (top[0] * top[1]).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";

    #[test]
    fn test_both() {
        let (part_1, part_2) = run_both(INPUT.to_string());
        assert_eq!(part_1, "10605");
        assert_eq!(part_2, "2713310158");
    }
}
