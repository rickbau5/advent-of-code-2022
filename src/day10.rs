use std::fmt::{self, Formatter};
use std::num::ParseIntError;

use crate::input;

pub fn run() -> (String, String) {
    let inp = input::load_input(10);
    // let inp = input::load_test();
    return run_both(inp.clone());
}

#[derive(Debug, Clone, Copy)]
enum Cmd {
    Addx(i32),
    Noop,
}

impl Cmd {
    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut spl = s.split(" ");
        let cmd_word = spl.next().expect("need at least one word for command");

        let cmd = match cmd_word {
            "noop" => Cmd::Noop,
            "addx" => Cmd::Addx(spl.next().unwrap().parse()?),
            _ => unreachable!(),
        };

        Ok(cmd)
    }

    fn cost(self) -> i32 {
        match self {
            Cmd::Addx(_) => 2,
            Cmd::Noop => 1,
        }
    }
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            &Cmd::Addx(n) => Ok(write!(f, "addx {}", n))?,
            &Cmd::Noop => Ok(write!(f, "noop"))?,
        }
    }
}

fn fmt_crt_row(r: &[char; 40]) -> String {
    let mut str = String::new();
    for i in 0..r.len() {
        if r[i] == '?' {
            break;
        }
        str.push(r[i])
    }

    return str;
}

fn run_both(inp: String) -> (String, String) {
    let mut commands = inp.lines().map(Cmd::from_str).filter_map(|f| f.ok());

    let mut pc = 1;
    let mut pos = 0;
    let mut reg = 1;
    let mut cmd_cycles = 0;
    let mut cmd = Cmd::Noop;

    let mut strengths = vec![];
    let mut obs_next = 20;

    let mut display: [[char; 40]; 6] = [['?'; 40]; 6];
    let mut crt_row = 0;

    loop {
        // check if cmd is complete
        if cmd_cycles == 0 {
            if let Cmd::Addx(num) = cmd {
                reg += num;
                println!(
                    "End of cycle {}: finish executing {} (Register X is now {})",
                    pc, cmd, reg
                );
            }
            let next_cmd = commands.next();
            match next_cmd {
                Some(c) => {
                    cmd_cycles = c.cost();
                    cmd = c;
                }
                None => break,
            }
        }

        let is_start = match cmd {
            Cmd::Addx(_) => cmd_cycles == 2,
            Cmd::Noop => cmd_cycles == 1,
        };

        if is_start {
            println!("Start cycle {}: begin executing {}", pc, cmd);
        } else {
            println!("During cycle {}: CRT draws pixel in position {}", pc, pos);
        }

        // part 1
        if pc == obs_next {
            strengths.push(pc * reg);
            obs_next += 40;
        }

        // draw
        pos = (pc - 1) % 40;
        if pos == 0 && pc != 1 {
            crt_row += 1;
            if display.len() == crt_row as usize {
                // done
                break;
            }
        }

        if (pos - reg).abs() <= 1 {
            display[crt_row][pos as usize] = '#'
        } else {
            display[crt_row][pos as usize] = '.'
        }
        println!("Current CRT row: {}", fmt_crt_row(&display[crt_row]));

        cmd_cycles -= 1;
        pc += 1;
    }

    // the answer to part two is from the printed output
    for row in display {
        println!("{}", fmt_crt_row(&row))
    }

    (strengths.iter().sum::<i32>().to_string(), "".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop\n";

    #[test]
    fn test_both() {
        let (part1, part2) = run_both(INPUT.to_string());
        assert_eq!(part1, "13140");
        assert_eq!(part2, "")
    }
}
