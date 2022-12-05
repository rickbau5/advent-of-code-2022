use std::vec;

use crate::input;

pub fn run() -> (String, String) {
    let inp = input::load_input(5);
    // let inp = input::load_test();
    return (run_part1(inp.clone()), run_part2(inp.clone()));
}

fn run_part1(inp: String) -> String {
    let (mut stacks, moves) = parse_input(inp);

    for mv in moves {
        for _ in 0..mv.num {
            let cr = stacks.get_mut(mv.from as usize - 1).unwrap().pop().unwrap();
            stacks.get_mut(mv.to as usize - 1).unwrap().push(cr)
        }
    }

    let tops = stacks.iter().map(|f| *f.last().unwrap()).collect::<Vec<char>>();

    return tops.into_iter().collect();
}

fn run_part2(inp: String) -> String {
    let (mut stacks, moves) = parse_input(inp);

    for mv in moves {
        let crates: &mut Vec<char> = &mut vec![];
        for _ in 0..mv.num {
            let cr = stacks.get_mut(mv.from as usize - 1).unwrap().pop().unwrap();
            crates.push(cr);
        }
        crates.reverse();
        stacks.get_mut(mv.to as usize - 1).unwrap().append(crates);
    }

    let tops = stacks.iter().map(|f| *f.last().unwrap()).collect::<Vec<char>>();

    return tops.into_iter().collect();
}

#[derive(Debug)]
struct Move {
    num: i32,
    from: i32,
    to: i32
}

fn parse_input(inp: String) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut moves: Vec<Move> = vec![];
    let mut parse_stacks = true;

    for line in inp.lines(){
        if line.len() == 0 {
            parse_stacks = false;
            continue;
        }

        if parse_stacks {
            let chars = line.chars().collect::<Vec<char>>();
            let chunks = chars.chunks(4);

            if stacks.len() == 0 {
                for _ in 0..chunks.len() {
                    stacks.push(vec![]);
                }
            }

            let mut stack_index = 0;
            for chunk in chunks {
                if chunk[0] == '[' {
                    stacks[stack_index].push(chunk[1])
                }
                stack_index += 1
            }
        } else {
            // take the odd elements as numbers indicating num, from stack, and to stack respectively
            let parts = line.split(" ").
                zip(0..line.len()).
                filter_map(|pair| match pair.1 {
                    num if num % 2 == 1 => Some(pair.0.parse::<i32>().unwrap()),
                    _ => None
                }).collect::<Vec<i32>>();

            moves.push(Move{num: parts[0], from: parts[1], to: parts[2]})
        }
    }

    // we need to reverse the vecs because we pushed on to the back from top to bottom
    for i in 0..stacks.len() {
        stacks[i].reverse()
    }

    return (stacks, moves);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";

    #[test]
    fn test_part1() {
        let res = run_part1(INPUT.to_string());
        assert_eq!(res, "CMZ")
    }

    #[test]
    fn test_part2() {
        let res = run_part2(INPUT.to_string());
        assert_eq!(res, "MCD")
    }
}
