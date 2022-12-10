use std::collections::HashSet;

use crate::input;

pub fn run() -> (String, String) {
    let inp = input::load_input(9);
    // let inp = input::load_test();
    return (run_part1(inp.clone()), run_part2(inp.clone()));
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    spaces: u32,
}

fn parse_input(inp: String) -> Vec<Move> {
    inp.lines().map(|line| {
        let parts: Vec<&str> = line.split(" ").collect();
        let spaces = parts[1].parse().unwrap();
        let direction = match parts[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => unreachable!("unhandled move"),
        };

        Move{direction, spaces}
    }).collect::<Vec<Move>>()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

fn print_visited(visited: &HashSet<Point>) {
    let max = visited
        .iter()
        .fold((0, 0), |acc, Point { x, y }| (acc.0.max(*x), acc.1.max(*y)));

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

fn get_tail_move(tail: Point, head: Point) -> Point {
    if tail.x.max(head.x) - tail.x.min(head.x) < 2 && tail.y.max(head.y) - tail.y.min(head.y) < 2 {
        return tail;
    }

    if tail.x == head.x {
        // left - right
        Point {
            x: tail.x,
            y: tail.y + (if head.y > tail.y { 1 } else { -1 }),
        }
    } else if tail.y == head.y {
        // up - down
        Point {
            x: tail.x + (if head.x > tail.x { 1 } else { -1 }),
            y: tail.y,
        }
    } else {
        let x = if tail.x > head.x {
            tail.x - 1
        } else {
            tail.x + 1
        };
        let y = if tail.y > head.y {
            tail.y - 1
        } else {
            tail.y + 1
        };
        // diagonal
        Point { x, y }
    }
}

fn do_moves(mv: Move, head: &mut Point, tail: &mut Point, visited: &mut HashSet<Point>) {
    for _ in 0..mv.spaces {
        match mv.direction {
            Direction::Down => {
                head.y -= 1;
            },
            Direction::Up => {
                head.y += 1;
            },
            Direction::Left => {
                head.x -= 1;
            },
            Direction::Right => {
                head.x += 1;
            },
        };

        let new_pos = get_tail_move(tail.clone(), head.clone());
        tail.x = new_pos.x;
        tail.y = new_pos.y;
        visited.insert(new_pos);
    }
}

fn run_part1(inp: String) -> String {
    let moves = parse_input(inp);
    let mut visited: HashSet<Point> = HashSet::new();

    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };

    for mv in moves {
        do_moves(mv, &mut head, &mut tail, &mut visited)
    }

    print_visited(&visited);

    visited.len().to_string()
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
        assert_eq!(res, "13")
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let res = run_part2(INPUT.to_string());
        assert_eq!(res, "1")
    }

    #[test]
    fn test_get_tail_move() {
        // move right
        assert_eq!(
            get_tail_move(Point { x: 1, y: 1 }, Point { x: 3, y: 1 }),
            Point { x: 2, y: 1 }
        );
        // move left
        assert_eq!(
            get_tail_move(Point { x: 3, y: 1 }, Point { x: 1, y: 1 }),
            Point { x: 2, y: 1 }
        );
        // move "down"
        assert_eq!(
            get_tail_move(Point { x: 1, y: 1 }, Point { x: 1, y: 3 }),
            Point { x: 1, y: 2 }
        );
        // move "up"
        assert_eq!(
            get_tail_move(Point { x: 1, y: 3 }, Point { x: 1, y: 1 }),
            Point { x: 1, y: 2 }
        );

        // diag up & right
        assert_eq!(
            get_tail_move(Point { x: 1, y: 3 }, Point { x: 2, y: 1 }),
            Point { x: 2, y: 2 },
        );

        // diag up & right still
        assert_eq!(
            get_tail_move(Point { x: 1, y: 3 }, Point { x: 3, y: 2 }),
            Point { x: 2, y: 2 },
        );
    }
}
