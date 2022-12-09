use std::collections::HashMap;

use crate::input;

pub fn run() -> (String, String) {
    let inp = input::load_input(7);
    // let inp = input::load_test();
    return (run_part1(inp.clone()), run_part2(inp.clone()));
}

#[derive(Debug)]
enum Cmd {
    Cd(String),
    Ls(Vec<Listing>),
}

#[derive(Debug, Clone)]
enum Listing {
    File { name: String, size: i32 },
    Dir(String),
}

fn parse_input(inp: String) -> Vec<Cmd> {
    let lines: Vec<&str> = inp.lines().collect();
    let mut commands = Vec::new();
    let mut i: usize = 0;
    while i < lines.len() {
        let line = lines[i];
        let parts: Vec<&str> = line.split(" ").collect();

        let cmd = match parts[1] {
            "cd" => Cmd::Cd(parts[2].to_string()),
            "ls" => Cmd::Ls({
                let mut out_idx = i + 1;
                let mut output = Vec::new();
                while out_idx < lines.len() && lines[out_idx].chars().next() != Some('$') {
                    let listing_parts = lines[out_idx].split(" ").collect::<Vec<&str>>();
                    let listing = match listing_parts[0] {
                        "dir" => Listing::Dir(listing_parts[1].to_string()),
                        _ => Listing::File {
                            name: listing_parts[1].to_string(),
                            size: listing_parts[0]
                                .parse()
                                .expect("for file, this should be the size"),
                        },
                    };

                    output.push(listing);
                    out_idx += 1
                }

                i += 1 + output.len();
                output
            }),
            _ => unreachable!("unhandled case for command: {}", parts[1]),
        };

        match cmd {
            Cmd::Cd(_) => i += 1,
            _ => {}
        }

        commands.push(cmd);
    }

    return commands;
}

fn run_part1(inp: String) -> String {
    let commands = parse_input(inp);
    let mut current_dir_stack: Vec<String> = Vec::new();
    let mut dir_sizes = HashMap::new();

    for command in commands {
        match command {
            Cmd::Cd(dir) => {
                match dir.as_str() {
                    ".." => {
                        current_dir_stack.pop();
                    }
                    "/" => {
                        current_dir_stack.clear();
                        current_dir_stack.push("".to_string());
                    }
                    _ => {
                        current_dir_stack.push(dir);
                    }
                };
            }
            Cmd::Ls(listings) => {
                let total_file_size = listings
                    .iter()
                    .filter_map(|f| match f {
                        Listing::File { size, .. } => Some(*size),
                        _ => None,
                    })
                    .sum::<i32>();

                dir_sizes
                    .entry(current_dir_stack.join("/"))
                    .and_modify(|f| *f += total_file_size)
                    .or_insert(total_file_size);
            }
        };
    }

    let ret: Vec<(&String, i32)> = dir_sizes
        .iter()
        .map(|entry| {
            let children_size = dir_sizes
                .iter()
                .filter_map(|(ent, o_size)| {
                    if ent.starts_with(entry.0) && !ent.eq(entry.0) {
                        Some(o_size)
                    } else {
                        None
                    }
                })
                .sum::<i32>();

            (entry.0, entry.1 + children_size)
        })
        .collect();

    let result = ret
        .iter()
        .filter_map(|(.., size)| if *size < 100_000 { Some(size) } else { None })
        .sum::<i32>();

    return result.to_string();
}

fn run_part2(inp: String) -> String {
    let mut res = 0;

    // res += 1;
    // println!("input:\n{}", inp);

    return res.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n";

    #[test]
    fn test_part1() {
        let res = run_part1(INPUT.to_string());
        assert_eq!(res, "95437")
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let res = run_part2(INPUT.to_string());
        assert_eq!(res, "24933642")
    }
}
