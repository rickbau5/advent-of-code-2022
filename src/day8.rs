use std::collections::HashMap;

use crate::input;

pub fn run() -> (String, String) {
    let inp = input::load_input(8);
    // let inp = input::load_test();
    return (run_part1(inp.clone()), run_part2(inp.clone()));
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_trees(inp: String) -> HashMap<Point, i32> {
    let mut trees = HashMap::new();

    for (y, line) in inp.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            trees.insert(
                Point {
                    x: x as i32,
                    y: y as i32,
                },
                char.to_string().parse().unwrap(),
            );
        }
    }

    return trees;
}

fn run_part1(inp: String) -> String {
    let trees = parse_trees(inp);

    let mut visible = 0;
    let mut pt = Point { x: 0, y: 0 };
    'outer: for (coord, height) in trees.iter() {
        pt.x = coord.x;
        pt.y = coord.y;

        pt.x += 1;
        loop {
            match trees.get(&pt) {
                Some(next) => {
                    if next >= height {
                        break;
                    }
                    pt.x += 1;
                }
                None => {
                    visible += 1;
                    continue 'outer;
                }
            }
        }

        pt.x = coord.x - 1;
        loop {
            match trees.get(&pt) {
                Some(next) => {
                    if next >= height {
                        break;
                    }
                    pt.x -= 1;
                }
                None => {
                    visible += 1;
                    continue 'outer;
                }
            }
        }

        pt.x = coord.x;
        pt.y = coord.y + 1;
        loop {
            match trees.get(&pt) {
                Some(next) => {
                    if next >= height {
                        break;
                    }
                    pt.y += 1;
                }
                None => {
                    visible += 1;
                    continue 'outer;
                }
            }
        }

        pt.y = coord.y - 1;
        loop {
            match trees.get(&pt) {
                Some(next) => {
                    if next >= height {
                        break;
                    }
                    pt.y -= 1;
                }
                None => {
                    visible += 1;
                    continue 'outer;
                }
            }
        }
    }

    return visible.to_string();
}

fn run_part2(inp: String) -> String {
    let trees = parse_trees(inp);
    let mut tree_scores: HashMap<Point, i32> = HashMap::new();

    let mut pt = Point { x: 0, y: 0 };
    for (coord, height) in trees.iter() {
        let mut score = 1;
        pt.x = coord.x;
        pt.y = coord.y;

        pt.x += 1;
        loop {
            match trees.get(&pt) {
                Some(next) => {
                    if next >= height {
                        score *= pt.x - coord.x;
                        break;
                    }
                    pt.x += 1;
                }
                None => {
                    score *= pt.x - coord.x - 1;
                    break;
                }
            }
        }

        pt.x = coord.x - 1;
        loop {
            match trees.get(&pt) {
                Some(next) => {
                    if next >= height {
                        score *= coord.x - pt.x;
                        break;
                    }
                    pt.x -= 1;
                }
                None => {
                    score *= coord.x - pt.x - 1;
                    break;
                }
            }
        }

        pt.x = coord.x;
        pt.y = coord.y + 1;
        loop {
            match trees.get(&pt) {
                Some(next) => {
                    if next >= height {
                        score *= pt.y - coord.y;
                        break;
                    }
                    pt.y += 1;
                }
                None => {
                    score *= pt.y - coord.y - 1;
                    break;
                }
            }
        }

        pt.y = coord.y - 1;
        loop {
            match trees.get(&pt) {
                Some(next) => {
                    if next >= height {
                        score *= coord.y - pt.y;
                        break;
                    }
                    pt.y -= 1;
                }
                None => {
                    score *= coord.y - pt.y - 1;
                    break;
                }
            }
        }

        tree_scores.insert(
            Point {
                x: coord.x,
                y: coord.y,
            },
            score,
        );
    }

    let best = tree_scores.values().max().unwrap();

    return best.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373\n25512\n65332\n33549\n35390\n";

    #[test]
    fn test_part1() {
        let res = run_part1(INPUT.to_string());
        assert_eq!(res, "21")
    }

    #[test]
    fn test_part2() {
        let res = run_part2(INPUT.to_string());
        assert_eq!(res, "8")
    }
}
