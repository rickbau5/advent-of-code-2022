use std::collections::HashMap;

use crate::input;

pub fn run() -> (String, String) {
    let inp = input::load_input(7);
    // let inp = input::load_test();
    return run_both(inp);
}

#[derive(Debug)]
enum Cmd {
    Cd(String),
    Ls(Vec<Listing>),
}

#[derive(Debug, Clone)]
enum Listing {
    File { size: i32 },
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

fn run_both(inp: String) -> (String, String) {
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

    let processed: HashMap<String, i32> = dir_sizes
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

            (entry.0.clone(), entry.1 + children_size)
        })
        .collect();

    let part1 = processed
        .iter()
        .filter_map(|(.., size)| if *size < 100_000 { Some(size) } else { None })
        .sum::<i32>();

    let disk_size = 70000000;
    let free_req = 30000000;
    let used = processed.get("").expect("should find the root");
    let need = free_req - (disk_size - used);

    let mut candidates: Vec<i32> = processed
        .iter()
        .filter_map(|(.., &size)| if size >= need { Some(size) } else { None })
        .collect();
    candidates.sort();

    return (part1.to_string(), candidates.first().unwrap().to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n";

    #[test]
    fn test_part1() {
        let (part1, part2) = run_both(INPUT.to_string());
        assert_eq!(part1, "95437");
        assert_eq!(part2, "24933642");
    }
}
