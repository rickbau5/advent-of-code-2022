use std::collections::HashSet;

use crate::input;

pub fn run() -> (String, String) {
    let inp = input::load_input(6);
    // let inp = input::load_test();
    return (run_part1(inp.clone()), run_part2(inp.clone()));
}

// parse returns the index of the start of the first n distinct run of characters in the string.
fn parse(inp: impl Into<String>, n: usize) -> Option<usize>{
    let chars = inp.into().chars().collect::<Vec<char>>();
    let mut res = 0;

    for i in 0..chars.len() - n {
        let mut hs = HashSet::new();
        for c in &chars[i..i+n] {
            let before = hs.len();
            hs.insert(c);
            if hs.len() - before != 1 {
                break;
            }
        }

        if hs.len() == n {
            res = i;
            break;
        }
    }

    return match res {
        0 => None,
        _ => Some(res)
    };
}

fn run_part1(inp: String) -> String {
    let res = parse(inp, 4).unwrap() + 4;

    return res.to_string();
}

fn run_part2(inp: String) -> String {
    let res = parse(inp, 14).unwrap() + 14;
    return res.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb\n";

    #[test]
    fn test_part1() {
        let res = run_part1(INPUT.to_string());
        assert_eq!(res, "7")
    }

    #[test]
    fn test_part2() {
        let res = run_part2(INPUT.to_string());
        assert_eq!(res, "19")
    }
}
