use crate::input;

pub fn run() -> (String, String) {
    let inp = input::load_input(4);
    // let inp = input::load_test();
    return (run_part1(inp.clone()), run_part2(inp.clone()));
}

fn run_part1(inp: String) -> String {
    let mut res = 0;

    for line in inp.lines() {
        let ranges = line.split(",").
            map(|range| {
                let spl = range.split("-").collect::<Vec<&str>>();
                (spl[0].parse::<i32>().unwrap(), spl[1].parse::<i32>().unwrap())
            }).collect::<Vec<(i32, i32)>>();

        let first = ranges[0];
        let second = ranges[1];

        if first.0 <= second.0 && first.1 >= second.1 {
            res += 1
        } else if second.0 <= first.0 && second.1 >= first.1 {
            res += 1
        }
    }

    return res.to_string();
}

fn run_part2(inp: String) -> String {
    let mut res = 0;

    for line in inp.lines() {
        let ranges = line.split(",").
            map(|range| {
                let spl = range.split("-").collect::<Vec<&str>>();
                (spl[0].parse::<i32>().unwrap(), spl[1].parse::<i32>().unwrap())
            }).collect::<Vec<(i32, i32)>>();

        let first = ranges[0];
        let second = ranges[1];

        if first.0 <= second.0 && second.0 <= first.1 {
            res += 1
        } else if first.0 <= second.1 && second.1 <= first.1 {
            res += 1
        } else if second.0 <= first.0 && first.0 <= second.1 {
            res += 1
        } else if second.0 <= first.0 && first.1 <= second.1 {
            res += 1
        } else {
            println!("{} ({:?}) -> no", line, ranges)
        }
    }

    return res.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn test_part1() {
        let res = run_part1(INPUT.to_string());
        assert_eq!(res, "2")
    }

    #[test]
    fn test_part2() {
        let res = run_part2(INPUT.to_string());
        assert_eq!(res, "4")
    }
}
