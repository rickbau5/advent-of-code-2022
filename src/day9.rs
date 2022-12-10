use std::collections::HashSet;

use crate::input;

pub fn run() -> (String, String) {
    let inp = input::load_input(9);
    // let inp = input::load_test();
    return (run_part1(inp.clone()), run_part2(inp.clone()));
}

enum Move {
    Up(u32),
    Down(u32),
    Right(u32),
    Left(u32),
}

fn parse_input(inp: String) -> Vec<Move> {
    let mut moves = vec![];
    for line in inp.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let spaces = parts[1].parse().unwrap();
        moves.push(match parts[0] {
            "U" => Move::Up(spaces),
            "D" => Move::Down(spaces),
            "R" => Move::Right(spaces),
            "L" => Move::Left(spaces),
            _ => unreachable!("unhandled move"),
        })
    }

    moves
}

#[derive(Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn print_rope(head: Point, tail: Point, start: Point, size: (i32, i32)) {
    for y in 0..size.1 {
        let line = (0..size.0)
            .map(|x| {
                if head.x == x && head.y == y {
                    'H'
                } else if tail.x == x && tail.y == y {
                    'T'
                } else if start.x == x && start.y == y {
                    's'
                } else {
                    '.'
                }
            })
            .collect::<String>();

        println!("{}", line)
    }
}

fn print_visited(visited: HashSet<Point>) {
    let max = visited
        .iter()
        .fold((0, 0), |acc, Point { x, y }| (acc.0.max(*x), acc.1.max(*y)));

    dbg!(max);

    for y in 0..max.1 + 1 {
        let line = (0..max.0 + 1)
            .map(|x| {
                if x == 0 && y == max.1 {
                    's'
                } else if let Some(_) = visited.get(&Point { x: x, y: y }) {
                    '#'
                } else {
                    '.'
                }
            })
            .collect::<String>();

        println!("{}", line)
    }
}

fn run_part1(inp: String) -> String {
    let moves = parse_input(inp);
    // let visited: HashSet<Point> = HashSet::new();

    let visited: HashSet<Point> = HashSet::from([
        Point { x: 2, y: 0 },
        Point { x: 3, y: 0 },
        Point { x: 3, y: 1 },
        Point { x: 4, y: 1 },
        Point { x: 1, y: 2 },
        Point { x: 2, y: 2 },
        Point { x: 3, y: 2 },
        Point { x: 4, y: 2 },
        Point { x: 4, y: 3 },
        Point { x: 0, y: 4 },
        Point { x: 1, y: 4 },
        Point { x: 2, y: 4 },
        Point { x: 3, y: 4 },
    ]);

    print_visited(visited);
    print_rope(
        Point { x: 3, y: 2 },
        Point { x: 4, y: 1 },
        Point { x: 0, y: 4 },
        (6, 5),
    );

    "".to_string()
}

fn run_part2(inp: String) -> String {
    let mut res = 0;

    res += 1;
    println!("input:\n{}", inp);

    return res.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";

    #[test]
    fn test_part1() {
        let res = run_part1(INPUT.to_string());
        assert_eq!(res, "1")
    }

    #[test]
    fn test_part2() {
        let res = run_part2(INPUT.to_string());
        assert_eq!(res, "1")
    }
}
