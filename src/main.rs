use std::{fs, path::Path};

static TEST: bool = false;

fn main() {
    print_solutions(run_day2());
}

fn print_solutions(soln: (String, String)) {
    println!("part 1: {}\npart 2: {}", soln.0, soln.1)
}

fn run_day1() -> (String, String) {
    let input = load_input(1);
    
    let mut vec = vec![0];
    for line in input.lines() {
        if line.len() == 0 {
            vec.push(0);
            continue;
        }
        
        let v: i32 = line.parse().unwrap();
        let current = vec.last_mut().unwrap();
        *current += v;
    }

    vec.sort();
    vec.reverse();
    let part1 = vec.first().unwrap();
    let part2: i32 = vec.iter().take(3).sum();

    return (part1.to_string(), part2.to_string());
}

fn run_day2() -> (String, String) {
    let input = load_input(2);

    let mut total_p1 = 0;
    let mut total_p2 = 0;
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let opponent = parts.get(0).unwrap();
        let me = parts.get(1).unwrap();

        let opponent_move = match *opponent {
            "A" => "rock",
            "B" => "paper",
            "C" => "scissors",
            _=>panic!("unknown value {}", opponent)
        };

        let my_move = match *me {
            "X" => "rock",
            "Y" => "paper",
            "Z" => "scissors",
            _=> panic!("unknown value {}", opponent)
        };

        let calc_score = |o: &str, m: &str| -> i32 {
            let mut score = match m {
                "rock" => 1,
                "paper" => 2,
                "scissors" => 3,
                _=> panic!("unknown value {}", opponent)
            };

            if o == m {
                // draw
                score += 3;
            } else if (m == "rock" && o == "scissors") ||
                (m == "paper" && o == "rock") ||
                (m == "scissors" && o == "paper")
            {
                // win
                score += 6
            } else {
                println!("Lost!")
            }

            score
        };

        let score_p1 = calc_score(opponent_move, my_move);

        // part 2
        let mut score_p2 = 0;
        let my_move_p2 = match *me {
            // lose
            "X" => match opponent_move {
                "rock" => "scissors",
                "paper" => "rock",
                "scissors" => "paper",
                _=> panic!("unknown value {}", opponent)
            },
            // draw
            "Y" => opponent_move,
            // win
            "Z" => match opponent_move {
                "rock" => "paper",
                "paper" => "scissors",
                "scissors" => "rock",
                _=> panic!("unknown value {}", opponent)
            },
            _=> panic!("unknown value {}", opponent)
        };

        let score_p2 = calc_score(opponent_move, my_move_p2);

        println!("opponent: {}, me: {}, score: {}", opponent, me, score_p1);
        total_p1 += score_p1;
        total_p2 += score_p2;
    }

    return (total_p1.to_string(), total_p2.to_string())
}

fn load_input(day: i32) -> String {
    let f = match TEST {
        false => format!("./input/day_{day}.txt"),
        true => "./input/test.txt".to_string(),
    };

    let input = fs::read_to_string(Path::new(&f)).
        expect("Input should exist");

    return input;
}
