use crate::input;
use crate::utils;

use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() -> (String, String) {
    return (run_part1(), run_part2());
}

fn run_part2() -> String {
    let inp = input::load_input(3);
    let windows = utils::window_input(inp, 3);

    let mut prio_sum = 0;
    for window in windows {
        let mut all_occ = HashMap::new();

        for pack in window {
            let set: HashSet<char> = HashSet::from_iter(pack.chars().into_iter());
            for c in set {
                all_occ.entry(c).and_modify(|v| *v += 1).or_insert(1);
            }
        }

        prio_sum += char_priority(*all_occ.iter().filter(|v| *v.1 == 3).next().unwrap().0).1
    }

    return prio_sum.to_string();
}

fn run_part1() -> String {
    let inp = input::load_input(3);

    let mut prio_sum = 0;
    for ele in inp.lines() {
        let items_first = ele.get(..ele.len() / 2).unwrap();
        let items_second = ele.get(ele.len() / 2..).unwrap();
        let mut hs_first = HashSet::new();

        items_first.chars().for_each(|c| {
            hs_first.insert(c);
        });

        let mut in_both = HashSet::new();
        items_second
            .chars()
            .filter(|c| items_first.contains(*c))
            .for_each(|c| {
                in_both.insert(c);
            });

        let prio = in_both.iter().map(|c| char_priority(*c));

        let this_sum = prio.fold(0, |a, b| a + b.1);
        prio_sum += this_sum;
    }

    return prio_sum.to_string();
}

fn char_priority(c: char) -> (char, i32) {
    match c.is_uppercase() {
        true => (c, (c as i32) - 38),
        false => (c, (c as i32) - 96),
    }
}
