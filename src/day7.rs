use crate::input;

pub fn run() -> (String, String) {
    let inp = input::load_input(7);
    // let inp = input::load_test();
    return (run_part1(inp.clone()), run_part2(inp.clone()));
}

#[derive(Debug)]
struct Cmd {
    cmd: String,
    arg: Option<String>,
    out: Option<Vec<Listing>>,
}

#[derive(Debug)]
enum ListingType {
    File,
    Dir,
}

#[derive(Debug)]
struct Listing {
    typ: ListingType,
    name: String,
    size: Option<i32>
}

fn parse_input(inp: String) -> Vec<Cmd> {
    let lines: Vec<&str> = inp.lines().collect();
    let mut commands = Vec::new();
    let mut i: usize = 0;
    while i < lines.len() {
        let line = lines[i];
        let parts: Vec<&str> = line.split(" ").collect();
        let cmd_str = parts[1];
        let arg = match cmd_str {
            "cd" => Some(parts[2]),
            _ => None,
        };

        let output_result = match cmd_str {
            "ls" => {
                let mut out_idx = i + 1;
                let mut output = Vec::new();
                while out_idx < lines.len() && lines[out_idx].chars().next() != Some('$') {
                    let listing_parts = lines[out_idx].split(" ").collect::<Vec<&str>>();
                    let listing_type = match listing_parts[0] {
                        "dir" => ListingType::Dir,
                        _ => ListingType::File,
                    };
                    let size: Option<i32> = match listing_type {
                        ListingType::File => Some(listing_parts[0].parse().expect("for file, this should be the size")),
                        _ => None
                    };

                    let listing = Listing{
                        typ: listing_type,
                        name: listing_parts[1].to_string(),
                        size: size,
                    };

                    output.push(listing);
                    out_idx += 1
                }

                Some((output, out_idx - i))
            }
            _ => None,
        };

        let out = match output_result {
            Some((out, output_consumed)) => {
                // this is awkward having side effects here
                i += output_consumed;
                Some(out)
            },
            _ => {
                i += 1;
                None
            }
        };

        let cmd = Cmd{
            cmd: cmd_str.to_string(),
            arg: arg.map(|f| f.to_string()),
            out: out
        };

        commands.push(cmd);
    }

    return commands;
}

fn run_part1(inp: String) -> String {
    let commands = parse_input(inp);

    for command in commands {
        println!("command: {:?}", command);
    }

    return String::from("");
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

    const INPUT: &str = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n";

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
